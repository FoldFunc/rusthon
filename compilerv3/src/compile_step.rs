use std::{error::Error, fs::OpenOptions};
use std::io::Write;
use crate::pre_procesor::parser::Ast;

pub fn compile(ast: &Ast) -> Result<bool, Box<dyn Error>> {
    let mut asm_lines = Vec::new();
    for stmt in ast.stmts {
        asm_lines.push(stmt.codegen());
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out.asm")?;
    writeln!(file, "global _start")?;
    writeln!(file, "section .text")?;
    writeln!(file, "_start:")?;
    for line in asm_lines {
        writeln!(file, "{}", line)?;
    }
    Ok(true)
}
