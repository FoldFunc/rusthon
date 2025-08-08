use std::collections::HashMap;
use lexer::lexer::Tokens;
use codegen::codegen::Stmt;
use codegen::codegen::Expr;
mod helpers;
mod parser;
mod lexer;
mod codegen;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file_name = helpers::command_line_args()?;
    let _ = helpers::valid_path(&file_name);
    let file_contents = helpers::file_context(&file_name);
    let tokens: Vec<Tokens> = lexer::lexer::tokenize(&file_contents.unwrap());
    Ok(())
}
