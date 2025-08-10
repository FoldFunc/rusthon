use std::{error::Error, fs::OpenOptions};
use std::io::Write;
use crate::post_processor::air::Air;
use crate::pre_procesor::to_asm::code_generator;
pub fn compile(air: &Air) -> Result<bool, Box<dyn Error>> {
    let mut codegen = code_generator::new(&air);
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
