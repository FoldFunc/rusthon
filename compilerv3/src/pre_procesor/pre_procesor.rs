use crate::pre_procesor::ir;
use crate::pre_procesor::lexer;
use crate::pre_procesor::parser;
use crate::pre_procesor::parser::Ast;
pub fn pre_proces(file: &String) -> Ast{
    let mut tokens = Vec::new();
    let mut lexer = lexer::Lexer::new(file);
    loop {
        let token = lexer.next_token();
        if token == lexer::Token::EOF {
            break;
        }
        tokens.push(token);
    }
    println!("Tokens: \n{:?}", tokens);
    let mut parser = parser::Parser::new(tokens.to_vec());
    let ast = parser.parse();
    println!("AST debug:\n{:?}", &ast);
    println!("AST:\n{}", &ast);
    let _intermedate = ir::IrGen::new(ast.clone()); // For now no use there is no complexe things
    return ast;
}
