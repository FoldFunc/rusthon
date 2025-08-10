use crate::post_processor::air::{Air, IRExpr, IRFunction, IRStmt};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Var {
    stack_loc: i32, // stack offset (0 = top of stack)
}

pub struct CodeGenerator {
    asm: Vec<String>,
    ast: Air,
    stack_size: i32,           // number of values on stack
    variables: HashMap<String, Var>, // maps variable names to stack offset
}

impl CodeGenerator {
    pub fn new(ast: &Air) -> Self {
        CodeGenerator {
            asm: Vec::new(),
            ast: ast.clone(),
            stack_size: 0,
            variables: HashMap::new(),
        }
    }

    pub fn push(&mut self, reg: &str) {
        self.asm.push(format!("    push {}", reg));
        self.stack_size += 1;
    }

    pub fn pop(&mut self, reg: &str) {
        self.asm.push(format!("    pop {}", reg));
        self.stack_size -= 1;
    }

    pub fn load_var(&mut self, name: &str) {
        let var = self.variables.get(name).expect("variable not found");
        let offset = (self.stack_size - var.stack_loc - 1) * 8;
        self.asm.push(format!("    mov rax, [rsp + {}]", offset));
        self.push("rax");
    }

    pub fn store_var(&mut self, name: &str) {
        let var = self.variables.get(name).expect("variable not found");
        let offset = (self.stack_size - var.stack_loc - 1) * 8;
        self.pop("rax");
        self.asm.push(format!("    mov [rsp + {}], rax", offset));
    }

    pub fn codegen_expr(&mut self, expr: &IRExpr) {
        match expr {
            IRExpr::Int(n) => {
                self.asm.push(format!("    mov rax, {}", n));
                self.push("rax");
            }
            IRExpr::Var(name) => {
                self.load_var(name);
            }
            other => panic!("Not supported for now: {:?}", other),
        }
    }

    pub fn codegen_stmt(&mut self, stmt: IRStmt) {
        match stmt {
            IRStmt::Return(expr) => {
                self.codegen_expr(&expr);
                self.pop("rdi"); // return value in rdi
                self.asm.push("    mov rax, 60".to_string()); // exit syscall
                self.asm.push("    syscall".to_string());
            }
            IRStmt::Scope { stmts } => {
                // Save current stack info
                let saved_stack_size = self.stack_size;
                let saved_variables = self.variables.clone();

                for stmt in stmts {
                    self.codegen_stmt(stmt);
                }

                // Pop variables introduced in this scope
                while self.stack_size > saved_stack_size {
                    self.pop("rax");
                }

                // Restore variables to outer scope (remove shadowed vars)
                self.variables = saved_variables;
            }
            IRStmt::Let { name, expr } => {
                if self.variables.contains_key(&name) {
                    // Variable already exists — treat as assignment
                    self.codegen_expr(&expr);
                    self.store_var(&name);
                } else {
                    // New variable — push expr on stack and record offset
                    self.codegen_expr(&expr);
                    self.variables.insert(
                        name,
                        Var {
                            stack_loc: self.stack_size,
                        },
                    );
                }
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
        for function in &self.ast.ir {
            self.asm.push(format!("global {}", function.name));
        }
        let ir = self.ast.ir.clone();
        for function in ir {
            self.codegen_fn(&function);
        }
        self.asm.clone()
    }
}

