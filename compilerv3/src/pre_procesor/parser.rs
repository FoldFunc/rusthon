use crate::pre_procesor::ast::{Ast, Node_Fucntion};
use crate::pre_procesor::lexer::Token;
use crate::pre_procesor::stmt::{Expr, Stmt};

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

    pub fn parse_expr(&mut self) -> Expr {
        match self.peek(None) {
            Token::Number{ val } => {
                self.advance(Some(1));
                Expr::Int_lit{ val: val }
            }
            Token::Ident { name } => {
                self.advance(Some(1));
                Expr::Ident{ name: name }
            }
            expr => panic!("Invalid value in parse_expr: {:?}", expr),
        }
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        match self.peek(None) {
            Token::Return => {
                self.advance(Some(1));
                let expr = self.parse_expr();
                assert!(self.eat(Token::SemiColon));
                Stmt::Return { expr }
            }
            Token::Var_Decl => {
                self.advance(Some(1));
                let name = self.peek(None);
                let name_name: String;
                match name {
                    Token::Ident { name } => name_name = name,
                    other => panic!("Invalid type after var name: {:?}", other),
                }
                self.advance(Some(1));
                assert!(self.eat(Token::Assign));
                let expr = self.parse_expr();
                assert!(self.eat(Token::SemiColon));
                Stmt::Var { name: name_name, expr: expr }
            }
            token => panic!("Invalid value in parse_stmt: {:?}", token),
        }
    }

    pub fn find_functions(&mut self) -> Vec<Node_Fucntion> {
        let mut functions = Vec::new();
        let mut i = 0;
        while i < self.tokens.len() {
            if self.tokens[i] == Token::Func_Decl {
                if let Token::Ident{name} = &self.tokens[i + 1] {
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

        assert!(self.peek(None) == Token::Ident{ name: func.name.clone()});
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
        ast
    }
}
