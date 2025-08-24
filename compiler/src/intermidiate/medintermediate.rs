use super::intermediate::{IrExpr, IRT};
pub struct MIRStmts {

}
pub struct MIRT {
    mirt: Vec<MIRStmts>,
}
pub struct MIR {
    ir: IRT,
    ifs: Vec<IrExpr>,
}
impl MIR {
    pub fn new(ir: IRT) -> Self {
        MIR { ir, ifs: Vec::new() }
    }
    pub fn simplify() -> 
}
