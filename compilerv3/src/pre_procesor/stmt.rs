use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int_lit {
        val: i32,
    },
    Ident {
        name: String,
    }
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
