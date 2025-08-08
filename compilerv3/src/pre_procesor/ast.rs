use crate::pre_procesor::stmt::Stmt;
use crate::pre_procesor::lexer::find_var_offset;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Node_Fucntion{
    pub name: String,
    pub stmts: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub struct Ast {
    pub node_funcitons: Vec<Node_Fucntion>,
}
impl Ast {
    pub fn new(node_funcitons: Vec<Node_Fucntion>) -> Self {
        Ast { node_funcitons: node_funcitons}
    }
}
