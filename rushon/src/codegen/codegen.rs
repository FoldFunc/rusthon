#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
}
impl Expr {
    pub fn codegen_into(&self, asm: &mut Vec<String>, spaces: String) {
        match self {
            Expr::Number(n) => {
                asm.push(format!("{}mov rax, {}", spaces, n));
            }
        }
    }
}
impl Stmt {
    pub fn codegen(&self, spaces: String) -> String {
        let mut asm:Vec<String> = Vec::new();
        match self {
            Stmt::Return(expr) => {
                expr.codegen_into(&mut asm, spaces.clone());
                asm.push(format!("{}mov rdi, rax", spaces.clone()).into());
                asm.push(format!("{}mov rax, 60", spaces.clone()).into());
                asm.push(format!("{}syscall", spaces.clone()).into());
            }
        }
        asm.join("\n")
    }
}
