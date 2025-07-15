use crate::lexer::lexer::Tokens;
use std::collections::HashMap;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref LISTS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

static LIST_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Char(char),
    List(Vec<Expr>),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Ident(String),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
    VarDecl { name: String, value: Expr },
    VarRedecl { name: String, value: Expr },
}

pub struct Parser {
    tokens: Vec<Tokens>,
    position: usize,
}

enum Assoc {
    Left,
    Right,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current(&self) -> &Tokens {
        self.tokens.get(self.position).unwrap_or(&Tokens::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn eat(&mut self, expected: &Tokens) -> bool {
        if self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while self.current() != &Tokens::EOF {
            let stmt = match self.current() {
                Tokens::Return => {
                    self.advance();
                    let expr = self.parse_expr(0);
                    assert!(self.eat(&Tokens::SemiColon));
                    Stmt::Return(expr)
                }
                Tokens::Var => {
                    self.advance();
                    let name = match self.current() {
                        Tokens::Ident(ident) => {
                            let ident = ident.clone();
                            self.advance();
                            ident
                        }
                        _ => panic!("Expected a var name after the var keyword"),
                    };
                    assert!(self.eat(&Tokens::Eq));
                    let value = self.parse_expr(0);
                    assert!(self.eat(&Tokens::SemiColon));
                    Stmt::VarDecl { name, value }
                }
                Tokens::Ident(s) => {
                    let name = s.clone();
                    self.advance();
                    assert!(self.eat(&Tokens::Eq));
                    let value = self.parse_expr(0);
                    assert!(self.eat(&Tokens::SemiColon));
                    Stmt::VarRedecl { name, value }
                }
                _ => panic!("Expected statement, found {:?}", self.current()),
            };

            stmts.push(stmt);
        }

        stmts
    }

    pub fn parse_expr(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_primary();

        loop {
            let (prec, assoc) = match self.current() {
                Tokens::Plus => (1, Assoc::Left),
                Tokens::Minus => (1, Assoc::Left),
                Tokens::Star => (2, Assoc::Left),
                Tokens::Slash => (2, Assoc::Left),
                _ => break,
            };

            if prec < min_prec {
                break;
            }

            let op_token = self.current().clone();
            self.advance();

            let next_min = match assoc {
                Assoc::Left => prec + 1,
                Assoc::Right => prec,
            };

            let right = self.parse_expr(next_min);

            let op = match op_token {
                Tokens::Plus => BinaryOp::Plus,
                Tokens::Minus => BinaryOp::Minus,
                Tokens::Star => BinaryOp::Mul,
                Tokens::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }

    pub fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Tokens::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if self.current() != &Tokens::RBracket {
                    loop {
                        let expr = self.parse_expr(0);
                        elements.push(expr);
                        if self.eat(&Tokens::Comma) {
                            continue;
                        } else {
                            break;
                        }
                    }
                }
                assert!(self.eat(&Tokens::RBracket));
                Expr::List(elements)
            }
            Tokens::Char(c) => {
                let ctoken = *c;
                self.advance();
                Expr::Char(ctoken)
            }
            Tokens::Number(n) => {
                let val = *n;
                self.advance();
                Expr::Number(val)
            }
            Tokens::Ident(s) => {
                let val = s.clone();
                self.advance();
                Expr::Ident(val)
            }
            Tokens::LParen => {
                self.advance();
                let expr = self.parse_expr(0);
                assert!(self.eat(&Tokens::RParen));
                expr
            }
            _ => panic!("Unexpected token in primary: {:?}", self.current()),
        }
    }
}

impl Expr {
    pub fn codegen_into(&self, asm: &mut Vec<String>) {
        match self {
            Expr::List(elements) => {
                let id = LIST_ID.fetch_add(1, Ordering::SeqCst);
                let label = format!("vec{}", id);

                for (i, elem) in elements.iter().enumerate() {
                    elem.codegen_into(asm); // puts result in rax
                    asm.push(format!("    mov [{}_addr + 8*{}], rax", label, i));
                }

                asm.push(format!("    lea rax, [{}_addr]", label));
                LISTS.lock().unwrap().insert(label, elements.len());
            }
            Expr::Char(c) => {
                asm.push(format!("    mov byte rax, '{}'", c));
            }
            Expr::Ident(s) => {
                asm.push(format!("    mov rax, [{}]", s));
            }
            Expr::Number(n) => {
                asm.push(format!("    mov rax, {}", n));
            }
            Expr::Binary { left, op, right } => {
                left.codegen_into(asm);
                asm.push("    push rax".into());
                right.codegen_into(asm);
                asm.push("    pop rbx".into());

                match op {
                    BinaryOp::Plus => asm.push("    add rax, rbx".into()),
                    BinaryOp::Minus => {
                        asm.push("    mov rcx, rax".into());
                        asm.push("    mov rax, rbx".into());
                        asm.push("    sub rax, rcx".into());
                    }
                    BinaryOp::Mul => {
                        asm.push("    xchg rax, rbx".into());
                        asm.push("    imul rax, rbx".into());
                    }
                    BinaryOp::Div => {
                        asm.push("    xchg rax, rbx".into());
                        asm.push("    mov rdx, 0".into());
                        asm.push("    div rbx".into());
                    }
                }
            }
        }
    }

    pub fn codegen(&self) -> String {
        let mut asm = Vec::new();
        self.codegen_into(&mut asm);
        asm.join("\n")
    }
}

impl Stmt {
    pub fn codegen(&self) -> String {
        let mut asm = Vec::new();
        match self {
            Stmt::Return(expr) => {
                expr.codegen_into(&mut asm);
                asm.push("    mov rdi, rax".into());
                asm.push("    mov rax, 60".into());
                asm.push("    syscall".into());
            }
            Stmt::VarDecl { name, value } => {
                value.codegen_into(&mut asm);
                asm.push(format!("    mov [{}], rax", name));
            }
            Stmt::VarRedecl { name, value } => {
                value.codegen_into(&mut asm);
                asm.push(format!("    mov [{}], rax", name));
            }
        }
        asm.join("\n")
    }
}

pub fn parse(tokens: &Vec<Tokens>) -> Result<Vec<Stmt>, Box<dyn Error>> {
    let mut parser = Parser::new(tokens.to_vec());
    Ok(parser.parse())
}

