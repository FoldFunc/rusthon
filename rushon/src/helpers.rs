use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::error::Error;
use crate::codegen::codegen::Stmt;
pub fn command_line_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("Invalid amount of command line arguments".into());
    }
    Ok(args[1].clone())
}
pub fn valid_path(path: &String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    if path.exists() {
        return Ok(());
    } else {
        return Err("Invalid path to file.".into());
    }
}
pub fn file_context(path: &String) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(path)
        .map_err(|e| format!("Error ocured when looking at file: {}", e).into())
}
pub fn compile(ast: &Vec<Stmt>) -> Result<bool, Box<dyn Error>> {
    let mut asm_lines = Vec::new();
    for stmt in ast {
        asm_lines.push(stmt.codegen("    ".to_string()));
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
