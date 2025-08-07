/*
    Mission for tommorow (as reading today):
    Re factor deez nuts (parser),
    and also assembly generation acordingly.

    It needs to be a hecking tree not a list you bozo.

    This will allow to add another step of complicity
    by adding the IR (intermidiate representation)
    to this compiler.
    
    So this file should take the Vec<Token>
    and return a Abstract syntax TREE,
    This TREE shouldn't have any values simplified
    ex:
        input: return (1 + 3 / 4 - (10 * 9))
        syntax tree for that:
         Return
           |
         Binary(-)
        /         \
   Binary(+)      Binary(*)
   /       \      /       \
  1      Binary(/)      10   9
          /    \
         3      4
*/
use crate::pre_procesor::lexer::Token;
use crate::pre_procesor::lexer::Typees;
use crate::pre_procesor::ast::Ast;
use crate::pre_procesor::stmt::Stmt;
#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
    vars: Vec<(String, String)>,
    ast: Ast,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
            vars: Vec::new(),
            ast: Ast::new(),
        }
    }
    fn expect(&mut self, expected: Token) {
        if self.current() != &expected {
            panic!("Expected {:?}, found {:?}", expected, self.current());
        }
        self.advance();
    }

    fn prev(&self) -> &Token {
        if self.position >= 1 {
            self.tokens.get(self.position - 1).unwrap_or(&Token::EOF)
        } else {
            return &Token::EOF;
        }
    }
    fn next(&self) -> &Token {
        if self.position < self.tokens.len() {
            self.tokens.get(self.position + 1).unwrap_or(&Token::EOF)
        } else {
            return &Token::EOF;
        }
    }
    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn eat(&mut self, expected: &Token) -> bool {
        if self.current() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_return(&mut self) -> Stmt {
        self.advance(); // consume `return`
        let val = match self.current() {
            Token::Number(n) => {
                let v = *n;
                self.advance();
                v.to_string()
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            Token::Ident(s) => {
                let v = s.clone();
                self.advance();
                v
            }
            _ => panic!("Expected number after return"),
        };
        assert!(self.eat(&Token::SemiColon));
        if val.parse::<i32>().is_ok() {
            assert!(val.parse::<i32>().unwrap() <= 256 && val.parse::<i32>().unwrap() >= 0);
        }
        Stmt::Ret { val }
    }
    fn parse_var_decl(&mut self) -> Stmt {
        self.advance();
        let name = match self.current() {
            Token::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => panic!("Invalid var name"),
        };
        if self.eat(&Token::AssignQuick) {
            let val = match self.current() {
                Token::Ident(s) => {
                    let mut rval = " ".to_string();
                    for (name, val) in &self.vars {
                        if &s == &name {
                            rval = val.to_string();
                        } else {
                            continue;
                        }
                    }
                    if rval == " " {
                        panic!("Invlaid var name");
                    }
                    self.advance();
                    rval.to_string()
                }
                Token::Number(n) => {
                    let nret = n.clone();
                    self.advance();
                    nret.to_string()
                }
                Token::Char(c) => {
                    let ccar = c.clone();
                    self.advance();
                    ccar.to_string()
                }
                Token::LeftParent => {
                    let rawdawg: i32 = self.parse_binary();
                    rawdawg.to_string()
                }
                _ => panic!("Invalid value: {:?}", self.current()),
            };
            assert!(self.eat(&Token::SemiColon));
            self.vars.push((name.clone(), val.clone()));
            return Stmt::VarQuick {
                name: name,
                val: val,
            };
        }
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
                Typees::Char => "char".to_string(),
                Typees::Stringg => "string".to_string(),
                Typees::Boolean => "boolean".to_string(),
                Typees::List(n) => format!("list<{:?}>", n),
            },
            _ => panic!("Invalid type"),
        };
        self.advance();
        assert!(self.eat(&Token::Assign));
        let val = match self.current() {
            Token::Number(n) => {
                let nret = n.clone();
                self.advance();
                nret.to_string()
            }
            Token::Char(c) => {
                let ccor = c.clone();
                self.advance();
                ccor.to_string()
            }
            Token::Stringg(s) => {
                let sret = s.clone();
                self.advance();
                sret.to_string()
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            Token::List(l) => {
                let mut lret: String = String::new();

                let mut i = 0;
                while i < l.len() {
                    let token = &l[i];
                    match token {
                        Token::Number(n) => {
                            lret.push_str(&n.to_string());
                            lret.push(' ');
                            i += 1;
                        }
                        Token::Char(c) => {
                            lret.push_str(&c.to_string());
                            lret.push(' ');
                            i += 1;
                        }
                        Token::Stringg(s) => {
                            lret.push_str(&s);
                            lret.push(' ');
                            i += 1;
                        }
                        Token::LeftParent => {
                            // Parse the expression from the list slice starting at i
                            let mut subparser = Parser {
                                tokens: l[i..].to_vec(), // Clone from current list index
                                position: 0,
                                vars: self.vars.clone(),
                                ast: Ast::new(),
                            };
                            let value = subparser.parse_binary();
                            lret.push_str(&value.to_string());
                            lret.push(' ');

                            // Skip over the tokens that were parsed
                            let consumed = subparser.position;
                            i += consumed;
                        }
                        _ => panic!("Invalid type inside list: {:?}", token),
                    }
                }

                self.advance(); // consume the List token
                lret
            }
            _ => panic!("Invalid value: {:?}", self.current()),
        };
        assert!(self.eat(&Token::SemiColon));
        self.vars.push((name.clone(), val.clone()));
        return Stmt::Var {
            name: name,
            typee: typee,
            val: val,
        };
    }
    pub fn double_is(&mut self) -> bool {
        let mut tmp = self.clone();
        let mut is = false;
        while tmp.current() != &Token::RightParent {
            match tmp.current() {
                Token::DoubleIs => is = true,
                _ => is = is,
            }
            tmp.advance();
        }
        return is;
    }
    pub fn parse_binary_good(&mut self) -> i32 {
        let mut if_ret = 1; // 1 - false, 0 - true 
        let mut first = String::new();
        let mut second = String::new();
        let mut change = false;
        self.advance();
        while self.current() != &Token::RightParent {
            match self.current() {
                Token::Number(n) if change == false => first.push_str(&n.to_string()),
                Token::Number(n) => second.push_str(&n.to_string()),
                Token::Ident(s) if change == false => first.push_str(s),
                Token::Ident(s) => second.push_str(s),
                Token::DoubleIs => change = true,
                t => panic!("invalid in if: t: {:?}", t),
            }
            self.advance();
        }
        if !first.parse::<i32>().is_ok() {
            for (name, val) in &self.vars {
                if first == name.to_string() {
                    first = val.to_string();
                } else {
                    continue;
                }
            }
        }
        if !second.parse::<i32>().is_ok() {
            for (name, val) in &self.vars {
                if second == name.to_string() {
                    second = val.to_string();
                } else {
                    continue;
                }
            }
        }
        if first.parse::<i32>() == second.parse::<i32>() {
            if_ret = 0;
        } else {
            if_ret = 1;
        }
        self.advance();
        return if_ret;
    }
    fn parse_binary(&mut self) -> i32 {
        if self.double_is() {
            self.parse_binary_good()
        } else {
            self.parse_expr()
        }
    }

    fn parse_expr(&mut self) -> i32 {
        let mut left = self.parse_term();
        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_term();
            match op {
                Token::Plus => left += right,
                Token::Minus => left -= right,
                _ => unreachable!(),
            }
        }
        left
    }

    fn parse_term(&mut self) -> i32 {
        let mut left = self.parse_factor();
        while matches!(self.current(), Token::Mul | Token::Div) {
            let op = self.current().clone();
            self.advance();
            let right = self.parse_factor();
            match op {
                Token::Mul => left *= right,
                Token::Div => left /= right,
                _ => unreachable!(),
            }
        }
        left
    }

    fn parse_factor(&mut self) -> i32 {
        match self.current().clone() {
            Token::Number(n) => {
                self.advance();
                n
            }
            Token::Ident(s) => {
                self.advance();
                let mut curr_val: i32 = -1;
                for (name, val) in self.vars.to_vec() {
                    if name == s {
                        curr_val = val.parse::<i32>().unwrap();
                    }
                }
                if curr_val == -1 {
                    panic!("No such variable");
                }
                curr_val
            }
            Token::LeftParent => {
                self.advance();
                let val = self.parse_expr();
                if !self.eat(&Token::RightParent) {
                    panic!("Expected `)`");
                }
                val
            }
            _ => panic!("Unexpected token in factor: {:?}", self.current()),
        }
    }

    fn parse_fn(&mut self) -> Stmt {
        self.advance(); // consume `fn`
        let mut name = match self.current() {
            Token::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => panic!("Expected function name"),
        };
        assert!(self.eat(&Token::LeftParent));
        assert!(self.eat(&Token::RightParent));
        assert!(self.eat(&Token::LeftSBracket));

        let mut body = vec![];
        while self.current() != &Token::RightSBracket {
            body.push(self.parse_stmt());
        }
        if name == "main" {
            name = "main".to_string();
        }
        assert!(self.eat(&Token::RightSBracket));
        Stmt::Fn { name, body }
    }
    fn parse_var_re_decl(&mut self) -> Stmt {
        self.advance();
        let name = match self.current() {
            Token::Ident(s) => {
                let n = s.clone();
                self.advance();
                n
            }
            _ => panic!("Invalid var name"),
        };
        if self.eat(&Token::AssignQuick) {
            let val = match self.current() {
                Token::Number(n) => {
                    let nret = n.clone();
                    self.advance();
                    nret.to_string()
                }
                Token::Char(c) => {
                    let ccar = c.clone();
                    self.advance();
                    ccar.to_string()
                }
                Token::LeftParent => {
                    let rawdawg: i32 = self.parse_binary();
                    rawdawg.to_string()
                }
                _ => panic!("Invalid value: {:?}", self.current()),
            };
            assert!(self.eat(&Token::SemiColon));
            self.vars.push((name.clone(), val.clone()));
            return Stmt::ReVarQuick {
                name: name,
                val: val,
            };
        }
        let typee = match self.current() {
            Token::Type(t) => match t {
                Typees::Int32 => "int32".to_string(),
                Typees::Char => "char".to_string(),
                Typees::Stringg => "string".to_string(),
                Typees::Boolean => "boolean".to_string(),
                Typees::List(n) => format!("list<{:?}>", n),
            },
            _ => panic!("Invalid type"),
        };
        self.advance();
        assert!(self.eat(&Token::Assign));
        let val = match self.current() {
            Token::Number(n) => {
                let nret = n.clone();
                self.advance();
                nret.to_string()
            }
            Token::Char(c) => {
                let ccar = c.clone();
                self.advance();
                ccar.to_string()
            }
            Token::LeftParent => {
                let rawdawg: i32 = self.parse_binary();
                rawdawg.to_string()
            }
            _ => panic!("Invalid value: {:?}", self.current()),
        };
        assert!(self.eat(&Token::SemiColon));
        self.vars.push((name.clone(), val.clone()));
        return Stmt::ReVar {
            name: name,
            typee: typee,
            val: val,
        };
    }
    pub fn parse_if(&mut self) -> Stmt {
        self.advance(); // Skip 'if'
        let condition = self.parse_binary() == 0;

        self.expect(Token::LeftSBracket); // Expect '{'
        let mut body = vec![];
        while self.current() != &Token::RightSBracket {
            body.push(self.parse_stmt());
        }
        self.expect(Token::RightSBracket); // Expect '}'

        Stmt::If { condition, body }
    }

    pub fn parse_elif(&mut self) -> Stmt {
        self.advance(); // Skip 'elif'
        let condition = self.parse_binary() == 0;

        self.expect(Token::LeftSBracket); // Expect '{'
        let mut body = vec![];
        while self.current() != &Token::RightSBracket {
            body.push(self.parse_stmt());
        }
        self.expect(Token::RightSBracket); // Expect '}'

        Stmt::Elif { condition, body }
    }

    pub fn parse_else(&mut self) -> Stmt {
        self.advance(); // Skip 'else'

        self.expect(Token::LeftSBracket); // Expect '{'
        let mut body = vec![];
        while self.current() != &Token::RightSBracket {
            body.push(self.parse_stmt());
        }
        self.expect(Token::RightSBracket); // Expect '}'
        Stmt::Else { body }
    }
    pub fn parse_stmt(&mut self) -> Stmt {
        println!("self.prev: {:?}", self.prev());
        println!("self.current: {:?}", self.current());
        println!("self.next: {:?}", self.next());

        match self.current() {
            Token::Return => self.parse_return(),
            Token::Func_Decl => self.parse_fn(),
            Token::Var_Decl => self.parse_var_decl(),
            Token::Var_Update => self.parse_var_re_decl(),
            Token::If => self.parse_if(),
            Token::Elif => self.parse_elif(),
            Token::Else => self.parse_else(),
            _ => panic!("Unexpected token: {:?}", self.current()),
        }
    }
    pub fn parse(&mut self) -> Ast {
        while self.current() != &Token::EOF {
            let stmt = self.parse_stmt();
            self.ast.push(stmt);
        }

        for stmt in &mut self.ast.stmts {
            if let Stmt::Fn { body, .. } = stmt {
                let mut pruned = vec![];
                let mut i = 0;
                let mut taken = false;

                while i < body.len() {
                    match &body[i] {
                        Stmt::If {
                            condition,
                            body: inner,
                        } if *condition && !taken => {
                            pruned.push(Stmt::Condition {
                                body: inner.clone(),
                            });
                            taken = true;
                        }
                        Stmt::Elif {
                            condition,
                            body: inner,
                        } if *condition && !taken => {
                            pruned.push(Stmt::Condition {
                                body: inner.clone(),
                            });
                            taken = true;
                        }
                        Stmt::Else { body: inner } if !taken => {
                            pruned.push(Stmt::Condition {
                                body: inner.clone(),
                            });
                            taken = true;
                        }
                        _ => {
                            // only keep non-conditional statements
                            if !matches!(
                                body[i],
                                Stmt::If { .. } | Stmt::Elif { .. } | Stmt::Else { .. }
                            ) {
                                pruned.push(body[i].clone());
                            }
                        }
                    }
                    i += 1;
                }

                *body = pruned;
            }
        }

        println!("self.ast:\n{}", self.ast);
        self.ast.clone()
    }
}
