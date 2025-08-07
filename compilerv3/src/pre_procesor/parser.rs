use crate::pre_procesor::lexer::{Token, Typees};
use std::{env::var, fmt, mem::offset_of};
#[derive(Debug, Clone)]
pub enum Stmt {
    Fn {
        name: String,
        body: Vec<Stmt>,
    },
    ReVar {
        name: String,
        typee: String,
        val: String,
    },
    ReVarQuick {
        name: String,
        val: String,
    },
    Ret {
        val: String,
    },
    Var {
        name: String,
        typee: String,
        val: String,
    },
    VarQuick {
        name: String,
        val: String,
    },
}
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
impl Stmt {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, ident: usize) -> fmt::Result {
        let pad = "  ".repeat(ident);
        match self {
            Stmt::Fn { name, body } => {
                writeln!(f, "{}Fn: {}", pad, name)?;
                for stmt in body {
                    stmt.fmt_with_indent(f, ident + 1)?;
                }
            }
            Stmt::Ret { val } => {
                writeln!(f, "{}Ret: {}", pad, val)?;
            }
            Stmt::Var { name, typee, val } => {
                writeln!(
                    f,
                    "{}Var:\n{}{}name:{}\n{}{}type:{}\n{}{}val:{}",
                    pad, pad, pad, name, pad, pad, typee, pad, pad, val
                )?;
            }
            Stmt::VarQuick { name, val } => {
                writeln!(
                    f,
                    "{}Var:\n{}{}name:{}\n{}{}val:{}",
                    pad, pad, pad, name, pad, pad, val
                )?;
            }
            Stmt::ReVarQuick { name, val } => {
                writeln!(
                    f,
                    "{}Re assign quick:\n{}{}name:{}\n{}{}val:{}",
                    pad, pad, pad, name, pad, pad, val
                )?;
            }
            Stmt::ReVar { name, typee, val } => {
                writeln!(
                    f,
                    "{}Re assign:\n{}{}name:{}\n{}{}type: {}{}{}val:{}",
                    pad, pad, pad, name, pad, pad, typee, pad, pad, val
                )?;
            }
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}
impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmtt in &self.stmts {
            writeln!(f, "{}", stmtt)?;
        }
        Ok(())
    }
}
pub fn find_var_offset(offset: &[(String, i32)], name: &str) -> i32 {
    for (var_name, off) in offset {
        if var_name == name {
            return *off;
        }
    }
    panic!("Variable '{}' out of scope", name);
}
impl Ast {
    pub fn new() -> Self {
        Ast { stmts: vec![] }
    }

    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
    pub fn codegen_into_fn(&self, stmts: &Vec<Stmt>) -> (Vec<String>, Vec<(String, i32)>) {
        let mut offset: Vec<(String, i32)> = Vec::new(); // var name -> offset relative to r12
        let mut asm_lines: Vec<String> = Vec::new();
        let spaces = " ".repeat(4);

        for stmt in stmts {
            match stmt {
                Stmt::Ret { val } => {
                    // Check if val is immediate int
                    if let Ok(immediate) = val.parse::<i32>() {
                        asm_lines.push(format!("{}mov rdi, {}", spaces, immediate));
                    } else if val.len() == 1 && !val.parse::<i32>().is_ok() {
                        // Single char literal e.g. 'a'
                        let c = val.chars().next().unwrap();
                        asm_lines.push(format!("{}mov rdi, {}", spaces, c as u8));
                    } else {
                        // Variable: load pointer from [r12 + offset], then load value
                        let var_offset = find_var_offset(&offset, val);
                        asm_lines.push(format!("{}mov r13, qword [r12 + {}]", spaces, var_offset));
                        asm_lines.push(format!("{}mov rdi, qword [r13]", spaces));
                    }
                    asm_lines.push(format!("{}call exit", spaces));
                }

                Stmt::Var { name, typee, val } => {
                    // Determine next offset
                    let last_offset = offset.last().map(|(_, off)| *off).unwrap_or(-8);
                    let new_offset = last_offset + 8;
                    offset.push((name.to_string(), new_offset));

                    match typee.as_str() {
                        "int32" => {
                            asm_lines.push(format!("{}mov rdi, 8", spaces));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                        }
                        "char" => {
                            asm_lines.push(format!("{}mov rdi, 1", spaces)); // allocate 1 byte for char
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            let c = val.chars().next().expect("Char value expected");
                            asm_lines.push(format!("{}mov byte [rax], {}", spaces, c as u8));
                        }
                        "string" => {
                            let len = val.len() + 1; // +1 for null terminator
                            asm_lines.push(format!("{}mov rdi, {}", spaces, len));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            for (i, c) in val.chars().enumerate() {
                                asm_lines
                                    .push(format!("{}mov byte [rax + {}], {}", spaces, i, c as u8));
                            }
                            asm_lines.push(format!("{}mov byte [rax + {}], 0", spaces, val.len())); // null terminator
                        }
                        "boolean" => {
                            asm_lines.push(format!("{}mov rdi, 1", spaces));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                        }
                        "list<Int32>" => {
                            let list_val: Vec<i32> = val
                                .split_whitespace()
                                .map(|s| s.parse::<i32>().expect("Invalid input"))
                                .collect();
                            let len = list_val.len() * 8;
                            asm_lines.push(format!("{}mov rdi, {}", spaces, len));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            let mut offset_local = 0;
                            for c in list_val {
                                asm_lines.push(format!(
                                    "{}mov qword [rax + {}], {}",
                                    spaces, offset_local, c
                                ));
                                offset_local += 8;
                            }
                        }
                        _ => panic!("Unsupported type '{}'", typee),
                    }
                }

                Stmt::VarQuick { name, val } => {
                    // Quick let without type, guess from val
                    let last_offset = offset.last().map(|(_, off)| *off).unwrap_or(-8);
                    let new_offset = last_offset + 8;
                    offset.push((name.to_string(), new_offset));

                    if val.len() == 1 && !val.parse::<i32>().is_ok() {
                        // Single char
                        asm_lines.push(format!("{}mov rdi, 1", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                        let c = val.chars().next().unwrap();
                        asm_lines.push(format!("{}mov byte [rax], {}", spaces, c as u8));
                    } else {
                        // Assume int32
                        asm_lines.push(format!("{}mov rdi, 8", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                        asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                    }
                }

                Stmt::ReVar { name, val, .. } | Stmt::ReVarQuick { name, val } => {
                    let var_offset = find_var_offset(&offset, name);
                    if val.len() == 1 && !val.parse::<i32>().is_ok() {
                        // Char update
                        asm_lines.push(format!("{}mov r13, qword [r12 + {}]", spaces, var_offset));
                        let c = val.chars().next().unwrap();
                        asm_lines.push(format!("{}mov byte [r13], {}", spaces, c as u8));
                    } else {
                        // Int update
                        asm_lines.push(format!("{}mov r13, qword [r12 + {}]", spaces, var_offset));
                        asm_lines.push(format!("{}mov qword [r13], {}", spaces, val));
                    }
                }

                _ => panic!("Unsupported statement in function"),
            }
        }

        (asm_lines, offset)
    }
    pub fn codegen(&self) -> Vec<String> {
        let mut asm_lines: Vec<String> = vec![
            "extern exit".to_string(),
            "extern malloc".to_string(),
            "global main".to_string(),
            "section .text".to_string(),
        ];
        for stmt in &self.stmts {
            match stmt {
                Stmt::Fn { name, body } => {
                    asm_lines.push(format!("{}:", name));
                    asm_lines.push(format!("{}mov rdi, 1024", " ".repeat(4)));
                    asm_lines.push(format!("{}call malloc", " ".repeat(4)));
                    asm_lines.push(format!("{}mov r12, rax", " ".repeat(4)));
                    let (add_lines, offset) = self.codegen_into_fn(body);
                    for line in add_lines {
                        asm_lines.push(line);
                    }
                }
                _ => panic!("Not implemented yet."),
            }
        }
        for line in &asm_lines {
            println!("line of asm: {}", line);
        }
        return asm_lines;
    }
}
#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    vars: Vec<(String, String)>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
            vars: Vec::new(),
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn eat(&mut self, expected: &Token) -> bool {
        if self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance(); // consume `return`
        let val = match self.current() {
            Token::Number(n) => {
                let v = *n;
                self.advance();
                v.to_string()
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            Token::Ident(s) => {
                let v = s.clone();
                self.advance();
                v
            }
            _ => panic!("Expected number after return"),
        };
        assert!(self.eat(&Token::SemiColon));
        if val.parse::<i32>().is_ok() {
            assert!(val.parse::<i32>().unwrap() <= 256 && val.parse::<i32>().unwrap() >= 0);
        }
        Stmt::Ret { val }
    }
    fn parse_var_decl(&mut self) -> Stmt {
        self.advance();
        let name = match self.current() {
            Token::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => panic!("Invalid var name"),
        };
        if self.eat(&Token::AssignQuick) {
            let val = match self.current() {
                Token::Ident(s) => {
                    let mut rval = " ".to_string();
                    for (name, val) in &self.vars {
                        if &s == &name {
                            rval = val.to_string();
                        } else {
                            continue;
                        }
                    }
                    if rval == " " {
                        panic!("Invlaid var name");
                    }
                    self.advance();
                    rval.to_string()
                }
                Token::Number(n) => {
                    let nret = n.clone();
                    self.advance();
                    nret.to_string()
                }
                Token::Char(c) => {
                    let ccar = c.clone();
                    self.advance();
                    ccar.to_string()
                }
                Token::LeftParent => {
                    let rawdawg: i32 = self.parse_binary();
                    rawdawg.to_string()
                }
                _ => panic!("Invalid value: {:?}", self.current()),
            };
            assert!(self.eat(&Token::SemiColon));
            self.vars.push((name.clone(), val.clone()));
            return Stmt::VarQuick {
                name: name,
                val: val,
            };
        }
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
                Typees::Char => "char".to_string(),
                Typees::Stringg => "string".to_string(),
                Typees::Boolean => "boolean".to_string(),
                Typees::List(n) => format!("list<{:?}>", n),
            },
            _ => panic!("Invalid type"),
        };
        self.advance();
        assert!(self.eat(&Token::Assign));
        let val = match self.current() {
            Token::Number(n) => {
                let nret = n.clone();
                self.advance();
                nret.to_string()
            }
            Token::Char(c) => {
                let ccor = c.clone();
                self.advance();
                ccor.to_string()
            }
            Token::Stringg(s) => {
                let sret = s.clone();
                self.advance();
                sret.to_string()
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            Token::List(l) => {
                let mut lret: String = String::new();

                let mut i = 0;
                while i < l.len() {
                    let token = &l[i];
                    match token {
                        Token::Number(n) => {
                            lret.push_str(&n.to_string());
                            lret.push(' ');
                            i += 1;
                        }
                        Token::Char(c) => {
                            lret.push_str(&c.to_string());
                            lret.push(' ');
                            i += 1;
                        }
                        Token::Stringg(s) => {
                            lret.push_str(&s);
                            lret.push(' ');
                            i += 1;
                        }
                        Token::LeftParent => {
                            // Parse the expression from the list slice starting at i
                            let mut subparser = Parser {
                                tokens: l[i..].to_vec(), // Clone from current list index
                                position: 0,
                                vars: self.vars.clone(),
                            };
                            let value = subparser.parse_binary();
                            lret.push_str(&value.to_string());
                            lret.push(' ');

                            // Skip over the tokens that were parsed
                            let consumed = subparser.position;
                            i += consumed;
                        }
                        _ => panic!("Invalid type inside list: {:?}", token),
                    }
                }

                self.advance(); // consume the List token
                lret
            }
            _ => panic!("Invalid value: {:?}", self.current()),
        };
        println!("self.eat val: {:?}", self.current());
        assert!(self.eat(&Token::SemiColon));
        self.vars.push((name.clone(), val.clone()));
        return Stmt::Var {
            name: name,
            typee: typee,
            val: val,
        };
    }
    pub fn double_is(&mut self) -> bool {
        let mut tmp = self.clone();
        let mut is = false;
        while tmp.current() != &Token::RightParent {
            match tmp.current() {
                Token::DoubleIs => is = true,
                _ => is = is,
            }
            tmp.advance();
        }
        println!("is double?: {}", is);
        return is;
    }
    pub fn parse_binary_good(&mut self) -> i32 {
        let mut if_ret = 1; // 1 - false, 0 - true 
        let mut first = String::new();
        let mut second = String::new();
        let mut change = false;
        self.advance();
        while self.current() != &Token::RightParent {
            match self.current() {
                Token::Number(n) if change == false => first.push_str(&n.to_string()),
                Token::Number(n) => second.push_str(&n.to_string()),
                Token::Ident(s) if change == false => first.push_str(s),
                Token::Ident(s) => second.push_str(s),
                Token::DoubleIs => change = true,
                t => panic!("invalid in if: t: {:?}", t),
            }
            self.advance();
        }
        if !first.parse::<i32>().is_ok() {
            for (name, val) in &self.vars {
                if first == name.to_string() {
                    first = val.to_string();
                } else {
                    continue;
                }
            }
        }
        if !second.parse::<i32>().is_ok() {
            for (name, val) in &self.vars {
                if second == name.to_string() {
                    second = val.to_string();
                } else {
                    continue;
                }
            }
        }
        if first.parse::<i32>() == second.parse::<i32>() {
            if_ret = 0;
        } else {
            if_ret = 1;
        }
        self.advance();
        return if_ret;
    }
    fn parse_binary(&mut self) -> i32 {
        println!("self.current parse binary: {:?}", self.current());
        if self.double_is() {
            self.parse_binary_good()
        } else {
            self.parse_expr()
        }
    }

    fn parse_expr(&mut self) -> i32 {
        let mut left = self.parse_term();
        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_term();
            match op {
                Token::Plus => left += right,
                Token::Minus => left -= right,
                _ => unreachable!(),
            }
        }
        left
    }

    fn parse_term(&mut self) -> i32 {
        let mut left = self.parse_factor();
        while matches!(self.current(), Token::Mul | Token::Div) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_factor();
            match op {
                Token::Mul => left *= right,
                Token::Div => left /= right,
                _ => unreachable!(),
            }
        }
        left
    }

    fn parse_factor(&mut self) -> i32 {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                n
            }
            Token::Ident(s) => {
                self.advance();
                let mut curr_val: i32 = -1;
                for (name, val) in self.vars.to_vec() {
                    if name == s {
                        curr_val = val.parse::<i32>().unwrap();
                    }
                }
                if curr_val == -1 {
                    panic!("No such variable");
                }
                curr_val
            }
            Token::LeftParent => {
                self.advance();
                let val = self.parse_expr();
                if !self.eat(&Token::RightParent) {
                    panic!("Expected `)`");
                }
                val
            }
            _ => panic!("Unexpected token in factor: {:?}", self.current()),
        }
    }

    fn parse_fn(&mut self) -> Stmt {
        self.advance(); // consume `fn`
        let mut name = match self.current() {
            Token::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => panic!("Expected function name"),
        };
        assert!(self.eat(&Token::LeftParent));
        assert!(self.eat(&Token::RightParent));
        assert!(self.eat(&Token::LeftSBracket));

        let mut body = vec![];
        while self.current() != &Token::RightSBracket {
            body.push(self.parse_stmt());
        }
        if name == "main" {
            name = "main".to_string();
        }
        assert!(self.eat(&Token::RightSBracket));
        Stmt::Fn { name, body }
    }
    fn parse_var_re_decl(&mut self) -> Stmt {
        self.advance();
        let name = match self.current() {
            Token::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => panic!("Invalid var name"),
        };
        if self.eat(&Token::AssignQuick) {
            let val = match self.current() {
                Token::Number(n) => {
                    let nret = n.clone();
                    self.advance();
                    nret.to_string()
                }
                Token::Char(c) => {
                    let ccar = c.clone();
                    self.advance();
                    ccar.to_string()
                }
                Token::LeftParent => {
                    let rawdawg: i32 = self.parse_binary();
                    rawdawg.to_string()
                }
                _ => panic!("Invalid value: {:?}", self.current()),
            };
            assert!(self.eat(&Token::SemiColon));
            self.vars.push((name.clone(), val.clone()));
            return Stmt::ReVarQuick {
                name: name,
                val: val,
            };
        }
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
                Typees::Char => "char".to_string(),
                Typees::Stringg => "string".to_string(),
                Typees::Boolean => "boolean".to_string(),
                Typees::List(n) => format!("list<{:?}>", n),
            },
            _ => panic!("Invalid type"),
        };
        self.advance();
        assert!(self.eat(&Token::Assign));
        let val = match self.current() {
            Token::Number(n) => {
                let nret = n.clone();
                self.advance();
                nret.to_string()
            }
            Token::Char(c) => {
                let ccar = c.clone();
                self.advance();
                ccar.to_string()
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            _ => panic!("Invalid value: {:?}", self.current()),
        };
        assert!(self.eat(&Token::SemiColon));
        self.vars.push((name.clone(), val.clone()));
        return Stmt::ReVar {
            name: name,
            typee: typee,
            val: val,
        };
    }
    fn parse_stmt(&mut self) -> Stmt {
        match self.current() {
            Token::Return => self.parse_return(),
            Token::Func_Decl => self.parse_fn(),
            Token::Var_Decl => self.parse_var_decl(),
            Token::Var_Update => self.parse_var_re_decl(),
            _ => panic!("Unexpected token: {:?}", self.current()),
        }
    }
    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast::new();
        while self.current() != &Token::EOF {
            let stmt = self.parse_stmt();
            ast.push(stmt);
        }
        ast
    }
}
