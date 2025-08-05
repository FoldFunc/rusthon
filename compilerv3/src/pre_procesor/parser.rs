use crate::pre_procesor::lexer::{Token, Typees};
use std::fmt;
#[derive(Debug, Clone)]
pub enum Stmt {
    Fn {
        name: String,
        body: Vec<Stmt>,
    },
    Ret {
        val: String,
    },
    Var {
        name: String,
        typee: String,
        val: i32,
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
    pub fn codegen_into_fn(&self, stmts: &Vec<Stmt>) -> (Vec<String>, Vec<String>) {
        let mut var_names: Vec<String> = Vec::new();
        let mut asm_lines: Vec<String> = Vec::new();
        let spaces = " ".repeat(4);
        for stmt in stmts {
            match stmt {
                Stmt::Ret { val } => {
                    asm_lines.push(format!("{}xor rdi, rdi", spaces));
                    asm_lines.push(format!("{}xor rax, rax", spaces));
                    if val.parse::<i32>().is_ok() {
                        asm_lines.push(format!("{}mov rdi, {}", spaces, val));
                    } else {
                        asm_lines.push(format!("{}mov rdi, [{}]", spaces, val));
                    }
                    asm_lines.push(format!("{}mov rax, 60", spaces));
                    asm_lines.push(format!("{}syscall", spaces));
                }
                Stmt::Var {
                    name,
                    typee: _,
                    val,
                } => {
                    var_names.push(name.clone());
                    asm_lines.push(format!("{}mov rax, {}", spaces, val));
                    asm_lines.push(format!("{}mov [{}], rax", spaces, name));
                }
                _ => panic!("Invalid in function"),
            }
        }
        return (asm_lines, var_names);
    }
    pub fn codegen_bss(&self, names: &Vec<String>) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("section .bss"));
        for line in names {
            lines.push(format!("{}: resq 1", line));
        }
        return lines;
    }
    pub fn codegen(&self) -> Vec<String> {
        let mut asm_lines: Vec<String> =
            vec!["global _start".to_string(), "section .text".to_string()];
        let mut var_lines: Vec<String> = Vec::new();
        for stmt in &self.stmts {
            match stmt {
                Stmt::Fn { name, body } => {
                    asm_lines.push(format!("{}:", name));
                    let (add_lines, var_names) = self.codegen_into_fn(body);
                    for line in add_lines {
                        asm_lines.push(line);
                    }
                    for name in &var_names {
                        var_lines.push(name.clone());
                    }
                }
                _ => panic!("Not implemented yet."),
            }
        }
        let bss_lines: Vec<String> = self.codegen_bss(&var_lines);
        for line in bss_lines {
            asm_lines.push(line);
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
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
            },
            _ => panic!("Invalid type"),
        };
        self.advance();
        assert!(self.eat(&Token::Assign));
        let val = match self.current() {
            Token::Number(n) => {
                let nret = n.clone();
                self.advance();
                nret
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg
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
            name = "_start".to_string();
        }
        assert!(self.eat(&Token::RightSBracket));
        Stmt::Fn { name, body }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.current() {
            Token::Return => self.parse_return(),
            Token::Func_Decl => self.parse_fn(),
            Token::Var_Decl => self.parse_var_decl(),
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
