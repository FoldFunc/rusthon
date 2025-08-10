use std::fmt::Binary;

use crate::pre_procesor::ast::{Ast, Node_Fucntion};
use crate::pre_procesor::lexer::Token;
use crate::pre_procesor::stmt::{Expr, Op, Stmt, Term};

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // Looks at position +x in tokens. None = +0
    pub fn peek(&self, x: Option<i32>) -> Token {
        let x = x.unwrap_or(0) as usize;
        if self.pos + x >= self.tokens.len() {
            panic!("Last position in Tokens reached");
        }
        self.tokens[self.pos + x].clone()
    }

    pub fn advance(&mut self, x: Option<i32>) {
        let x = x.unwrap_or(1);
        if self.pos + x as usize > self.tokens.len() {
            panic!("Last position in Tokens reached");
        }
        self.pos += x as usize;
    }

    pub fn eat(&mut self, expected: Token) -> bool {
        if self.peek(None) == expected {
            self.advance(Some(1));
            true
        } else {
            false
        }
    }

    pub fn parse_term(&mut self) -> Term {
        match self.peek(None) {
            Token::Number { val } => {
                self.advance(Some(1));
                Term::Int_lit { val }
            }
            Token::Ident { name } => {
                self.advance(Some(1));
                Term::Ident { name }
            }
            Token::LeftParent => {
                self.advance(Some(1));
                let expr = self.parse_binary_expr(0);
                assert!(self.eat(Token::RightParent), "Expected ')'");
                Term::Grouped {
                    expr: Box::new(expr),
                }
            }
            other => panic!("Invalid token in parse_term: {:?}", other),
        }
    }
    pub fn parse_binary_expr(&mut self, min_prec: u8) -> Expr {
        let mut left = Expr::Term(self.parse_term());

        while let Some(op) = self.match_operator() {
            let prec = op.precedence();
            if prec < min_prec {
                break;
            }
            self.advance(Some(1)); // consume operator
            let right = self.parse_binary_expr(prec + 1);
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }
    fn match_operator(&self) -> Option<Op> {
        match self.peek(None) {
            Token::Plus => Some(Op::Plus),
            Token::Minus => Some(Op::Minus),
            Token::Mul => Some(Op::Mul),
            Token::Div => Some(Op::Div),
            Token::EqulesDouble => Some(Op::Equals),
            _ => None,
        }
    }
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_binary_expr(0)
    }
    pub fn parse_stmt(&mut self) -> Stmt {
        match self.peek(None) {
            Token::LeftSBracket => {
                self.advance(Some(1));
                let mut stmts: Vec<Stmt> = Vec::new();
                while self.peek(None) != Token::RightSBracket {
                    stmts.push(self.parse_stmt());
                }
                self.advance(Some(1));
                Stmt::Scope { stmts: Box::new(stmts) }
            }
            Token::Ident { name } => {
                self.advance(Some(1));
                if self.eat(Token::Assign) {
                    let expr = self.parse_expr();
                    assert!(self.eat(Token::SemiColon));
                    Stmt::Var {
                        name: name,
                        expr: expr,
                    }
                } else {
                    match self.peek(None) {
                        Token::PlusEq => {
                            self.advance(Some(1));
                            let rhs_expr = self.parse_expr();
                            assert!(self.eat(Token::SemiColon));
                            let lhs_expr = Expr::Term(Term::Ident { name: name.clone() });
                            let new_expr = Expr::Binary {
                                left: Box::new(lhs_expr),
                                op: Op::Plus,
                                right: Box::new(rhs_expr),
                            };
                            return Stmt::Var {
                                name: name,
                                expr: new_expr,
                            };
                        }
                        Token::MinusEq => {
                            self.advance(Some(1));
                            let rhs_expr = self.parse_expr();
                            assert!(self.eat(Token::SemiColon));
                            let lhs_expr = Expr::Term(Term::Ident { name: name.clone() });
                            let new_expr = Expr::Binary {
                                left: Box::new(lhs_expr),
                                op: Op::Minus,
                                right: Box::new(rhs_expr),
                            };
                            return Stmt::Var {
                                name: name,
                                expr: new_expr,
                            };
                        }
                        Token::MulEq => {
                            self.advance(Some(1));
                            let rhs_expr = self.parse_expr();
                            assert!(self.eat(Token::SemiColon));
                            let lhs_expr = Expr::Term(Term::Ident { name: name.clone() });
                            let new_expr = Expr::Binary {
                                left: Box::new(lhs_expr),
                                op: Op::Mul,
                                right: Box::new(rhs_expr),
                            };
                            return Stmt::Var {
                                name: name,
                                expr: new_expr,
                            };
                        }
                        Token::DivEq => {
                            self.advance(Some(1));
                            let rhs_expr = self.parse_expr();
                            assert!(self.eat(Token::SemiColon));
                            let lhs_expr = Expr::Term(Term::Ident { name: name.clone() });
                            let new_expr = Expr::Binary {
                                left: Box::new(lhs_expr),
                                op: Op::Div,
                                right: Box::new(rhs_expr),
                            };
                            return Stmt::Var {
                                name: name,
                                expr: new_expr,
                            };
                        }
                        some => panic!("Invalid operator: {:?}", some),
                    }
                }
            }
            Token::Return => {
                self.advance(Some(1));
                let expr = self.parse_expr();
                assert!(self.eat(Token::SemiColon));
                Stmt::Return { expr }
            }
            Token::Var_Decl => {
                self.advance(Some(1));
                let name_token = self.peek(None);
                let name_name: String;
                match name_token {
                    Token::Ident { name } => name_name = name,
                    other => panic!("Invalid type after var name: {:?}", other),
                }
                self.advance(Some(1));
                assert!(self.eat(Token::Assign));
                let expr = self.parse_expr();
                assert!(self.eat(Token::SemiColon));
                Stmt::Var {
                    name: name_name,
                    expr,
                }
            }
            token => panic!("Invalid value in parse_stmt: {:?}", token),
        }
    }

    pub fn find_functions(&mut self) -> Vec<Node_Fucntion> {
        let mut functions = Vec::new();
        let mut i = 0;
        while i < self.tokens.len() {
            if self.tokens[i] == Token::Func_Decl {
                if let Token::Ident { name } = &self.tokens[i + 1] {
                    functions.push(Node_Fucntion {
                        name: name.clone(),
                        stmts: Vec::new(),
                    });
                } else {
                    panic!("Invalid function name at position {}", i + 1);
                }
            }
            i += 1;
        }
        functions
    }

    pub fn parse_function(&mut self, func: &Node_Fucntion) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        // Expect sequence: Func_Decl Ident '(' ')' '{'
        assert!(self.peek(None) == Token::Func_Decl);
        self.advance(Some(1));

        assert!(
            self.peek(None)
                == Token::Ident {
                    name: func.name.clone()
                }
        );
        self.advance(Some(1));

        assert!(self.peek(None) == Token::LeftParent);
        self.advance(Some(1));

        assert!(self.peek(None) == Token::RightParent);
        self.advance(Some(1));

        assert!(self.peek(None) == Token::LeftSBracket);
        self.advance(Some(1));

        while self.peek(None) != Token::RightSBracket {
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }

        self.advance(Some(1)); // consume RightSBracket
        stmts
    }

    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast::new(self.find_functions());
        println!("Found entry point to the program");

        for function in ast.node_funcitons.iter_mut() {
            println!("Parsing function: {:?}", function.name);
            let body = self.parse_function(&function);
            function.stmts = body;
        }
        println!("ast: \n{:?}", ast);
        ast
    }
}
