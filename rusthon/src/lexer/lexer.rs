#[derive(Debug, PartialEq)]
pub enum Tokens {
    Ident(String),
    Number(i64),
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
        match self.advance {
            Some(ch) if ch.is_ascii_digit() => self.lex_number(ch),
            Some(ch) if ch.is_ascii_alphabetic() => self.lex_ident(ch),
            None => Tokens::EOF,
        }
    }
    pub fn skip_white_space() {
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
        Tokens::Ident(ident.parse().unwrap())
    }
}
