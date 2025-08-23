use crate::{
    intermidiate::intermediate::{IRT, IrExpr, IrStmts},
    parser::parser::{Expr, Stmts, Term},
};
use std::{env::VarError, error::Error};

#[derive(Debug, Clone)]
pub struct Var {
    pub stack_pos: u32,
    pub value: i32,
    pub name: String,
}

pub struct Generator {
    asm: Vec<String>,
    ast: IRT,
    vars: Vec<Var>,
    current_stack_bytes: u32,
}

impl Generator {
    pub fn new(ast: IRT) -> Self {
        Generator {
            asm: Vec::new(),
            ast,
            vars: Vec::new(),
            current_stack_bytes: 0,
        }
    }

    pub fn read(&mut self, reg: &str, var: &Var) -> Result<(), Box<dyn Error>> {
        self.asm
            .push(format!("mov {}, [rbp - {}]", reg, var.stack_pos));
        Ok(())
    }

    pub fn alloc_from_reg(
        &mut self,
        reg: &str,
        value: i32,
        name: &String,
    ) -> Result<Var, Box<dyn Error>> {
        self.current_stack_bytes += 8;
        let var = Var {
            stack_pos: self.current_stack_bytes,
            value,
            name: name.clone(),
        };
        self.vars.push(var.clone());
        self.asm.push(format!("sub rsp, 8"));
        self.asm
            .push(format!("mov [rbp - {}], {}", var.stack_pos, reg));

        Ok(var)
    }
    pub fn re_decl(&mut self, value: i32, name: &String) -> Result<Var, Box<dyn Error>> {
        if let Some(var) = self.vars.iter_mut().find(|v| v.name == *name) {
            var.value = value;
            self.asm.push(format!("mov [rbp - {}], rax", var.stack_pos));
            Ok(var.clone())
        } else {
            Err("Error while changing the value in re_decl"
                .to_string()
                .into())
        }
    }
    pub fn get_value_by_name(&mut self, var_name: &String) -> Result<i32, Box<dyn Error>> {
        if let Some(var) = self.vars.iter().find(|v| v.name == *var_name) {
            return Ok(var.value);
        }
        Err(format!("Invalid variable name: {}", var_name).into())
    }
    pub fn get_position_by_name(&mut self, var_name: &String) -> Result<u32, Box<dyn Error>> {
        if let Some(var) = self.vars.iter().find(|v| v.name == *var_name) {
            return Ok(var.stack_pos);
        }
        Err(format!("Invalid variable name: {}", var_name).into())
    }
    pub fn generate_expr_re_decl(
        &mut self,
        expr: &IrExpr,
        name: Option<&String>,
    ) -> Result<Var, Box<dyn Error>> {
        match expr {
            IrExpr::IntLit(n) => {
                let name_correct = name.unwrap();
                self.asm.push(format!("mov rax, {}", n));
                let var = self.re_decl(*n, name_correct)?;
                Ok(var)
            }
            IrExpr::Variable(var) => {
                let name_correct = name.unwrap();
                let var_value = self.get_position_by_name(var)?;
                self.asm.push(format!("mov rax, [rbp - {}]", var_value));
                let n = self.get_value_by_name(var)?;
                let var = self.re_decl(n, name_correct)?;
                Ok(var)
            }
            IrExpr::Boolean(state) => {
                let name_correct = name.unwrap();
                self.asm.push(format!("mov rax, {}", *state as i32));
                let var = self.re_decl(*state as i32, name_correct)?;
                Ok(var)
            }
        }
    }
    pub fn generate_expr(
        &mut self,
        expr: &IrExpr,
        name: Option<&String>,
    ) -> Result<Var, Box<dyn Error>> {
        match expr {
            IrExpr::IntLit(n) => {
                let normal = "returnvalue".to_string();
                let name_corrent = name.unwrap_or(&normal);
                self.asm.push(format!("mov rax, {}", n));
                let var = self.alloc_from_reg("rax", *n, name_corrent)?;
                Ok(var)
            }
            IrExpr::Variable(var) => {
                let normal = "returnvlaue".to_string();
                let name_correct = name.unwrap_or(&normal);
                let var_value = self.get_position_by_name(var)?;
                self.asm.push(format!("mov rax, [rbp - {}]", var_value));
                let n = self.get_value_by_name(var)?;
                let var = self.alloc_from_reg("rax", n, name_correct)?;
                Ok(var)
            }
            IrExpr::Boolean(state) => {
                let normal = "returnvalue".to_string();
                let name_corrent = name.unwrap_or(&normal);
                self.asm.push(format!("mov rax, {}", *state as i32));
                let var = self.alloc_from_reg("rax", *state as i32, name_corrent)?;
                Ok(var)
            }
            some => Err(format!("Invalid in generate_expr: {:?}", some).into()),
        }
    }
    pub fn generate_into_scope(
        &mut self,
        stmt: &IrStmts,
        vars_out: Vec<Var>,
        vars_in: Vec<Var>,
    ) -> Result<(), Box<dyn Error>> {
        match stmt {
            IrStmts::Return(expr) => {
                let result_var = self.generate_expr(expr, None)?;
                self.read("rdi", &result_var)?;
                self.asm.push("mov rax, 60".to_string());
                self.asm.push("syscall".to_string());
                Ok(())
            }
            IrStmts::Var { name, val } => {
                let _v = self.generate_expr(val, Some(name))?;
                Ok(())
            }
            IrStmts::VarRe { name, val } => {
                let _v = self.generate_expr_re_decl(val, Some(name))?;
                Ok(())
            }
            some => Err(format!("Invalid in generate_stmt: {:?}", some).into()),
        }
    }

    fn pop_scope(&mut self, saved_vars_len: usize, saved_stack_bytes: u32) {
        if self.current_stack_bytes > saved_stack_bytes {
            let bytes_to_free = self.current_stack_bytes - saved_stack_bytes;
            self.asm.push(format!("add rsp, {}", bytes_to_free));
            self.current_stack_bytes = saved_stack_bytes;
        }
        self.vars.truncate(saved_vars_len);
    }

    pub fn generate_stmt(&mut self, stmt: &IrStmts) -> Result<(), Box<dyn Error>> {
        match stmt {
            IrStmts::Scope { body } => {
                let saved_vars_len = self.vars.len();
                let saved_stack_bytes = self.current_stack_bytes;

                for irstmt in body.iter() {
                    self.generate_stmt(irstmt)?;
                }

                // unwind: remove inner-scope variables and free stack space
                self.pop_scope(saved_vars_len, saved_stack_bytes);
                Ok(())
            }

            IrStmts::Return(expr) => {
                let result_var = self.generate_expr(expr, None)?;
                self.read("rdi", &result_var)?;
                self.asm.push("mov rax, 60".to_string());
                self.asm.push("syscall".to_string());
                Ok(())
            }
            IrStmts::Var { name, val } => {
                let _v = self.generate_expr(val, Some(name))?;
                Ok(())
            }
            IrStmts::VarRe { name, val } => {
                let _v = self.generate_expr_re_decl(val, Some(name))?;
                Ok(())
            }
            some => Err(format!("Invalid in generate_stmt: {:?}", some).into()),
        }
    }
    pub fn generate(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        self.asm.push("push rbp".to_string());
        self.asm.push("mov rbp, rsp".to_string());

        let stmts = self.ast.irstmts.clone();
        for stmt in &stmts {
            self.generate_stmt(stmt)?;
        }
        Ok(self.asm.clone())
    }
}
