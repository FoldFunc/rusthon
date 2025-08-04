use crate::pre_procesor::parser::Ast;

pub struct IrGen {
    ast: Ast,
}
impl IrGen {
    pub fn new(ast: Ast) -> Self {
        IrGen { ast: ast }
    }
}
