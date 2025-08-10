#[derive(Debug, Clone)]
pub enum IRExpr {
    Int(i32),
    Var(String),
}
#[derive(Debug, Clone)]
pub enum IRStmt {
    Let { name: String, expr: IRExpr },
    Assign { name: String, expr: IRExpr},
    Return(IRExpr),
    Scope { stmts: Vec<IRStmt>},
}
#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub stmts: Vec<IRStmt>,
}
#[derive(Debug, Clone)]
pub struct Air {
    pub ir: Vec<IRFunction>,
}
impl Air {
    pub fn new() -> Self {
        Air { ir: Vec::new() }
    }
}
