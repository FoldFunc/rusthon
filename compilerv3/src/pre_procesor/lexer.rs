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
    Func_Decl,
    Var_Decl,
    Return,
    SemiColon,
    Assign,
    Ident {
        name: String,
    },
    Number {
        val: i32,
    },
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
            Some(';') => Token::SemiColon,
            Some(')') => Token::RightParent,
            Some('(') => Token::LeftParent,
            Some('{') => Token::LeftSBracket,
            Some('}') => Token::RightSBracket,
            Some('=') => Token::Assign,
            Some('~') => self.lex_comment(),
            None => Token::EOF,
            Some(c) => panic!("Not supported token: {}", c),
        }
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
            c => Token::Ident{name: c.to_string()},
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
        return Token::Number{val: ident.parse().unwrap()};
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
        return Token::Number{val: ident.parse().unwrap()};
    }
}
pub fn find_var_offset(offset: &[(String, i32)], name: &str) -> i32 {
    for (var_name, off) in offset {
        if var_name == name {
            return *off;
        }
    }
    panic!("Variable '{}' out of scope", name);
}
