#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Var{name: String, val: Expr},
}
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Ident(String),
}
impl Expr {
    pub fn codegen_into(&self, asm: &mut Vec<String>, spaces: String) {
        match self {
            Expr::Number(n) => {
                asm.push(format!("{}mov rax, {}", spaces, n));
            }
            Expr::Ident(s) => {
                asm.push(format!("{}mov rax, [{}]", spaces, s));
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
            Stmt::Var { name, val } => {
                val.codegen_into(&mut asm, spaces.clone());
                asm.push(format!("{}mov [{}], rax", spaces, name));

            }
        }
        asm.join("\n")
    }
}
