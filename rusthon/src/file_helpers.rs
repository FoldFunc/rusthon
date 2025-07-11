use crate::parser_ast::parser::Stmt;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
pub fn take_command_line_args() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("Invalid amount of command line arguments".into());
    }
    Ok(args[1].clone())
}
pub fn check_valid_path(path_string: String) -> Result<bool, Box<dyn Error>> {
    let path = Path::new(&path_string);
    if path.exists() {
        return Ok(true);
    }
    Err("Invalid path to the file.".into())
}
pub fn give_file_content(path_to_file: String) -> Result<String, Box<dyn Error>> {
    match fs::read_to_string(path_to_file) {
        Ok(content) => {
            return Ok(content);
        }
        Err(e) => Err(format!("Error acured while reading the file: {}", e).into()),
    }
}
pub fn gen_begging() -> bool {
    let asm_start = format!("global _start\nsection .text\n_start:\n    sub rsp, 64\n");
    match fs::write("out.asm", asm_start) {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
pub fn gen_asm(ast: &Vec<Stmt>) -> Result<bool, Box<dyn Error>> {
    // collect all var names
    let mut globals = Vec::new();

    // generate .text
    let mut asm_lines = Vec::new();
    for stmt in ast {
        if let Stmt::VarDecl { name, .. } = stmt {
            globals.push(name.clone());
        }
        asm_lines.push(stmt.codegen());
    }

    // write prologue + code
    let mut file = OpenOptions::new().write(true).create(true).open("out.asm")?;
    writeln!(file, "global _start")?;
    writeln!(file, "section .text")?;
    writeln!(file, "_start:")?;
    writeln!(file, "    ; … your prologue here (e.g. sub rsp…)")?;
    for line in asm_lines {
        writeln!(file, "{}", line)?;
    }

    // emit .bss globals
    if !globals.is_empty() {
        writeln!(file, "\nsection .bss")?;
        for name in globals {
            // reserve 8 bytes for each 64‑bit var
            writeln!(file, "{}:    resq 1", name)?;
        }
    }

    Ok(true)
}

