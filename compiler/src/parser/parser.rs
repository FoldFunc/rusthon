use std::error::Error;

use crate::tokenizer::tokenizer::Tokens;

#[derive(Debug, Clone)]
pub enum Expr {
    Term(Term),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    IsEq,
    MoreThan,
    LessThan,
    MoreEqThan,
    LessEqThan,
    Modulo,
}
impl Op {
    pub fn precedence(&self) -> Result<u8, Box<dyn Error>> {
        match self {
            Op::IsEq | Op::LessEqThan | Op::LessThan | Op::MoreEqThan | Op::MoreThan | Op::Modulo => Ok(0),
            Op::Plus | Op::Minus => Ok(1),
            Op::Mul | Op::Div => Ok(2),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Term {
    IntLit { val: i32 },
    Boolean{ state: bool},
    VarName{ name: String },
}
#[derive(Debug, Clone)]
pub enum Stmts {
    Scope { body: Box<Vec<Stmts>> },
    Return { val: Expr },
    Var { name: String, val: Expr },
    VarRe { name: String, val: Expr },
    If {condition: Expr, body: Box<Stmts> },
    Elif {condition: Expr, body: Box<Stmts> },
    Else {body: Box<Stmts> },
}

#[derive(Debug, Clone)]
pub struct AST {
    pub stmts: Vec<Stmts>,
}

impl AST {
    pub fn new() -> Self {
        AST { stmts: Vec::new() }
    }
}

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Tokens>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Peek token at current position + x.
    /// If the requested index is past the end, return Tokens::EOF instead of an error.
    pub fn peek(&self, x: usize) -> Result<Tokens, Box<dyn Error>> {
        if self.pos + x >= self.tokens.len() {
            // Treat out-of-bounds as EOF so parser can check for end-of-input.
            return Ok(Tokens::EOF);
        }
        Ok(self.tokens[self.pos + x].clone())
    }

    /// Advance the position by x, but clamp at tokens.len() (do not error).
    pub fn advance(&mut self, x: usize) -> Result<(), Box<dyn Error>> {
        self.pos = std::cmp::min(self.pos + x, self.tokens.len());
        Ok(())
    }

    pub fn eat(&mut self, expected: Tokens) -> Result<bool, Box<dyn Error>> {
        if self.peek(0)? == expected {
            self.advance(1)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn match_operators(&self) -> Result<Option<Op>, Box<dyn Error>> {
        match self.peek(0)? {
            Tokens::Plus => Ok(Some(Op::Plus)),
            Tokens::Minus => Ok(Some(Op::Minus)),
            Tokens::Star => Ok(Some(Op::Mul)),
            Tokens::Slash => Ok(Some(Op::Div)),
            Tokens::IsEq => Ok(Some(Op::IsEq)),
            Tokens::LessThan=> Ok(Some(Op::LessThan)),
            Tokens::LessEqThan=> Ok(Some(Op::LessEqThan)),
            Tokens::MoreThan=> Ok(Some(Op::MoreThan)),
            Tokens::MoreEqThan=> Ok(Some(Op::MoreEqThan)),
            Tokens::Modulo => Ok(Some(Op::Modulo)),
            _ => Ok(None),
        }
    }
    pub fn parse_term(&mut self) -> Result<Term, Box<dyn Error>> {
        match self.peek(0)? {
            Tokens::Number(n) => {
                self.advance(1)?;
                Ok(Term::IntLit { val: n })
            }
            Tokens::Ident(s) => {
                self.advance(1)?;
                Ok(Term::VarName { name: s })
            }
            Tokens::True => {
                self.advance(1)?;
                Ok(Term::Boolean { state: true })
            }
            Tokens::False => {
                self.advance(1)?;
                Ok(Term::Boolean { state: false})
            }
            some => Err(format!("Invalid in parse term: {:?}", some).into()),
        }
    }
    pub fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, Box<dyn Error>> {
        let mut left = Expr::Term(self.parse_term()?);
        loop {
            let op = match self.match_operators()? {
                Some(op) => op,
                None => break,
            };

            let prec = op.precedence()?;
            if prec < min_prec {
                break;
            }

            self.advance(1)?; // consume operator

            // left-associative, so use `prec + 1` only if you had right-associative ops.
            let right = self.parse_binary_expr(prec + 1)?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
    pub fn parse_expr(&mut self) -> Result<Expr, Box<dyn Error>> {
        self.parse_binary_expr(0)
    }
    pub fn parse_var_name(&mut self) -> Result<String, Box<dyn Error>> {
        match self.peek(0)? {
            Tokens::Ident(s) => Ok(s),
            some => Err(format!("Invalid in parse var name: {:?}", some).into()),
        }
    }
    pub fn parse_stmt(&mut self) -> Result<Stmts, Box<dyn Error>> {
        match self.peek(0)? {
            Tokens::LeftCurly => {
                let mut stmts: Vec<Stmts> = Vec::new();
                self.advance(1)?;
                loop {
                    if self.peek(0)? == Tokens::RightCurly {
                        self.advance(1)?;
                        break;
                    } else {
                        stmts.push(self .parse_stmt()?);
                    }
                }
                Ok(Stmts::Scope { body: Box::new(stmts) })
            }
            Tokens::If => {
                self.advance(1)?;
                let condition = self.parse_expr()?;
                let body = self.parse_stmt()?;
                match body {
                    Stmts::Scope { body: _body_scope } => Ok(Stmts::If { condition, body: Box::new(Stmts::Scope { body: _body_scope }) }),
                    some => Err(format!("Invlalid after if: {:?}", some).into()),
                }
            }
            Tokens::Elif => {
                self.advance(1)?;
                let condition = self.parse_expr()?;
                let body = self.parse_stmt()?;
                match body {
                    Stmts::Scope { body: _body_scope } => Ok(Stmts::Elif { condition, body: Box::new(Stmts::Scope { body: _body_scope }) }),
                    some => Err(format!("Invlalid after elif: {:?}", some).into()),
                }
            }
            Tokens::Else => {
                self.advance(1)?;
                let body = self.parse_stmt()?;
                match body {
                    Stmts::Scope { body: _body_scope } => Ok(Stmts::Else { body: Box::new(Stmts::Scope { body: _body_scope }) }),
                    some => Err(format!("Invlalid after else: {:?}", some).into()),
                }
            }
            Tokens::Return => {
                self.advance(1)?;
                let expr = self.parse_expr()?;
                assert!(self.eat(Tokens::Semi)?);
                Ok(Stmts::Return { val: expr })
            }
            Tokens::Ident(n) => {
                self.advance(1)?;
                assert!(self.eat(Tokens::Eq)?);
                let expr = self.parse_expr()?;
                assert!(self.eat(Tokens::Semi)?);
                Ok(Stmts::VarRe { name: n, val: expr })
            }
            Tokens::VarDecl => {
                self.advance(1)?;
                let name = self.parse_var_name()?;
                self.advance(1)?;
                assert!(self.eat(Tokens::Eq)?);
                let expr = self.parse_expr()?;
                assert!(self.eat(Tokens::Semi)?);
                Ok(Stmts::Var { name, val: expr })
            }
            some => Err(format!("Invalid in parse statement: {:?}", some).into()),
        }
    }

    pub fn parse(&mut self) -> Result<AST, Box<dyn Error>> {
        let mut ast = AST::new();
        loop {
            if self.peek(0)? == Tokens::EOF {
                break;
            }
            ast.stmts.push(self.parse_stmt()?);
        }
        Ok(ast)
    }
}
