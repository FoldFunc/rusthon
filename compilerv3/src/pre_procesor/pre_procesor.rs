use crate::pre_procesor::lexer;
use crate::pre_procesor::parser::Parser;
use crate::pre_procesor::ast::Ast;
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
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    return ast;
}
