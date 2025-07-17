#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Var{name: String, typee: Type, val: Expr},
}
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Ident(String),
    Char(char),
    List(Vec<Expr>),
}
#[derive(Debug, Clone)]
pub enum Type{
    Char,
    Int32,
    List,
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
            Expr::Char(c) => {
                asm.push(format!("{}mov rax, \'{}\'", spaces, c));
            }
            Expr::List(items) => {
                for (i, item) in items.iter().enumerate() {
                    item.codegen_into(asm, spaces.clone()); // puts value in rax
                    asm.push(format!("{}mov [{}_{}], rax", spaces, "__list", i)); // or use real variable name + offset
                }
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
            Stmt::Var { name, typee: _, val } => {
                match val {
                    Expr::List(items) => {
                        for (i, item) in items.iter().enumerate() {
                            item.codegen_into(&mut asm, spaces.clone()); // loads item into rax
                            asm.push(format!("{}mov [{} + {}], rax", spaces, name, i * 8)); // 8-byte slots
                        }
                    }
                    _ => {
                        val.codegen_into(&mut asm, spaces.clone());
                        asm.push(format!("{}mov [{}], rax", spaces, name));
                    }
                }
            }
        }
        asm.join("\n")
    }
}
