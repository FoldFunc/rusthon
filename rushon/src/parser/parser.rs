use crate::lexer::lexer::Tokens;
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
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
        Parser { tokens: tokens, position: 0 }
    }
    fn current(&self) -> &Tokens {
        return self.tokens.get(self.position).unwrap_or(&Tokens::EOF);
    }
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }
    fn eat(&mut self, expected: &Tokens) -> bool {
        if self.current() == expected {
            self.advance();
            return true;
        }
        return false;
    }
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.current() != &Tokens::EOF {
            let stmt = match self.current() {
                Tokens::Return => {
                    self.advance();
                    let expr = self.parse_expr();
                    assert!(self.eat(&Tokens::SemiColon));
                    Stmt::Return(expr)
                }
                _ => panic!("Expected statment recived: {:?}", self.current()),
            };
            stmts.push(stmt);
        }
        return stmts;
    }
    pub fn parse_expr(&mut self) -> Expr {
        match self.current() {
            Tokens::Number(i) => {
                let val = *i;
                self.advance();
                return Expr::Number(val);
            }
            _ => panic!("Invalid expr in return statment: {:?}", self.current()),
        }
    }
}
pub fn parse(lex_tokens: &Vec<Tokens>) -> Vec<Stmt> {
    let mut parser = Parser::new(lex_tokens.to_vec());
    return parser.parse();
}
