use std::{error::Error, fs::OpenOptions};
use std::io::Write;
use crate::pre_procesor::ast::Ast;
use crate::pre_procesor::to_asm::code_generator;
pub fn compile(ast: &Ast) -> Result<bool, Box<dyn Error>> {
    let mut codegen = code_generator::new(&ast);
    let asm_lines = codegen.codegen();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out.asm")?;
    for line in asm_lines {
        writeln!(file, "{}", line)?;
    }
    Ok(true)
}
