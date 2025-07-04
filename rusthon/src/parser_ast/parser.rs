use crate::lexer::lexer::Tokens;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
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
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Tokens {
        self.tokens.get(self.position).unwrap_or(&Tokens::EOF)
    }
    fn get_all(&self) -> Vec<Tokens> {
        self.tokens.clone()
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
            println!("self.get_all: {:?}", self.get_all());
            println!("self.current: {:?}", self.current());

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

    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Tokens::Number(n) => {
                let val = *n;
                self.advance();
                Expr::Number(val)
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
            Expr::Number(n) => {
                asm.push(format!("    mov rax, {}", n));
            }
            Expr::Binary { left, op, right } => {
                left.codegen_into(asm);
                asm.push("    push rax".into());

                right.codegen_into(asm);
                asm.push("    pop rbx".into());

                match op {
                    BinaryOp::Plus => {
                        asm.push("    add rax, rbx".into());
                    }
                    BinaryOp::Minus => {
                        asm.push("    mov rcx, rax".into());
                        asm.push("    mov rax, rbx".into());
                        asm.push("    sub rax, rcx".into());
                    }
                    BinaryOp::Mul => {
                        // multiplication expects rax = left, rbx = right
                        // currently rax=right, rbx=left, so swap:
                        asm.push("    xchg rax, rbx".into()); // swap rax and rbx
                        asm.push("    imul rax, rbx".into());
                    }
                    BinaryOp::Div => {
                        // division expects rax = dividend, rbx = divisor
                        // currently rax=right, rbx=left, so swap:
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
    pub fn codegen(&self, stack_position: i32) -> String {
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
                asm.push(format!("    ; store var {} on stack ", name));
                asm.push(format!("    mov [rsp - {}], rax", stack_position));
            }
        }
        asm.join("\n")
    }
}

pub fn parse(tokens: &Vec<Tokens>) -> Result<Vec<Stmt>, Box<dyn Error>> {
    let mut parser = Parser::new(tokens.to_vec());
    Ok(parser.parse())
}
