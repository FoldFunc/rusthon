use std::char;

#[derive(Debug, PartialEq, Clone)]
pub enum Typees {
    Int32,
    Char,
    Stringg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    Func_Decl,
    Var_Update,
    Var_Decl,
    Return,
    SemiColon,
    Assign,
    AssignQuick,
    Ident(String),
    Stringg(String),
    Char(char),
    Number(i32),
    Type(Typees),
    LeftParent,
    RightParent,
    LeftSBracket,
    RightSBracket,
    Comment,
    EOF,
}

pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize,
}

impl Lexer {
    pub fn new(cont: &String) -> Self {
        Lexer {
            input: cont.chars().collect(),
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        ch
    }

    pub fn skip_white_space(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        match self.advance() {
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => self.lex_ident(ch),
            Some(ch) if ch.is_ascii_digit() => self.lex_number(ch),
            Some('\'') => self.lex_char(),
            Some('\"') => self.lex_string(),
            Some(';') => Token::SemiColon,
            Some('+') => Token::Plus,
            Some('*') => Token::Mul,
            Some('/') => Token::Div,
            Some(')') => Token::RightParent,
            Some('(') => Token::LeftParent,
            Some('{') => Token::LeftSBracket,
            Some('}') => Token::RightSBracket,
            Some('=') => Token::Assign,
            Some('~') => self.lex_comment(),
            Some(':') => self.lex_no_type(),
            Some('-') => {
                if self.peek() == Some('>') {
                    self.advance(); // consume '>'
                    self.lex_type()
                } else {
                    Token::Minus
                }
            }
            None => Token::EOF,
            Some(c) => panic!("Not supported token: {}", c),
        }
    }
    pub fn lex_comment(&mut self) -> Token {
        self.skip_white_space();
        while let Some(c) = self.peek() {
            println!("c: {}", c);
            if c == '~' {
                println!("break here you bozo");
                break;
            } else {
                self.advance();
                continue;
            }
        }
        println!("return");
        self.advance();
        return Token::Comment;
    }
    pub fn lex_string(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == ' ' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.advance();
        return Token::Stringg(ident);
    }
    pub fn lex_char(&mut self) -> Token {
        let ch: char;
        println!("self.peek: {}", self.peek().unwrap());
        if self.peek().unwrap().is_ascii() {
            ch = self.peek().unwrap();
            self.advance();
            if self.peek().unwrap() != '\'' {
                panic!("Char must be 1 character.");
            }
        } else {
            panic!("Invalid in char");
        }
        self.advance();
        return Token::Char(ch);
    }
    pub fn lex_no_type(&mut self) -> Token {
        println!("self.peek(): {:?}", self.peek());
        if self.peek().unwrap() != '=' {
            panic!("Unsuported after ':': {:?}", self.peek());
        } else {
            self.advance();
            return Token::AssignQuick;
        }
    }
    pub fn lex_type(&mut self) -> Token {
        self.skip_white_space();
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "int32" => Token::Type(Typees::Int32),
            "char" => Token::Type(Typees::Char),
            "string" => Token::Type(Typees::Stringg),
            _ => panic!("Invalid type"),
        }
    }

    pub fn lex_ident(&mut self, ch: char) -> Token {
        let mut ident = ch.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "function" => Token::Func_Decl,
            "return" => Token::Return,
            "let" => Token::Var_Decl,
            "update" => Token::Var_Update,
            c => Token::Ident(c.to_string()),
        }
    }

    pub fn lex_number(&mut self, ch: char) -> Token {
        let mut ident = ch.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(ident.parse().unwrap())
    }
}
