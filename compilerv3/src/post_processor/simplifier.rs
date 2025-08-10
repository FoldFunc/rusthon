use crate::pre_procesor::ast::{Ast, Node_Fucntion};
use crate::pre_procesor::stmt::{Expr, Stmt, Term, Op};
use crate::post_processor::air::{Air, IRExpr, IRFunction, IRStmt};
#[derive(Debug)]
pub struct Simplyfier {
    ast: Ast,
    ir: Air,
}

impl Simplyfier {
    pub fn new(ast: &Ast) -> Self {
        Simplyfier {
            ast: ast.clone(),
            ir: Air::new(),
        }
    }

    pub fn simplify_binary(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { left, op, right } => {
                let left = Box::new(self.simplify_binary(*left));
                let right = Box::new(self.simplify_binary(*right));

                match (&*left, &*right) {
                    (
                        Expr::Term(Term::Int_lit { val: l }),
                        Expr::Term(Term::Int_lit { val: r }),
                    ) => {
                        let val = match op {
                            Op::Plus => l + r,
                            Op::Minus => l - r,
                            Op::Mul => l * r,
                            Op::Div => {
                                if *r != 0 {
                                    l / r
                                } else {
                                    return Expr::Binary { left, op, right };
                                }
                            }
                            Op::Equals => if l == r { 1 } else { 0 },
                            Op::MoreThan => if l > r { 1 } else { 0 },
                            Op::LessThan => if l < r { 1 } else { 0 },
                        };
                        Expr::Term(Term::Int_lit { val })
                    }
                    _ => Expr::Binary { left, op, right },
                }
            }
            other => other,
        }
    }

    pub fn simplify_expr(&self, expr: &Expr) -> IRExpr {
        let simplified = match expr {
            Expr::Binary { .. } => self.simplify_binary(expr.clone()),
            _ => expr.clone(),
        };

        match simplified {
            Expr::Term(Term::Int_lit { val }) => IRExpr::Int(val),
            Expr::Term(Term::Ident { name }) => IRExpr::Var(name),
            _ => panic!("Expression not supported for simplification: {:?}", simplified),
        }
    }

    pub fn simplify_stmt(&self, stmt: &Stmt) -> IRStmt {
        match stmt {
            Stmt::Var { name, expr } => {
                let expr_simple = self.simplify_expr(expr);
                IRStmt::Let {
                    name: name.clone(),
                    expr: expr_simple,
                }
            }
            Stmt::Return { expr } => {
                let expr_simple = self.simplify_expr(expr);
                IRStmt::Return(expr_simple)
            }
            other => panic!("Statement not supported: {:?}", other),
        }
    }

    pub fn simplify_fn(&self, function: &Node_Fucntion) -> IRFunction {
        let stmts = function
            .stmts
            .iter()
            .map(|stmt| self.simplify_stmt(stmt))
            .collect();

        IRFunction {
            name: function.name.clone(),
            stmts,
        }
    }

    pub fn simplify(&mut self) -> Air {
        self.ir.ir.clear();

        for function in &self.ast.node_funcitons {
            let ir_func = self.simplify_fn(function);
            self.ir.ir.push(ir_func);
        }
        println!("ir: \n{:?}", &self.ir);
        self.ir.clone()
    }
}
