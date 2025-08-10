use crate::{post_processor::air::{Air, IRExpr, IRFunction, IRStmt}, pre_procesor::{
    ast::{Ast, Node_Fucntion},
    stmt::{Expr, Stmt, Term},
}};
use std::collections::HashMap;
pub struct Var {
    stack_loc: i32,
}
pub struct code_generator {
    asm: Vec<String>,
    ast: Air,
    stack_size: i32,
    variables: HashMap<String, Var>, // maps variable names to their stack position
}
impl code_generator {
    pub fn new(ast: &Air) -> Self {
        code_generator {
            asm: Vec::new(),
            ast: ast.clone(),
            stack_size: 0,
            variables: HashMap::new(),
        }
    }
    pub fn push(&mut self, reg: &str) {
        self.asm.push(format!("    push {}", reg));
        self.stack_size +=1;
    }
    pub fn pop(&mut self, reg: &str) {
        self.asm.push(format!("    pop {}", reg));
        self.stack_size -=1;
    }
    pub fn codegen_expr(&mut self, expr: &IRExpr) {
        match expr {
            IRExpr::Int(n) => {
                self.asm.push(format!("    mov rax, {}", n));
                self.push("rax");
            }
            IRExpr::Var(v) => {
                if !self.variables.contains_key(v) {
                    panic!("no such variable: {}", v);
                }
                let offset = self.variables.get(v).unwrap();
                self.asm.push(format!("    push QWORD [rsp + {}]", (self.stack_size - offset.stack_loc - 1) * 8));
            }
            other => panic!("Not supported for now: {:?}", other),
        }
    }
    pub fn codegen_stmt(&mut self, stmt: IRStmt) {
        match stmt {
            IRStmt::Return(expr) => {
                self.codegen_expr(&expr);
                self.pop("rdi");
                self.asm.push("    mov rax, 60".to_string()); // syscall number for exit
                self.asm.push("    syscall".to_string());
            }
            IRStmt::Let { name, expr } => {
                if self.variables.contains_key(&name) {
                    panic!("Variable already assigned");
                }
                self.variables.insert(name, Var { stack_loc: self.stack_size });
                self.codegen_expr(&expr);
            }
            other => panic!("Not supported for now: {:?}", other),
        }
    }
    pub fn codegen_fn(&mut self, function: &IRFunction) {
        self.asm.push(format!("{}:", function.name));
        for stmt in function.stmts.clone() {
            self.codegen_stmt(stmt);
        }
    }
    pub fn codegen(&mut self) -> Vec<String> {
        for function in self.ast.ir.clone() {
            self.asm.push(format!("global {}", function.name));
        }
        for function in self.ast.ir.clone() {
            self.codegen_fn(&function);
        }
        return self.asm.clone();
    }
}
