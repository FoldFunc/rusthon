#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Number(i32), // a number like in return '1'
    Ident(String), // Something like 'var' or 'return'
    Char(char), // 'c' eg.
    Type(String), // A type of a variable
    Return, // a 'return' keyword
    Var, // 'var'
    Eq, // '='
    SemiColon,
    EOF,
}
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}
impl Lexer {
    pub fn new(input: &String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }
    pub fn peek(&self) -> Option<char> {
        return self.input.get(self.position).cloned();
    }
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.position += 1;
        return ch;
    }
    pub fn next_token(&mut self) -> Tokens {
        self.skip_white_space();
        match self.advance() {
            Some(ch) if ch.is_ascii_digit() => self.lex_number(ch),
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => self.lex_ident(ch),
            Some(';') => Tokens::SemiColon,
            Some('=') => Tokens::Eq,
            Some(':') => self.lex_type(),
            Some('\'') => self.lex_char(),
            None => Tokens::EOF,
            Some(c) => panic!("Lexer error: Unexpected token: {}", c),
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
    pub fn lex_char(&mut self) -> Tokens {
        let ch = match self.peek() {
            Some(c) => {
                self.advance();
                c
            }
            None => panic!("Unexpected end of input in character literal"),
        };

        match self.peek() {
            Some('\'') => self.advance(), // Skip closing quote
            _ => panic!("Expected closing quote in character literal"),
        };
        Tokens::Char(ch)
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
        return Tokens::Number(num.parse().unwrap());
    }
    pub fn lex_type(&mut self) -> Tokens {
        self.advance();
        let mut typee = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_whitespace() {
                break;
            } else {
                typee.push(ch);
                self.advance();
            }
        }
        if typee == "int32" {
            return Tokens::Type(typee);
        } else if typee == "char" {
            return Tokens::Type(typee);
        } else {
            panic!("Invalid type of var: {}", typee);
        }
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
        return Tokens::Ident(ident);
    }


}
pub fn tokenize(content: &String) -> Vec<Tokens> {
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut lexer = Lexer::new(content);
    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());
        if token.clone() == Tokens::EOF {
            break;
        }
    }
    return tokens;
}
