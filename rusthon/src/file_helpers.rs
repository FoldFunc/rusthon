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
    let mut stack_pos = 8;
    let mut asm_gen = Vec::new();
    for element in ast {
        println!("element: {:?}", element);
        let stmt = element.codegen(stack_pos);
        asm_gen.push(stmt);
        asm_gen.join("\n");
        if let Stmt::VarDecl { .. } = element {
            stack_pos += 8;
        }
    }
    let mut file = OpenOptions::new().append(true).open("out.asm")?;
    writeln!(file, "{}", asm_gen.join("\n"))?;
    Ok(true)
}
