use crate::parser::parser::{Expr, Op, Stmts, Term, AST};
use std::error::Error;
#[derive(Debug, Clone)]
pub enum IrExpr {
    IntLit(i32),
    Variable(String),
    Boolean(bool),
}
#[derive(Debug, Clone)]
pub enum IrStmts {
    Scope { body: Box<Vec<IrStmts>>},
    Return(IrExpr),
    Var { name: String, val: IrExpr },
    VarRe { name: String, val: IrExpr },
    If {condition: bool, body: Box<IrStmts>},
    Else {body: Box<IrStmts> },
}
#[derive(Debug, Clone)]
pub struct IRT {
    pub irstmts: Vec<IrStmts>,
}
impl IRT {
    pub fn new() -> Self {
        IRT {
            irstmts: Vec::new(),
        }
    }
}
#[derive(Clone, Debug)]
enum ExprType {
    Int(i32),
    Str(String),
    Boolean(bool),
}
pub struct Var {
    name: String,
    val: ExprType,
}
pub struct Intermediate {
    ast: AST,
    vars: Vec<Var>,
}
impl Intermediate {
    pub fn new(ast: AST) -> Self {
        Intermediate {
            ast,
            vars: Vec::new(),
        }
    }
    pub fn simplify_binary(&mut self, expr: &Expr) -> Result<i32, Box<dyn Error>> {
        match expr {
            Expr::Term(Term::IntLit { val }) => Ok(*val),
            Expr::Term(Term::Boolean { state }) => {
                if *state {
                    Ok(1)
                } else {
                    Ok(0)
                }
            }
            Expr::Term(Term::VarName { name }) => match self.get_var_val(name)? {
                ExprType::Int(n) => Ok(n),
                ExprType::Boolean(boolean) => Ok(boolean as i32),
                ExprType::Str(_) => Err("Variable did not evaluate to an integer".into()),
            },
            Expr::Binary { left, op, right } => {
                let lval = self.simplify_binary(left)?;
                let rval = self.simplify_binary(right)?;
                let result = match op {
                    Op::IsEq => (lval == rval) as i32,
                    Op::LessThan => (lval < rval) as i32,
                    Op::LessEqThan => (lval <= rval) as i32,
                    Op::MoreThan => (lval > rval) as i32,
                    Op::MoreEqThan => (lval >= rval) as i32,
                    Op::Modulo => lval % rval,
                    Op::Plus => lval + rval,
                    Op::Minus => lval - rval,
                    Op::Mul => lval * rval,
                    Op::Div => {
                        if rval == 0 {
                            return Err("division by zero".into());
                        }
                        lval / rval
                    }
                };
                Ok(result)
            }
        }
    }
    fn get_var_val(&mut self, name: &String) -> Result<ExprType, Box<dyn Error>> {
        if let Some(var) = self.vars.iter().find(|v| v.name == *name) {
            Ok(var.val.clone())
        } else {
            Err(format!("Variable with this name: {} does not exist", name).into())
        }
    }
    pub fn simplify_expr(&mut self, expr: &Expr) -> Result<IrExpr, Box<dyn Error>> {
        match expr {
            Expr::Term(term) => match term {
                Term::IntLit { val } => Ok(IrExpr::IntLit(*val)),
                Term::VarName { name } => Ok(IrExpr::Variable(name.clone())),
                Term::Boolean { state } => Ok(IrExpr::Boolean(*state))
            },
            Expr::Binary { .. } => Ok(IrExpr::IntLit(self.simplify_binary(expr)?)),
        }
    }
    pub fn simplify_stmt(&mut self, stmt: &Stmts) -> Result<IrStmts, Box<dyn Error>> {
        match stmt {
            Stmts::Scope { body } => {
                let mut irstmts: Vec<IrStmts> = Vec::new();
                for stmt in body.iter() {
                    irstmts.push(self.simplify_stmt(stmt)?);
                }
                Ok(IrStmts::Scope { body: Box::new(irstmts) })
            }
            Stmts::Return { val: expr } => {
                let irexpr = self.simplify_expr(expr)?;
                Ok(IrStmts::Return(irexpr))
            }
            Stmts::Var { name, val } => {
                let irexpr = self.simplify_expr(val)?;
                let value: ExprType;
                match irexpr.clone() {
                    IrExpr::IntLit(n) => value = ExprType::Int(n),
                    IrExpr::Variable(val) => value = ExprType::Str(val),
                    IrExpr::Boolean(state) => value = ExprType::Boolean(state),
                };
                self.vars.push(Var {
                    name: name.to_string(),
                    val: value,
                });
                Ok(IrStmts::Var {
                    name: name.to_string(),
                    val: irexpr,
                })
            }
            Stmts::VarRe { name, val } => {
                if !self.vars.iter().any(|v| v.name == *name) {
                    return Err(format!("Variable with this name: {} does not exist", name).into());
                }

                let irexpr = self.simplify_expr(val)?;
                let new_value = match &irexpr {
                    IrExpr::IntLit(n) => ExprType::Int(*n),
                    IrExpr::Boolean(state) => ExprType::Boolean(*state),
                    IrExpr::Variable(var_name) => self.get_var_val(var_name)?,
                };

                if let Some(var) = self.vars.iter_mut().find(|v| v.name == *name) {
                    var.val = new_value;
                }

                Ok(IrStmts::VarRe {
                    name: name.to_string(),
                    val: irexpr,
                })
            }
            Stmts::If { condition, body } => {
                let irexpr = self.simplify_expr(condition)?;
                let bodyir: IrStmts = self.simplify_stmt(body)?;
                let c: bool;
                match irexpr {
                    IrExpr::IntLit(n) => {
                        c = n == 1;
                    }
                    some => {
                        return Err(format!("Imposible: {:?}", some).into());
                    }
                }
                Ok(IrStmts::If{ condition: c, body: Box::new(bodyir)})
            }
            Stmts::Elif { condition, body } => {
                let irexpr = self.simplify_expr(condition)?;
                let bodyir: IrStmts = self.simplify_stmt(body)?;
                let c: bool;
                match irexpr {
                    IrExpr::IntLit(n) => {
                        c = n == 1;
                    }
                    some => {
                        return Err(format!("Imposible: {:?}", some).into());
                    }
                }
                Ok(IrStmts::If{ condition: c, body: Box::new(bodyir)})
            }
            Stmts::Else { body } => {
                let bodyir: IrStmts = self.simplify_stmt(body)?;
                Ok(IrStmts::Else{ body: Box::new(bodyir) })
            }
            //some => Err(format!("Invalid in simplifier: {:?}", some).into()),
        }
    }
    pub fn simplify(&mut self) -> Result<IRT, Box<dyn Error>> {
        let mut irt = IRT::new();
        let stmts = self.ast.stmts.clone();
        for stmt in stmts {
            irt.irstmts.push(self.simplify_stmt(&stmt)?);
        }
        Ok(irt)
    }
}
