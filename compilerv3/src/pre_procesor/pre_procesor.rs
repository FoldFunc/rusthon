#[derive(Debug, PartialEq)]
pub enum Token {
    Func_Decl,
    Return,
    SemiColon,
    Ident(String),
    Number(i32),
    LeftParent,
    RightParent,
    LeftSBracket,
    RightSBracket,
    EOF,
}
pub struct Lexer {
    input: Vec<char>,
    position: usize,
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
            Some(';') => Token::SemiColon,
            Some('(') => Token::LeftParent,
            Some(')') => Token::RightParent,
            Some('{') => Token::LeftSBracket,
            Some('}') => Token::RightSBracket,
            None => Token::EOF,
            Some(c) => panic!("Not supported token: {}", c),
        }
    }
    pub fn lex_ident(&mut self, ch: char) -> Token {
        let mut ident = ch.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            }else {
                break;
            }
        }
        match ident.as_str() {
            "function" => Token::Func_Decl,
            "return" => Token::Return,
            c => Token::Ident(c.to_string()),
        }
    }
    pub fn lex_number(&mut self, ch: char) -> Token {
        let mut ident = ch.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                ident.push(c);
            } else {
                break;
            }
        }
        Token::Number(ident.parse().unwrap())
    }
}
pub fn pre_proces(file: &String) {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(file);
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }
    println!("Tokens: {:?}", tokens);

}
