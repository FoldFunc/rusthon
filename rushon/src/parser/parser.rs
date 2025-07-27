use crate::parser::helpers;
use std::collections::HashMap;
use crate::codegen::codegen::Type;
use crate::lexer::lexer::Binary;
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
    fn current(&mut self) -> &Tokens {
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
                    println!("self.current() in var: {:?}", self.current());
                    let name = match self.current() {
                        Tokens::Ident(s) => {
                            let ident = s.clone();
                            self.advance();
                            ident
                        }
                        _ => panic!("Unexpected token in variable decleration!"),
                    };
                    let typee = self.parse_type();
                    println!("Typee: {:?}", typee);
                    assert!(self.eat(&Tokens::Eq));
                    let val = self.parse_expr();
                    println!("val: {:?}", val);
                    println!("self.current(): {:?}", self.current());
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
                let t = match s.as_str() {
                    "char" => Type::Char,
                    "int32" => Type::Int32,
                    "list" => Type::List,
                    "bool" => Type::Bool,
                    _ => panic!("Invalid type of variable."),
                };
                self.advance(); // ✅ Consume the Type token
                t
            }
            _ => panic!("Invalid Token in function parse_type: {:?}", self.current()),
        }
    }

    pub fn parse_list(&mut self,token: &Tokens) -> Expr {
        match token {
            Tokens::Number(i) => {
                let val = i.clone();
                return Expr::Number(val);
            }
            _ => panic!("Invalid type in list"),
        }
    }
    pub fn parse_expr(&mut self) -> Expr {
        match self.current() {
            Tokens::Binary(b) => {
                let mut rawdawg: Vec<String> = Vec::new(); // Give it to me raw dawg.
                for element in b.clone() {
                    match element {
                        Binary::Number(i) => {
                            rawdawg.push(i.to_string());
                        }
                        Binary::Add => {
                            rawdawg.push("+".to_string());
                        }
                        Binary::Sub => {
                            rawdawg.push("-".to_string());
                        }
                        Binary::Mul => {
                            rawdawg.push("*".to_string());
                        }
                        Binary::Div => {
                            rawdawg.push("/".to_string());
                        }
                        Binary::ParL=> {
                            rawdawg.push("[".to_string());
                        }
                        Binary::ParR=> {
                            rawdawg.push("]".to_string());
                        }
                    }
                }
                let mut score = helpers::parse_binary_string_to_value(&rawdawg);
                return Expr::Binary(score);
            }
            Tokens::List(l) => {
                let mut list: Vec<Expr> = Vec::new();
                for i in l.clone() {
                    list.push(self.parse_list(&i));
                }
                self.advance();
                return Expr::List(list);
            }
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
            Tokens::Boolean(b) => {
                let val = b.clone();
                self.advance();
                return Expr::Bool(val);
            }
            _ => panic!("Invalid Expr: {:?}", self.current()),
        }
    }
}
pub fn parse(lex_tokens: &Vec<Tokens>) -> (Vec<Stmt>, HashMap<String, Expr>) {
    let mut parser = Parser::new(lex_tokens.to_vec());
    return parser.parse();
}
