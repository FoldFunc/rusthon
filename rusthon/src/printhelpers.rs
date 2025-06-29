use crate::lexer;
use crate::parser_ast::parser::Stmt;
pub fn print_path_to_file(path: &String) {
    println!("Path to file to compile: {}", *path);
}
pub fn print_file_contents(file_contents: &String) {
    println!("File contents raw: \n{:?}", *file_contents);
}
pub fn print_tokens(tokens: &Vec<lexer::lexer::Tokens>) {
    for token in tokens {
        println!("token: {:?}", token);
    }
}
pub fn print_ast(ast: &Stmt) {
    println!("ast: \n{:?}", ast);
}
