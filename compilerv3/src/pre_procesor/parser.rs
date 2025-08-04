use crate::pre_procesor::lexer::Token;
use std::{fmt, ops::Index};
#[derive(Debug, Clone)]
pub enum Stmt {
    Fn { name: String, body: Vec<Stmt> },
    Ret { val: i32 },
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
    pub fn codegen_into_fn(&self, stmts: &Vec<Stmt>) -> Vec<String> {
        let mut asm_lines: Vec<String> = Vec::new();
        let spaces = " ".repeat(4);
        for stmt in stmts {
            match stmt {
                Stmt::Ret { val } => {
                    asm_lines.push(format!("{}xor rdi, rdi", spaces));
                    asm_lines.push(format!("{}xor rax, rax", spaces));
                    asm_lines.push(format!("{}mov rdi, {}", spaces, val));
                    asm_lines.push(format!("{}mov rax, 60", spaces));
                    asm_lines.push(format!("{}syscall", spaces));
                }
                _ => panic!("Invalid in function"),
            }
        }
        return asm_lines;
    }
    pub fn codegen(&self) -> Vec<String> {
        let mut asm_lines: Vec<String> =
            vec!["global _start".to_string(), "section .text".to_string()];
        for stmt in &self.stmts {
            match stmt {
                Stmt::Fn { name, body } => {
                    asm_lines.push(format!("{}:", name));
                    let add_lines = self.codegen_into_fn(body);
                    for line in add_lines {
                        asm_lines.push(line);
                    }
                }
                Stmt::Ret { val } => {
                    asm_lines.push(format!("xor rdi, rdi"));
                    asm_lines.push(format!("xor rax, rax"));
                    asm_lines.push(format!("mov rdi, {}", val));
                    asm_lines.push(format!("mov rax, 60"));
                    asm_lines.push(format!("syscall"));
                }
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
                v
            }
            _ => panic!("Expected number after return"),
        };
        assert!(self.eat(&Token::SemiColon));
        assert!(val <= 256 && val >= 0);
        Stmt::Ret { val }
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
