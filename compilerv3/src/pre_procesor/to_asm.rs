use crate::pre_procesor::{
    ast::{Ast, Node_Fucntion},
    stmt::{Expr, Stmt},
};
use std::collections::HashMap;
pub struct code_generator {
    asm: Vec<String>,
    ast: Ast,
    stack_pos: i32,
    variables: HashMap<String, i32>, // maps variable names to their stack position
}
impl code_generator {
    pub fn new(ast: &Ast) -> Self {
        code_generator {
            asm: Vec::new(),
            ast: ast.clone(),
            stack_pos: 0,
            variables: HashMap::new(),
        }
    }
    pub fn codegen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Int_lit { val } => {
                self.asm.push(format!("    mov rax, {}", val));
                self.asm.push("    push rax".to_string());
                self.stack_pos += 1;
            }
            Expr::Ident { name } => {
                if let Some(offset) = self.variables.get(name) {
                    self.asm.push(format!(
                        "    mov rax, [rsp + {}]",
                        (self.stack_pos - 1 - offset) * 8
                    ));
                } else {
                    panic!("Undefined variable: {}", name);
                }
            }
        }
    }
    pub fn codegen_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Return { expr } => {
                self.codegen_expr(&expr);
                self.asm
                    .push(format!("    mov rdi, rax"));
                self.asm.push("    mov rax, 60".to_string()); // syscall number for exit
                self.asm.push("    syscall".to_string());
            }
            Stmt::Var { name, expr } => {
                self.codegen_expr(&expr);
                self.variables.insert(name, self.stack_pos - 1); // top of stack after push
            }
        }
    }
    pub fn codegen_fn(&mut self, function: &Node_Fucntion) {
        self.asm.push(format!("{}:", function.name));
        for stmt in function.stmts.clone() {
            self.codegen_stmt(stmt);
        }
    }
    pub fn codegen(&mut self) -> Vec<String> {
        for function in self.ast.node_funcitons.clone() {
            self.asm.push(format!("global {}", function.name));
        }
        for function in self.ast.node_funcitons.clone() {
            self.codegen_fn(&function);
        }
        return self.asm.clone();
    }
}
