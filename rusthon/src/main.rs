mod file_helpers;
mod lexer;
mod parser_ast;
mod printhelpers;
mod errorer;

use lexer::lexer::Tokens;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let path = file_helpers::take_command_line_args()?;

    printhelpers::print_path_to_file(&path);
    file_helpers::check_valid_path(path.clone())?;

    let file_contents = file_helpers::give_file_content(path)?;
    printhelpers::print_file_contents(&file_contents);

    let tokens: Vec<Tokens> = lexer::lexer::tokenize(file_contents)?;
    printhelpers::print_tokens(&tokens);

    let ast = parser_ast::parser::parse(&tokens)?;
    printhelpers::print_ast(&ast);
    errorer::errorer::find_errors(&ast);

    file_helpers::gen_asm(&ast)?;

    Ok(())
}

