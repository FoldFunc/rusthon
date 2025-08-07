use crate::pre_procesor::stmt::Stmt;
use crate::pre_procesor::lexer::find_var_offset;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}
impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmtt in &self.stmts {
            writeln!(f, "{}", stmtt)?;
        }
        Ok(())
    }
}
impl Ast {
    pub fn new() -> Self {
        Ast { stmts: vec![] }
    }
    pub fn remove(&mut self, stmt: &Stmt) {
        let mut position: i32 = -1;
        for i in &self.stmts {
            if i == stmt {
                position += 1;
            } else {
                continue;
            }
        }
        if position == -1 {
            panic!("No such stmt");
        }
        self.stmts.remove(position as usize);
    }
    pub fn push(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }
    pub fn len(&self) -> i32 {
        self.stmts.len() as i32
    }
    pub fn last(&self) -> Stmt {
        let ret = self.stmts.get(self.stmts.len()).unwrap();
        return ret.clone();
    }
    pub fn codegen_into_fn(&self, stmts: &Vec<Stmt>) -> (Vec<String>, Vec<(String, i32)>) {
        let mut offset: Vec<(String, i32)> = Vec::new(); // var name -> offset relative to r12
        let mut asm_lines: Vec<String> = Vec::new();
        let spaces = " ".repeat(4);

        for stmt in stmts {
            match stmt {
                Stmt::Ret { val } => {
                    // Check if val is immediate int
                    if let Ok(immediate) = val.parse::<i32>() {
                        asm_lines.push(format!("{}mov rdi, {}", spaces, immediate));
                    } else if val.len() == 1 && !val.parse::<i32>().is_ok() {
                        let var_offset = find_var_offset(&offset, val);
                        asm_lines.push(format!("{}mov r13, qword [r12 + {}]", spaces, var_offset));
                        asm_lines.push(format!("{}mov rdi, qword [r13]", spaces));
                    } else {
                        let c = val.chars().next().unwrap();
                        asm_lines.push(format!("{}mov rdi, {}", spaces, c as u8));
                    }
                    asm_lines.push(format!("{}call exit", spaces));
                }

                Stmt::Var { name, typee, val } => {
                    // Determine next offset
                    let last_offset = offset.last().map(|(_, off)| *off).unwrap_or(-8);
                    let new_offset = last_offset + 8;
                    offset.push((name.to_string(), new_offset));

                    match typee.as_str() {
                        "int32" => {
                            asm_lines.push(format!("{}mov rdi, 8", spaces));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                        }
                        "char" => {
                            asm_lines.push(format!("{}mov rdi, 1", spaces)); // allocate 1 byte for char
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            let c = val.chars().next().expect("Char value expected");
                            asm_lines.push(format!("{}mov byte [rax], {}", spaces, c as u8));
                        }
                        "string" => {
                            let len = val.len() + 1; // +1 for null terminator
                            asm_lines.push(format!("{}mov rdi, {}", spaces, len));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            for (i, c) in val.chars().enumerate() {
                                asm_lines
                                    .push(format!("{}mov byte [rax + {}], {}", spaces, i, c as u8));
                            }
                            asm_lines.push(format!("{}mov byte [rax + {}], 0", spaces, val.len())); // null terminator
                        }
                        "boolean" => {
                            asm_lines.push(format!("{}mov rdi, 1", spaces));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                        }
                        "list<Int32>" => {
                            let list_val: Vec<i32> = val
                                .split_whitespace()
                                .map(|s| s.parse::<i32>().expect("Invalid input"))
                                .collect();
                            let len = list_val.len() * 8;
                            asm_lines.push(format!("{}mov rdi, {}", spaces, len));
                            asm_lines.push(format!("{}call malloc", spaces));
                            asm_lines
                                .push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                            let mut offset_local = 0;
                            for c in list_val {
                                asm_lines.push(format!(
                                    "{}mov qword [rax + {}], {}",
                                    spaces, offset_local, c
                                ));
                                offset_local += 8;
                            }
                        }
                        _ => panic!("Unsupported type '{}'", typee),
                    }
                }

                Stmt::VarQuick { name, val } => {
                    // Quick let without type, guess from val
                    let last_offset = offset.last().map(|(_, off)| *off).unwrap_or(-8);
                    let new_offset = last_offset + 8;
                    offset.push((name.to_string(), new_offset));

                    if val.len() == 1 && !val.parse::<i32>().is_ok() {
                        // Single char
                        asm_lines.push(format!("{}mov rdi, 1", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                        let c = val.chars().next().unwrap();
                        asm_lines.push(format!("{}mov byte [rax], {}", spaces, c as u8));
                    } else {
                        // Assume int32
                        asm_lines.push(format!("{}mov rdi, 8", spaces));
                        asm_lines.push(format!("{}call malloc", spaces));
                        asm_lines.push(format!("{}mov qword [r12 + {}], rax", spaces, new_offset));
                        asm_lines.push(format!("{}mov qword [rax], {}", spaces, val));
                    }
                }

                Stmt::ReVar { name, val, .. } | Stmt::ReVarQuick { name, val } => {
                    let var_offset = find_var_offset(&offset, name);
                    if val.len() == 1 && !val.parse::<i32>().is_ok() {
                        // Char update
                        asm_lines.push(format!("{}mov r13, qword [r12 + {}]", spaces, var_offset));
                        let c = val.chars().next().unwrap();
                        asm_lines.push(format!("{}mov byte [r13], {}", spaces, c as u8));
                    } else {
                        // Int update
                        asm_lines.push(format!("{}mov r13, qword [r12 + {}]", spaces, var_offset));
                        asm_lines.push(format!("{}mov qword [r13], {}", spaces, val));
                    }
                }
                Stmt::Condition { body } => {
                    asm_lines.push(format!("{}jmp {}", spaces, "cond"));
                    asm_lines.push(format!("{}:", "cond"));
                    let (add_lines, _offset) = self.codegen_into_fn(body);
                    for line in add_lines {
                        asm_lines.push(line);
                    }
                }
                _ => panic!("Unsupported statement in function"),
            }
        }

        (asm_lines, offset)
    }
    pub fn codegen(&self) -> Vec<String> {
        let mut asm_lines: Vec<String> = vec![
            "extern exit".to_string(),
            "extern malloc".to_string(),
            "global main".to_string(),
            "section .text".to_string(),
        ];
        for stmt in &self.stmts {
            match stmt {
                Stmt::Fn { name, body } => {
                    asm_lines.push(format!("{}:", name));
                    asm_lines.push(format!("{}mov rdi, 1024", " ".repeat(4)));
                    asm_lines.push(format!("{}call malloc", " ".repeat(4)));
                    asm_lines.push(format!("{}mov r12, rax", " ".repeat(4)));
                    let (add_lines, _offset) = self.codegen_into_fn(body);
                    for line in add_lines {
                        asm_lines.push(line);
                    }
                }

                _ => panic!("Not implemented yet."),
            }
        }
        for line in &asm_lines {
            println!("line of asm: {}", line);
        }
        return asm_lines;
    }
}
