mod tokenizer;
mod parser;
mod generator;
mod intermidiate;
use std::{env, error::Error, fs::{self, File}};
use std::io::Write;
fn make_new_file(lines: Vec<String>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("out.asm")?;
    writeln!(file, "global main")?;
    writeln!(file, "section .text")?;
    writeln!(file, "main:")?;
    for line in lines {
        writeln!(file, "    {}", line)?;
    }
    Ok(())
}
fn get_file_content(file_name: String) -> Result<String, Box<dyn Error>> {
    let file = fs::read_to_string(file_name)?;
    Ok(file)
}
fn get_file_name() -> Result<String, Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Bad usage: ./compiler <filepath>");
    }
    Ok(args[1].clone())
}
fn main() -> Result<(), Box<dyn Error>>{
    let file_name = get_file_name()?;
    let file_content = get_file_content(file_name)?;
    let mut token_izer = tokenizer::tokenizer::Tokenizer::new(file_content);
    let tokens: Vec<tokenizer::tokenizer::Tokens> = token_izer.tokenize()?;
    println!("tokens: \n{:#?}", tokens);
    let mut parser = parser::parser::Parser::new(tokens);
    let ast = parser.parse()?;
    println!("ast: \n{:#?}", ast);
    let mut ir = intermidiate::intermediate::Intermediate::new(ast);
    let irt = ir.simplify()?;
    println!("irt: \n{:#?}", irt);
    let mut file_gen = generator::generator::Generator::new(irt);
    let file_new = file_gen.generate()?;
    make_new_file(file_new)?;
    Ok(())
}
