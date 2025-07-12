use std::error::Error;
#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Ident(String),
    Number(i64),
    Char(char),
    Return,
    Var,
    Eq,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    SemiColon,
    Apostrophe,
    EOF,
}
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
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
    pub fn next_token(&mut self) -> Tokens {
        self.skip_white_space();
        match self.advance() {
        Some(ch) if ch.is_ascii_digit() => self.lex_number(ch),
        Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => self.lex_ident(ch),
        Some(';') => Tokens::SemiColon,
        Some('+') => Tokens::Plus,
        Some('-') => Tokens::Minus,
        Some('*') => Tokens::Star,
        Some('/') => Tokens::Slash,
        Some('(') => Tokens::LParen,
        Some(')') => Tokens::RParen,
        Some('=') => Tokens::Eq,
        Some('\'') => {
            // Read one character after opening quote
            let ch = self.advance().expect("Expected character after single quote");
            // Ensure it’s closed properly
            if self.advance() != Some('\'') {
                panic!("Expected closing single quote for character literal");
            }
            Tokens::Char(ch)
        },
        None => Tokens::EOF,
        Some(c) => panic!("Unexpected token: {}", c),
    }
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
    pub fn lex_number(&mut self, first: char) -> Tokens {
        let mut num = first.to_string();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Tokens::Number(num.parse().unwrap())
    }
    pub fn lex_ident(&mut self, first: char) -> Tokens {
        let mut ident = first.to_string();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphabetic() {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if ident == "return" {
            return Tokens::Return;
        }
        if ident == "var" {
            return Tokens::Var;
        }
        Tokens::Ident(ident.parse().unwrap())
    }
}
pub fn tokenize(file_contents: String) -> Result<Vec<Tokens>, Box<dyn Error>> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(file_contents);
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token.clone() == Tokens::EOF {
            break;
        }
    }
    if false {
        return Err("How the hell".into());
    }
    Ok(tokens)
}
