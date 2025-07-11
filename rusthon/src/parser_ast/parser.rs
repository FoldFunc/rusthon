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
    // 1. Parse the left‐hand primary expression
    let mut left = self.parse_primary();

    // 2. As long as the next token is a binary op of at least min_prec, consume it
    loop {
        // Try to decode an operator + its precedence/associativity
        let (prec, assoc) = match self.current() {
            Tokens::Plus  => (1, Assoc::Left),
            Tokens::Minus => (1, Assoc::Left),
            Tokens::Star  => (2, Assoc::Left),
            Tokens::Slash => (2, Assoc::Left),
            _ => break,   // not an operator → break the loop
        };

        // If it’s lower‐priority than we need, stop parsing the binary chain
        if prec < min_prec {
            break;
        }

        // Consume the operator token
        let op_token = self.current().clone();
        self.advance();

        // Compute the next min_prec for the recursive call
        let next_min = match assoc {
            Assoc::Left  => prec + 1,
            Assoc::Right => prec,
        };

        // Parse the RHS with that new precedence
        let right = self.parse_expr(next_min);

        // Map the token to your enum
        let op = match op_token {
            Tokens::Plus  => BinaryOp::Plus,
            Tokens::Minus => BinaryOp::Minus,
            Tokens::Star  => BinaryOp::Mul,
            Tokens::Slash => BinaryOp::Div,
            _ => unreachable!(),
        };

        // Combine into a new left node
        left = Expr::Binary {
            left:  Box::new(left),
            op,
            right: Box::new(right),
        };
    }

    // 3. Once no more ops, return what we’ve built
    left
}
pub fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Tokens::Number(n) => {
                let val = n.clone();
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
            Expr::Ident(s) => {
                asm.push(format!("    mov rax, [{s}]"));
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
    pub fn codegen(&self) -> String {
        let mut asm = Vec::new();
        match self {
            Stmt::Return(expr) => {
                expr.codegen_into(&mut asm);
                asm.push(format!("    mov rdi, rax"));
                asm.push("    mov rax, 60".into());
                asm.push("    syscall".into());
            }
            Stmt::VarDecl { name, value } => {
                value.codegen_into(&mut asm);
                asm.push(format!("    ; store var: {} in global memory", name));
                asm.push(format!("    mov [{name}], rax", name = name));
                asm.push(format!("    xor rax, rax"))
            }
        }
        asm.join("\n")
    }
}

pub fn parse(tokens: &Vec<Tokens>) -> Result<Vec<Stmt>, Box<dyn Error>> {
    let mut parser = Parser::new(tokens.to_vec());
    Ok(parser.parse())
}
