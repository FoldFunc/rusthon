use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Term(Term),
    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Mul,
    Div,
    Equals,
    MoreThan,
    LessThan,
}
impl Op {
    pub fn precedence(&self) -> u8 {
        match self {
            Op::Plus | Op::Minus => 1,
            Op::Mul | Op::Div => 2,
            Op::Equals | Op::MoreThan | Op::LessThan => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Int_lit {
        val: i32,
    },
    Ident {
        name: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Return {
        expr: Expr,
    },
    Var {
        name: String,
        expr: Expr,
    },
}
