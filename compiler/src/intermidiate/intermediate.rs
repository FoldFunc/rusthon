use crate::parser::parser::{AST, Expr, Op, Stmts, Term};
use std::error::Error;

#[derive(Debug, Clone)]
pub enum IrExpr {
    IntLit(i32),
    Variable(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum IrStmts {
    Empty,
    Scope {
        body: Box<Vec<IrStmts>>,
    },
    Return(IrExpr),
    Var {
        name: String,
        val: IrExpr,
    },
    VarRe {
        name: String,
        val: IrExpr,
    },
    If {
        condition: bool,
        body: Box<IrStmts>,
        else_body: Option<Box<IrStmts>>,
    },
}

#[derive(Debug, Clone)]
pub struct IRT {
    pub irstmts: Vec<IrStmts>,
}
impl IRT {
    pub fn new() -> Self {
        IRT { irstmts: Vec::new() }
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

    // ---------- Expressions ----------

    pub fn simplify_binary(&mut self, expr: &Expr) -> Result<i32, Box<dyn Error>> {
        match expr {
            Expr::Term(Term::IntLit { val }) => Ok(*val),
            Expr::Term(Term::Boolean { state }) => Ok(if *state { 1 } else { 0 }),
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
                Term::Boolean { state } => Ok(IrExpr::Boolean(*state)),
            },
            Expr::Binary { .. } => Ok(IrExpr::IntLit(self.simplify_binary(expr)?)),
        }
    }

    // ---------- Statement lowering helpers ----------

    /// Lower a single non-if statement.
    fn simplify_stmt_nonif(&mut self, stmt: &Stmts) -> Result<IrStmts, Box<dyn Error>> {
        match stmt {
            Stmts::Scope { body } => {
                let lowered = self.simplify_stmts_list(body)?;
                Ok(IrStmts::Scope { body: Box::new(lowered) })
            }
            Stmts::Return { val: expr } => {
                let irexpr = self.simplify_expr(expr)?;
                Ok(IrStmts::Return(irexpr))
            }
            Stmts::Var { name, val } => {
                let irexpr = self.simplify_expr(val)?;
                let value = match irexpr.clone() {
                    IrExpr::IntLit(n) => ExprType::Int(n),
                    IrExpr::Variable(val) => ExprType::Str(val),
                    IrExpr::Boolean(state) => ExprType::Boolean(state),
                };
                self.vars.push(Var { name: name.to_string(), val: value });
                Ok(IrStmts::Var { name: name.to_string(), val: irexpr })
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
                Ok(IrStmts::VarRe { name: name.to_string(), val: irexpr })
            }
            // If/Elif/Else are handled by the chain-aware walker.
            Stmts::If { .. } | Stmts::Elif { .. } | Stmts::Else { .. } => {
                Err("internal: simplify_stmt_nonif received if/elif/else".into())
            }
        }
    }

    /// Lower an `if / elif* / else?` chain starting at `stmts[*idx]`.
    fn simplify_if_chain(
        &mut self,
        stmts: &[Stmts],
        idx: &mut usize,
    ) -> Result<IrStmts, Box<dyn Error>> {
        // 1) Expect an If at current idx
        let (cond_ir, then_ir) = match &stmts[*idx] {
            Stmts::If { condition, body } => {
                let cond_ir = self.simplify_expr(condition)?;
                let then_ir = self.simplify_stmt_single(body)?;
                *idx += 1;
                (cond_ir, then_ir)
            }
            _ => return Err("Expected If at start of chain".into()),
        };

        // 2) Look ahead for Elif / Else to build else_body
        let mut else_body: Option<Box<IrStmts>> = None;

        if *idx < stmts.len() {
            match &stmts[*idx] {
                Stmts::Elif { .. } => {
                    // Recurse to build nested If in else branch
                    let nested = self.simplify_elif_else_chain(stmts, idx)?;
                    else_body = Some(Box::new(nested));
                }
                Stmts::Else { body } => {
                    let else_ir = self.simplify_stmt_single(body)?;
                    else_body = Some(Box::new(else_ir));
                    *idx += 1;
                }
                _ => {}
            }
        }

        // 3) Convert condition IrExpr -> bool
        let cond_val = match cond_ir {
            IrExpr::IntLit(n) => n == 1,
            IrExpr::Boolean(b) => b,
            other => return Err(format!("Unexpected condition expr: {:?}", other).into()),
        };

        Ok(IrStmts::If {
            condition: cond_val,
            body: Box::new(then_ir),
            else_body,
        })
    }

    /// Helper to continue a chain when we already know the next token is Elif/Else.
    fn simplify_elif_else_chain(
        &mut self,
        stmts: &[Stmts],
        idx: &mut usize,
    ) -> Result<IrStmts, Box<dyn Error>> {
        match &stmts[*idx] {
            Stmts::Elif { condition, body } => {
                // Treat an elif as: else { if (cond) {body} ... }
                let cond_ir = self.simplify_expr(condition)?;
                let then_ir = self.simplify_stmt_single(body)?;
                *idx += 1;

                // chain more elif/else
                let mut else_body: Option<Box<IrStmts>> = None;
                if *idx < stmts.len() {
                    match &stmts[*idx] {
                        Stmts::Elif { .. } => {
                            let nested = self.simplify_elif_else_chain(stmts, idx)?;
                            else_body = Some(Box::new(nested));
                        }
                        Stmts::Else { body } => {
                            let else_ir = self.simplify_stmt_single(body)?;
                            else_body = Some(Box::new(else_ir));
                            *idx += 1;
                        }
                        _ => {}
                    }
                }

                let cond_val = match cond_ir {
                    IrExpr::IntLit(n) => n == 1,
                    IrExpr::Boolean(b) => b,
                    other => return Err(format!("Unexpected condition expr in elif: {:?}", other).into()),
                };

                Ok(IrStmts::If {
                    condition: cond_val,
                    body: Box::new(then_ir),
                    else_body,
                })
            }
            Stmts::Else { body } => {
                // else is the terminal: just lower its body
                let else_ir = self.simplify_stmt_single(body)?;
                *idx += 1;
                Ok(else_ir)
            }
            _ => Err("internal: expected Elif or Else in chain".into()),
        }
    }

    /// Lower a single statement (which itself can be a Scope or any non-if node).
    fn simplify_stmt_single(&mut self, stmt: &Stmts) -> Result<IrStmts, Box<dyn Error>> {
        match stmt {
            Stmts::If { .. } | Stmts::Elif { .. } | Stmts::Else { .. } => {
                // To lower a single if-statement that appears *as* a statement,
                // we need to treat it as a mini list of one or more stmts.
                // Here we emulate a one-element "list" starting at 0.
                let list = vec![stmt.clone()];
                let mut idx = 0usize;
                let lowered = self.simplify_stmts_list(&list)?;
                // Put the lowered statements into a scope to keep a single IrStmts result.
                Ok(IrStmts::Scope { body: Box::new(lowered) })
            }
            _ => self.simplify_stmt_nonif(stmt),
        }
    }

    /// Lower an entire list of AST statements, handling if/elif/else chains.
    fn simplify_stmts_list(&mut self, stmts: &[Stmts]) -> Result<Vec<IrStmts>, Box<dyn Error>> {
        let mut out = Vec::new();
        let mut idx = 0usize;
        while idx < stmts.len() {
            match &stmts[idx] {
                Stmts::If { .. } => {
                    let node = self.simplify_if_chain(stmts, &mut idx)?;
                    out.push(node);
                }
                Stmts::Elif { .. } | Stmts::Else { .. } => {
                    // These should only occur immediately after an If during chain consumption.
                    return Err("Elif/Else without preceding If".into());
                }
                _ => {
                    out.push(self.simplify_stmt_nonif(&stmts[idx])?);
                    idx += 1;
                }
            }
        }
        Ok(out)
    }

    // ---------- Legacy entry points ----------

    /// Kept for compatibility: lower a single AST stmt (non-chain-aware). Prefer `simplify_stmts_list`.
    pub fn simplify_stmt(&mut self, stmt: &Stmts) -> Result<IrStmts, Box<dyn Error>> {
        self.simplify_stmt_single(stmt)
    }

    pub fn m_simplify_ir_stmt(
        &mut self,
        irstmt: &mut IrStmts,
    ) -> Result<IrStmts, Box<dyn Error>> {
        match irstmt {
            IrStmts::If { condition, body, else_body } => {
                if *condition {
                    // collapse to then-body
                    Ok(*body.clone())
                } else if let Some(else_body) = else_body {
                    // collapse to else-body
                    Ok(*else_body.clone())
                } else {
                    // drop completely
                    Ok(IrStmts::Empty)
                }
            }
            IrStmts::Scope { body } => {
                let mut new_body = Vec::new();
                for stmt in body.iter_mut() {
                    let simplified = self.m_simplify_ir_stmt(stmt)?;
                    // Drop empties here
                    if let IrStmts::Empty = simplified {
                        // skip
                    } else {
                        new_body.push(simplified);
                    }
                }
                Ok(IrStmts::Scope { body: Box::new(new_body) })
            }
            other => Ok(other.clone()),
        }
    }

    pub fn simplify(&mut self) -> Result<IRT, Box<dyn Error>> {
        let mut irt = IRT::new();

        // Lower the whole AST program (list of Stmts) at once, so if/elif/else chains are handled.
        let lowered = self.simplify_stmts_list(&self.ast.stmts.clone())?;
        irt.irstmts = lowered;

        // Post pass: collapse true/false ifs and clean scopes.
        for irstmt in &mut irt.irstmts {
            *irstmt = self.m_simplify_ir_stmt(irstmt)?;
        }

        Ok(irt)
    }
}
