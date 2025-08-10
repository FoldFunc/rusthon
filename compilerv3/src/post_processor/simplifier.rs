use std::collections::HashMap;

// Adjust these imports according to your project structure
use crate::post_processor::air::{Air, IRExpr, IRFunction, IRStmt};
use crate::pre_procesor::ast::{Ast, Node_Fucntion};
use crate::pre_procesor::stmt::{Expr, Op, Stmt, Term};

#[derive(Debug)]
pub struct Simplifier {
    ast: Ast,
    ir: Air,
    vars: HashMap<String, IRExpr>,
}

impl Simplifier {
    pub fn new(ast: &Ast) -> Self {
        Simplifier {
            ast: ast.clone(),
            ir: Air::new(),
            vars: HashMap::new(),
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
                            Op::Equals => {
                                if l == r {
                                    1
                                } else {
                                    0
                                }
                            }
                            Op::MoreThan => {
                                if l > r {
                                    1
                                } else {
                                    0
                                }
                            }
                            Op::LessThan => {
                                if l < r {
                                    1
                                } else {
                                    0
                                }
                            }
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
            _ => panic!(
                "Expression not supported for simplification: {:?}",
                simplified
            ),
        }
    }

    // Now takes &mut self and a mutable reference to the vector of IRStmt
    pub fn simplify_stmt(&mut self, stmt: &Stmt, stmts: &mut Vec<IRStmt>) {
        match stmt {
            Stmt::Var { name, expr } => {
                let expr_simple = self.simplify_expr(expr);
                // Update or insert the variable's latest value
                self.vars.insert(name.clone(), expr_simple);
                // Don't push Let statements here — flush later
            }
            Stmt::Return { expr } => {
                // Flush all pending variable lets before return
                for (var, val) in self.vars.drain() {
                    stmts.push(IRStmt::Let {
                        name: var,
                        expr: val,
                    });
                }
                let expr_simple = self.simplify_expr(expr);
                stmts.push(IRStmt::Return(expr_simple));
            }
            other => panic!("Statement not supported: {:?}", other),
        }
    }

    pub fn simplify_fn(&mut self, function: &Node_Fucntion) -> IRFunction {
        self.vars.clear();
        let mut stmts = Vec::new();

        for stmt in &function.stmts {
            self.simplify_stmt(stmt, &mut stmts);
        }

        // Flush any remaining vars if no Return was found
        if !stmts.iter().any(|s| matches!(s, IRStmt::Return(_))) {
            for (var, val) in self.vars.drain() {
                stmts.push(IRStmt::Let {
                    name: var,
                    expr: val,
                });
            }
        }

        IRFunction {
            name: function.name.clone(),
            stmts,
        }
    }

    pub fn simplify(&mut self) -> Air {
        self.ir.ir.clear();
        let ast_nodes = &self.ast.node_funcitons.clone();
        for function in ast_nodes {
            let ir_func = self.simplify_fn(function);
            self.ir.ir.push(ir_func);
        }

        println!("Simplified IR:\n{:?}", &self.ir);
        self.ir.clone()
    }
}
