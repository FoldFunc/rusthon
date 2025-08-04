pub fn eval_expression(tokens: Vec<String>) -> i32 {
    let mut parser = Parser::new(tokens);
    parser.parse_expr().unwrap_or(0)
}

struct Parser {
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<String>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(String::as_str)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Result<i32, ()> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<i32, ()> {
        let mut result = self.parse_mul_div()?;
        while let Some(op) = self.current() {
            match op {
                "+" => {
                    self.advance();
                    result += self.parse_mul_div()?;
                }
                "-" => {
                    self.advance();
                    result -= self.parse_mul_div()?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_mul_div(&mut self) -> Result<i32, ()> {
        let mut result = self.parse_factor()?;
        while let Some(op) = self.current() {
            match op {
                "*" => {
                    self.advance();
                    result *= self.parse_factor()?;
                }
                "/" => {
                    self.advance();
                    let divisor = self.parse_factor()?;
                    result = if divisor != 0 { result / divisor } else { 0 };
                }
                _ => break,
            }
        }
        Ok(result)
    }

    fn parse_factor(&mut self) -> Result<i32, ()> {
        if let Some(tok) = self.current() {
            if tok == "(" {
                self.advance();
                let val = self.parse_expr()?;
                if self.current() == Some(")") {
                    self.advance();
                    Ok(val)
                } else {
                    Err(()) // missing closing parenthesis
                }
            } else {
                self.parse_number()
            }
        } else {
            Err(())
        }
    }

    fn parse_number(&mut self) -> Result<i32, ()> {
        if let Some(tok) = self.current() {
            if let Ok(num) = tok.parse::<i32>() {
                self.advance();
                Ok(num)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

