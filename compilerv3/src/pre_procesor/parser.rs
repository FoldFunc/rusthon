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
impl Ast {
    pub fn new() -> Self {
        Ast { stmts: vec![] }
    }

    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
    pub fn codegen_into_fn(&self, stmts: &Vec<Stmt>) -> (Vec<String>, Vec<(String, i32)>) {
        let mut offset: Vec<(String, i32)> = Vec::new();
        let mut asm_lines: Vec<String> = Vec::new();
        let spaces = " ".repeat(4);
        for stmt in stmts {
            match stmt {
                Stmt::Ret { val } => {
                    let mut offset_curret: i32 = -1;
                    for (var_name, index) in &offset {
                        if val.to_string() == var_name.to_string() {
                            offset_curret = *index;
                        }
                    }
                    if offset_curret == -1 {
                        panic!("Var out of scope");
                    }
                    asm_lines.push(format!("{}xor rdi, rdi", spaces));
                    if val.parse::<i32>().is_ok() {
                        asm_lines.push(format!(
                            "{}mov r13, qword [r12 + {}]",
                            spaces, offset_curret
                        ));
                        asm_lines.push(format!("{}mov rdi, qword [r13]", spaces));
                    } else {
                        asm_lines.push(format!(
                            "{}mov r13, qword [r12 + {}]",
                            spaces, offset_curret
                        ));
                        asm_lines.push(format!("{}mov rdi, qword [r13]", spaces));
                    }
                    asm_lines.push(format!("{}call exit", spaces));
                }
                Stmt::Var {
                    name,
                    typee: _,
                    val,
                } => {
                    let index_last: i32;
                    if let Some((_, index)) = offset.last() {
                        index_last = *index;
                    } else {
                        index_last = -8
                    }
                    offset.push((name.to_string(), index_last + 8));
                    if val.parse::<char>().is_ok() && !val.parse::<i32>().is_ok() {
                        asm_lines.push(format!("{}mov rdi, 8", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!(
                            "{}mov qword [r12 + {}], rax",
                            spaces,
                            index_last + 8
                        ));
                        asm_lines.push(format!("{}mov qword [rax], '{}'", spaces, val));
                    } else {
                        asm_lines.push(format!("{}mov rdi, 8", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!(
                            "{}mov qword [r12 + {}], rax",
                            spaces,
                            index_last + 8
                        ));
                        asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                    }
                    println!("offset: {:?}", offset);
                }
                Stmt::VarQuick { name, val } => {
                    let index_last: i32;
                    if let Some((_, index)) = offset.last() {
                        index_last = *index;
                    } else {
                        index_last = -8
                    }
                    offset.push((name.to_string(), index_last + 8));
                    if val.parse::<char>().is_ok() && !val.parse::<i32>().is_ok() {
                        asm_lines.push(format!("{}mov rdi, 8", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!(
                            "{}mov qword [r12 + {}], rax",
                            spaces,
                            index_last + 8
                        ));
                        asm_lines.push(format!("{}mov qword [rax], '{}'", spaces, val));
                    } else {
                        asm_lines.push(format!("{}mov rdi, 8", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!(
                            "{}mov qword [r12 + {}], rax",
                            spaces,
                            index_last + 8
                        ));
                        asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                    }
                    println!("offset: {:?}", offset);
                }
                Stmt::ReVar {
                    name,
                    typee: _,
                    val,
                } => {
                    let mut index_current: i32 = -1;
                    for (name_in, index) in &offset {
                        if *name_in == *name {
                            index_current = *index;
                        }
                    }
                    if index_current == -1 {
                        panic!("Val not in heap");
                    }
                    println!("current index: {}", index_current);
                    if val.parse::<char>().is_ok() && !val.parse::<i32>().is_ok() {
                        asm_lines.push(format!(
                            "{}mov r13, qword [r12 + {}]",
                            spaces, index_current
                        ));
                        asm_lines.push(format!("{}mov qword [r13], '{}'", spaces, val));
                    } else {
                        asm_lines.push(format!(
                            "{}mov r13, qword [r12 + {}]",
                            spaces, index_current
                        ));
                        asm_lines.push(format!("{}mov qword [r13], {}", spaces, val));
                    }
                    println!("offset: {:?}", offset);
                }
                Stmt::ReVarQuick {
                    name,
                    val,
                } => {
                    let mut index_current: i32 = -1;
                    for (name_in, index) in &offset {
                        if *name_in == *name {
                            index_current = *index;
                        }
                    }
                    if index_current == -1 {
                        panic!("Val not in heap");
                    }
                    println!("current index: {}", index_current);
                    if val.parse::<char>().is_ok() && !val.parse::<i32>().is_ok() {
                        asm_lines.push(format!(
                            "{}mov r13, qword [r12 + {}]",
                            spaces, index_current
                        ));
                        asm_lines.push(format!("{}mov qword [r13], '{}'", spaces, val));
                    } else {
                        asm_lines.push(format!(
                            "{}mov r13, qword [r12 + {}]",
                            spaces, index_current
                        ));
                        asm_lines.push(format!("{}mov qword [r13], {}", spaces, val));
                    }
                    println!("offset: {:?}", offset);
                }
                _ => panic!("Invalid in function"),
            }
        }
        return (asm_lines, offset);
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

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
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
            return Stmt::VarQuick {
                name: name,
                val: val,
            };
        }
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
                Typees::Char => "char".to_string(),
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
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            _ => panic!("Invalid value: {:?}", self.current()),
        };
        assert!(self.eat(&Token::SemiColon));
        return Stmt::Var {
            name: name,
            typee: typee,
            val: val,
        };
    }

    fn parse_binary(&mut self) -> i32 {
        self.parse_expr()
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
            return Stmt::ReVarQuick {
                name: name,
                val: val,
            };
        }
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
                Typees::Char => "char".to_string(),
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
        return Stmt::ReVar { name: name, typee: typee, val: val };
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
