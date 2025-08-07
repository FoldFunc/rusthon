use std::char;

#[derive(Debug, PartialEq, Clone)]
pub enum Typees {
    Int32,
    Char,
    Stringg,
    Boolean,
    List(Box<Typees>),
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
    DoubleIs,
    AssignQuick,
    Ident(String),
    Stringg(String),
    Char(char),
    Number(i32),
    List(Vec<Token>),
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
            Some('[') => self.lex_list(),
            Some('=') => {
                if self.peek() == Some('=') {
                    self.advance();
                    Token::DoubleIs
                }else {
                    Token::Assign
                }
            },
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
    pub fn lex_list(&mut self) -> Token {
        self.skip_white_space();

        let mut arr: Vec<Token> = Vec::new();

        loop {
            self.skip_white_space();

            if let Some(']') = self.peek() {
                self.advance(); // end of list
                break;
            }
            match self.peek() {
                Some(ch) if ch.is_ascii_digit() => {
                    arr.push(self.lex_number_list());
                }
                Some('(') => {
                    self.advance();
                    arr.push(Token::LeftParent);
                }
                Some(')') => {
                    self.advance();
                    arr.push(Token::RightParent);
                }
                Some('+') => {
                    self.advance();
                    arr.push(Token::Plus);
                }
                Some('-') => {
                    self.advance();
                    arr.push(Token::Minus);
                }
                Some('*') => {
                    self.advance();
                    arr.push(Token::Mul);
                }
                Some('/') => {
                    self.advance();
                    arr.push(Token::Div);
                }
                Some(',') => {
                    self.advance(); // just a separator between items
                }
                Some(other) => {
                    panic!("Unexpected character in list: '{}'", other);
                }
                None => {
                    panic!("Unexpected end of input in list");
                }
            }
        }

        Token::List(arr)
    }
    pub fn lex_comment(&mut self) -> Token {
        self.skip_white_space();
        while let Some(c) = self.peek() {
            if c == '~' {
                break;
            } else {
                self.advance();
                continue;
            }
        }
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
        let mut list_type: String = " ".to_string();
        if ident == "list" {
            list_type = self.lex_list_type();
        }
        if list_type == " " && ident == "list" {
            panic!("No list type");
        }
        match ident.as_str() {
            "int32" => Token::Type(Typees::Int32),
            "boolean" => Token::Type(Typees::Boolean),
            "char" => Token::Type(Typees::Char),
            "string" => Token::Type(Typees::Stringg),
            "list" => Token::Type(Typees::List(match list_type.as_str() {
                "int32" => Box::new(Typees::Int32),
                "string" => Box::new(Typees::Stringg),
                "char" => Box::new(Typees::Char),
                _ => panic!("Invadlid type in list"),
            })),
            _ => panic!("Invalid type"),
        }
    }
    pub fn lex_list_type(&mut self) -> String {
        self.skip_white_space();
        self.advance();
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c == '>' {
                break;
            } else {
                ident.push(c);
                self.advance();
            }
        }
        self.advance();
        return ident;
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
    pub fn lex_number_list(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        return Token::Number(ident.parse().unwrap());
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
