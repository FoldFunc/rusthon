use std::fs;
use crate::post_processor::air::Air;
use crate::post_processor::simplifier::Simplyfier;
use crate::pre_procesor::ast::Ast;
pub fn get_file(file_path: &String) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Couldn't read from a file");
    return contents;
}
pub fn post_proces(ast: &Ast) -> Air {
    let mut simplifier = Simplyfier::new(ast);
    let ir = simplifier.simplify();
    return ir;
}
