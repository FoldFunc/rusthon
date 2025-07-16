#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Number(i32), // a number like in return '1'
    Ident(String), // Something like 'var' or 'return'
    Return, // a 'return' keyword
    Var,
    Eq,
    SemiColon, // a ';'
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
