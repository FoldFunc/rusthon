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
    Var_Update,
    Return,
    SemiColon,
    Assign,
    MinusEq,
    PlusEq,
    MulEq,
    DivEq,
    Ident {
        name: String,
    },
    Number {
        val: i32,
    },
    EqulesDouble,
    Plus,
    Minus,
    Mul,
    Div,
    MoreThan,
    LessThan,
    ArrowLeft,
    ArrowRight,
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
            Some('+') => {
                if self.peek().unwrap_or(' ') == '=' {
                    self.advance();
                    return Token::PlusEq;
                }
                return Token::Plus;
            },
            Some('-') => {
                if self.peek().unwrap_or(' ') == '=' {
                    self.advance();
                    return Token::MinusEq;
                }
                return Token::Minus; 
            },
            Some('*') => Token::Mul,
            Some('/') => Token::Div,
            Some(')') => Token::RightParent,
            Some('(') => Token::LeftParent,
            Some('{') => Token::LeftSBracket,
            Some('}') => Token::RightSBracket,
            Some('>') => {
                if self.peek().unwrap_or(' ') == '=' {
                    self.advance();
                    return Token::MoreThan;
                } else {
                    return Token::ArrowRight;
                }
            }
            Some('<') => {
                if self.peek().unwrap_or(' ') == '=' {
                    self.advance();
                    return Token::LessThan;
                } else {
                    return Token::ArrowLeft;
                }
            }
            Some('=') => {
                if self.peek().unwrap_or(' ') == '=' {
                    self.advance();
                    return Token::EqulesDouble;
                } else {
                    return Token::Assign;
                }
            },
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
            "update" => Token::Var_Update,
            c => Token::Ident{name: c.to_string()},
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
        return Token::Number{val: ident.parse().unwrap()};
    }
}
