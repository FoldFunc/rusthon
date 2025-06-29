use crate::lexer::lexer::Tokens;
use std::error::Error;
#[derive(Debug)]
pub enum Expr {
    Number(i64),
}
#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
}
pub struct Parser {
    tokens: Vec<Tokens>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
    fn current(&self) -> &Tokens {
        self.tokens.get(self.position).unwrap_or(&Tokens::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
    pub fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.current() {
            Tokens::Ident(s) if s == "return" => {
                self.advance();
                let expr = match self.current() {
                    Tokens::Number(n) => {
                        let val = *n;
                        self.advance();
                        Expr::Number(val)
                    }
                    other => {
                        return Err(format!(
                            "Expected number after return stmt found: {:?}",
                            other
                        ));
                    }
                };

                match self.current() {
                    Tokens::SemiColon => {
                        self.advance();
                        Ok(Stmt::Return(expr))
                    }
                    other => {
                        return Err(format!(
                            "Expected semicolon after an return found: {:?}",
                            other
                        ));
                    }
                }
            }
            other => return Err(format!("Unexpected token at start of a stmt: {:?}", other)),
        }
    }
}

pub fn parse(tokens: &Vec<Tokens>) -> Result<Stmt, Box<dyn Error>> {
    let mut parser = Parser::new(tokens.to_vec());
    let stmt = parser.parse_stmt()?;
    Ok(stmt)
}
