use crate::parser_ast::parser::{Stmt, LISTS};
use crate::parser_ast::parser::Expr;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
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
        Ok(true)
    } else {
        Err("Invalid path to the file.".into())
    }
}

pub fn give_file_content(path_to_file: String) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(path_to_file)
        .map_err(|e| format!("Error occurred while reading the file: {}", e).into())
}

pub fn gen_asm(ast: &Vec<Stmt>) -> Result<bool, Box<dyn Error>> {
    let mut globals: HashMap<String, Expr> = HashMap::new();
    let mut asm_lines = Vec::new();

    // Collect global declarations
    for stmt in ast {
        if let Stmt::VarDecl { name, value } = stmt {
            globals.insert(name.clone(), value.clone());
        }
        asm_lines.push(stmt.codegen());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out.asm")?;

    // Emit .text section
    writeln!(file, "global _start")?;
    writeln!(file, "section .text")?;
    writeln!(file, "_start:")?;
    writeln!(file, "    sub rsp, 64")?;

    for line in asm_lines {
        writeln!(file, "{}", line)?;
    }

    // Emit .bss section
    if !globals.is_empty() || !LISTS.lock().unwrap().is_empty() {
        writeln!(file, "\nsection .bss")?;

        for (name, expr) in &globals {
            match expr {
                Expr::Number(_) | Expr::Char(_) | Expr::Ident(_) => {
                    writeln!(file, "{}:    resq 1", name)?;
                }
                Expr::List(elements) => {
                    let label = format!("{}0_addr", name);
                    writeln!(file, "{}:    resq 1", name)?; // pointer to list base
                    writeln!(file, "{}:    resq {}", label, elements.len())?; // backing array
                }
                _ => {}
            }
        }

        // Emit any lists not declared as vars (just to be safe)
        for (label, count) in LISTS.lock().unwrap().iter() {
            writeln!(file, "{}_addr: resq {}", label, count)?;
        }
    }

    Ok(true)
}

