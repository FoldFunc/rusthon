mod file_helpers;
mod lexer;
mod parser_ast;
mod printhelpers;
use lexer::lexer::Tokens;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let path = file_helpers::take_command_line_args()?;
    let _ = printhelpers::print_path_to_file(&path);
    let _ = file_helpers::check_valid_path(path.clone())?;
    let file_contents = file_helpers::give_file_content(path.clone())?;
    let _ = printhelpers::print_file_contents(&file_contents);
    let tokens: Vec<Tokens> = lexer::lexer::tokenize(file_contents)?;
    let _ = printhelpers::print_tokens(&tokens);
    let ast = parser_ast::parser::parse(&tokens);
    let _ = printhelpers::print_ast(&ast.as_ref().unwrap());
    let _ = file_helpers::gen_begging();
    let _ = file_helpers::gen_asm(&ast.unwrap());
    Ok(())
}
