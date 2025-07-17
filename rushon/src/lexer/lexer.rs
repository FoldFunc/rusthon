#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Number(i32),         // a number like 1
    Ident(String),       // variable names
    Char(char),          // single character like 'a'
    List(Vec<Tokens>),   // [1, 'a', 2]
    Type(String),        // :int32, :char
    Return,              // 'return'
    Var,                 // 'var'
    Eq,                  // '='
    SemiColon,           // ';'
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

    pub fn next_token(&mut self) -> Tokens {
        self.skip_white_space();
        match self.advance() {
            Some(ch) if ch.is_ascii_digit() => self.lex_number(ch),
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => self.lex_ident(ch),
            Some(';') => Tokens::SemiColon,
            Some('=') => Tokens::Eq,
            Some(':') => self.lex_type(),
            Some('\'') => self.lex_char(),
            Some('[') => self.lex_list(),
            None => Tokens::EOF,
            Some(c) => panic!("Lexer error: Unexpected token: {}", c),
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
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "return" => Tokens::Return,
            "var" => Tokens::Var,
            _ => Tokens::Ident(ident),
        }
    }

    pub fn lex_type(&mut self) -> Tokens {
        self.advance(); // skip ':'
        let mut typee = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_whitespace() {
                break;
            } else {
                typee.push(ch);
                self.advance();
            }
        }
        match typee.as_str() {
            "int32" | "char" | "list" => Tokens::Type(typee),
            _ => panic!("Invalid type: {}", typee),
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
            Some('\'') => {
                self.advance(); // skip closing quote
            }
            _ => panic!("Expected closing quote in character literal"),
        }

        Tokens::Char(ch)
    }

    pub fn lex_list(&mut self) -> Tokens {
        let mut list: Vec<Tokens> = Vec::new();

        loop {
            self.skip_white_space();
            match self.peek() {
                Some(']') => {
                    self.advance(); // consume closing bracket
                    break;
                }
                Some(',') => {
                    self.advance(); // skip comma
                }
                Some('\'') => {
                    self.advance(); // skip opening quote
                    list.push(self.lex_char());
                }
                Some(ch) if ch.is_ascii_digit() => {
                    let digit = self.advance().unwrap();
                    list.push(self.lex_number(digit));
                }
                Some(c) => {
                    panic!("Unexpected character in list: {}", c);
                }
                None => panic!("Unterminated list literal"),
            }
        }

        Tokens::List(list)
    }
}

// Entry point to tokenize a full input string
pub fn tokenize(content: &String) -> Vec<Tokens> {
    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(content);

    loop {
        let token = lexer.next_token();
        if token == Tokens::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    tokens
}

