mod file_helpers;
mod lexer;
mod printhelpers;
use std::error::Error;

use lexer::lexer::Tokens;
fn main() -> Result<(), Box<dyn Error>> {
    let path = file_helpers::take_command_line_args()?;
    let _ = printhelpers::print_path_to_file(&path);
    let _ = file_helpers::check_valid_path(path.clone())?;
    let file_contents = file_helpers::give_file_content(path.clone())?;
    let _ = printhelpers::print_file_contents(&file_contents);
    let tokens: Vec<Tokens> = lexer::lexer::tokenize(file_contents)?;
    let _ = printhelpers::print_tokens(&tokens);
    Ok(())
}
