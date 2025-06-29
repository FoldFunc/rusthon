use crate::lexer;
pub fn print_path_to_file(path: &String) {
    println!("Ptah to file to compile: {}", *path);
}
pub fn print_file_contents(file_contents: &String) {
    println!("File contents raw: \n{:?}", *file_contents);
}
pub fn print_tokens(tokens: &Vec<lexer::lexer::Tokens>) {
    for token in tokens {
        println!("token: {:?}", token);
    }
}
