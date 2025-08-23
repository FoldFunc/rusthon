use std::error::Error;

#[derive(Clone, PartialEq, Debug)]
pub enum Tokens {
    Ident(String),
    Number(i32),
    Plus,
    Minus,
    Star,
    Slash,
    Return,
    VarDecl,
    Eq,
    IsEq,
    LessThan,
    MoreThan,
    LessEqThan,
    MoreEqThan,
    Modulo,
    Semi,
    True,
    False,
    LeftCurly,
    RightCurly,
    If,
    Elif,
    Else,
    EOF,
}

pub struct Tokenizer {
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    /// Peek at the current character without consuming it.
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// Consume and return the current character (move position forward).
    fn advance_char(&mut self) -> Option<char> {
        if self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    /// Skip whitespace (consumes it)
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.advance_char();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Tokens, Box<dyn Error>> {
        self.skip_whitespace();
        match self.peek_char() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => self.tokenize_ident(),
            Some(c) if c.is_ascii_digit() => self.tokenize_number(),
            Some(';') => {
                self.advance_char(); // consume ';'
                Ok(Tokens::Semi)
            }
            Some('+') => {
                self.advance_char();
                Ok(Tokens::Plus)
            }
            Some('-') => {
                self.advance_char();
                Ok(Tokens::Minus)
            }
            Some('*') => {
                self.advance_char();
                Ok(Tokens::Star)
            },
            Some('/') => {
                self.advance_char();
                Ok(Tokens::Slash)
            },
            Some('{') => {
                self.advance_char();
                Ok(Tokens::LeftCurly)
            },
            Some('}') => {
                self.advance_char();
                Ok(Tokens::RightCurly)
            },
            Some('>') => {
                self.advance_char();
                if let Some(ch) = self.peek_char() {
                    if ch == '=' {
                        self.advance_char();
                        Ok(Tokens::MoreEqThan)
                    } else {
                        Ok(Tokens::MoreThan)
                    }
                } else {
                    Err("Should not happen".to_string().into())
                }
            }
            Some('%') => {
                self.advance_char();
                Ok(Tokens::Modulo)
            }
            Some('<') => {
                self.advance_char();
                if let Some(ch) = self.peek_char() {
                    if ch == '=' {
                        self.advance_char();
                        Ok(Tokens::LessEqThan)
                    } else {
                        Ok(Tokens::LessThan)
                    }
                } else {
                    Err("Should not happen".to_string().into())
                }
            }
            Some('=') => {
                self.advance_char();
                if let Some(ch) = self.peek_char() {
                    if ch == '=' {
                        self.advance_char();
                        Ok(Tokens::IsEq)
                    } else {
                        Ok(Tokens::Eq)
                    }
                } else {
                    Err("Should not happen".to_string().into())
                }
            },
            None => Ok(Tokens::EOF),
            Some(other) => Err(format!("Unexpected character '{}'", other).into()),
        }
    }

    fn tokenize_ident(&mut self) -> Result<Tokens, Box<dyn Error>> {
        let mut ident = String::new();

        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance_char();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "return" => Ok(Tokens::Return),
            "let" => Ok(Tokens::VarDecl),
            "true" => Ok(Tokens::True),
            "false" => Ok(Tokens::False),
            "if" => Ok(Tokens::If),
            "elif" => Ok(Tokens::Elif),
            "else" => Ok(Tokens::Else),
            some => Ok(Tokens::Ident(some.to_string()))
        }
    }

    fn tokenize_number(&mut self) -> Result<Tokens, Box<dyn Error>> {
        let mut num_str = String::new();

        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.advance_char();
            } else {
                break;
            }
        }

        let number: i32 = num_str.parse()?;
        Ok(Tokens::Number(number))
    }

    /// Tokenize the entire input into a Vec<Tokens> (does not append EOF).
    pub fn tokenize(&mut self) -> Result<Vec<Tokens>, Box<dyn Error>> {
        let mut result = Vec::new();
        loop {
            match self.next_token()? {
                Tokens::EOF => break,
                tok => result.push(tok),
            }
        }

        Ok(result)
    }
}
