use std::collections::HashMap;
use crate::codegen::codegen::Type;
use crate::lexer::lexer::Tokens;
use crate::codegen::codegen::Stmt;
use crate::codegen::codegen::Expr;
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
    pub fn parse(&mut self) -> (Vec<Stmt>, HashMap<String, Expr>) {
        let mut vars: HashMap<String, Expr> = HashMap::new();
        let mut stmts: Vec<Stmt> = Vec::new();
        while self.current() != &Tokens::EOF {
            let stmt = match self.current() {
                Tokens::Return => {
                    self.advance();
                    let expr = self.parse_expr();
                    assert!(self.eat(&Tokens::SemiColon));
                    Stmt::Return(expr)
                }
                Tokens::Var => {
                    self.advance();
                    let name = match self.current() {
                        Tokens::Ident(s) => {
                            let ident = s.clone();
                            self.advance();
                            ident
                        }
                        _ => panic!("Unexpected token in variable decleration!"),
                    };
                    let typee = self.parse_type();
                    self.advance();
                    assert!(self.eat(&Tokens::Eq));
                    let val = self.parse_expr();
                    assert!(self.eat(&Tokens::SemiColon));
                    vars.insert(name.clone(), val.clone());
                    Stmt::Var { name: name.clone(),typee: typee.clone(), val: val.clone()}
                }
                _ => panic!("Expected statment recived: {:?}", self.current()),
            };
            stmts.push(stmt);
        }
        return (stmts, vars);
    }
    pub fn parse_type(&mut self) -> Type {
        match self.current() {
            Tokens::Type(s) => {
                if s == "char" {
                    return Type::Char;
                } else if s == "int32" {
                    return Type::Int32;
                } else {
                    panic!("Invalid type in var statment: {}", s);
                }
            }
            _ => panic!("Invalid Token in function parse type: {:?}", self.current()),

        }
    }
    pub fn parse_expr(&mut self) -> Expr {
        match self.current() {
            Tokens::Number(i) => {
                let val = *i;
                self.advance();
                return Expr::Number(val);
            }
            Tokens::Ident(s) => {
                let val = s.clone();
                self.advance();
                return Expr::Ident(val);
            }
            Tokens::Char(c) => {
                let val = c.clone();
                self.advance();
                return Expr::Char(val);
            }
            _ => panic!("Invalid Expr: {:?}", self.current()),
        }
    }
}
pub fn parse(lex_tokens: &Vec<Tokens>) -> (Vec<Stmt>, HashMap<String, Expr>) {
    let mut parser = Parser::new(lex_tokens.to_vec());
    return parser.parse();
}
