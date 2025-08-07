use crate::pre_procesor::ir;
use crate::pre_procesor::lexer;
use crate::pre_procesor::lexer::Token;
use crate::pre_procesor::parser;
use crate::pre_procesor::parser::Ast;
pub fn pre_proces(file: &String) -> Ast {
    let mut tokens = Vec::new();
    let mut lexer = lexer::Lexer::new(file);
    loop {
        let token = lexer.next_token();
        if token == lexer::Token::EOF {
            break;
        }
        if token == lexer::Token::Comment {
            continue;
        }
        tokens.push(token);
    }
    for token in &tokens {
        println!("token: {:?}", token);
    }
    println!("\n");
    let mut parser = parser::Parser::new(tokens.to_vec());
    let ast = parser.parse();
    println!("AST debug:\n{:?}", &ast);
    println!("AST:\n{}", &ast);
    let _intermedate = ir::IrGen::new(ast.clone()); // For now no use there is no complexe things
    return ast;
}
