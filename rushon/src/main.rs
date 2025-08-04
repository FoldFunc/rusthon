use std::collections::HashMap;
use lexer::lexer::Tokens;
use codegen::codegen::Stmt;
use codegen::codegen::Expr;
mod helpers;
mod parser;
mod lexer;
mod codegen;
fn print_lex_tokens(tokens: &Vec<Tokens>) {
    for token in tokens {
        println!("Lex token: {:?}", token);
    }
}
fn print_parse_tokens(tokens: &Vec<Stmt>) {
    for token in tokens {
        println!("Parser token: {:?}", token);
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file_name = helpers::command_line_args()?;
    println!("file name: {:?}", &file_name);
    let _ = helpers::valid_path(&file_name);
    let file_contents = helpers::file_context(&file_name);
    let tokens: Vec<Tokens> = lexer::lexer::tokenize(&file_contents.unwrap());
    print_lex_tokens(&tokens);
    let (parser_tree, vars): (Vec<Stmt>, HashMap<String, Expr>) = parser::parser::parse(&tokens);
    print_parse_tokens(&parser_tree);
    let _comp = helpers::compile(&parser_tree, vars);
    Ok(())
}
