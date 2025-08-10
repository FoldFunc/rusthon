use std::collections::HashMap;

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

    fn resolve_ident(&self, name: &str) -> Option<i32> {
        match self.vars.get(name) {
            Some(IRExpr::Int(v)) => Some(*v),
            _ => None,
        }
    }

    fn simplify_binary(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Binary { left, op, right } => {
                let left = self.simplify_binary(*left);
                let right = self.simplify_binary(*right);

                let left = match left {
                    Expr::Term(Term::Grouped { expr }) => self.simplify_binary(*expr),
                    other => other,
                };
                let right = match right {
                    Expr::Term(Term::Grouped { expr }) => self.simplify_binary(*expr),
                    other => other,
                };

                let left_val = match &left {
                    Expr::Term(Term::Int_lit { val }) => Some(*val),
                    Expr::Term(Term::Ident { name }) => self.resolve_ident(name),
                    _ => None,
                };
                let right_val = match &right {
                    Expr::Term(Term::Int_lit { val }) => Some(*val),
                    Expr::Term(Term::Ident { name }) => self.resolve_ident(name),
                    _ => None,
                };

                if let (Some(l), Some(r)) = (left_val, right_val) {
                    let val = match op {
                        Op::Plus => l + r,
                        Op::Minus => l - r,
                        Op::Mul => l * r,
                        Op::Div => {
                            if r != 0 {
                                l / r
                            } else {
                                return Expr::Binary {
                                    left: Box::new(left),
                                    op,
                                    right: Box::new(right),
                                };
                            }
                        }
                        Op::Equals => (l == r) as i32,
                        Op::MoreThan => (l > r) as i32,
                        Op::LessThan => (l < r) as i32,
                    };
                    return Expr::Term(Term::Int_lit { val });
                }

                Expr::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                }
            }
            Expr::Term(Term::Grouped { expr }) => self.simplify_binary(*expr),
            other => other,
        }
    }

    pub fn simplify_expr(&self, expr: &Expr) -> IRExpr {
        let simplified = match expr {
            Expr::Binary { .. } => self.simplify_binary(expr.clone()),
            Expr::Term(Term::Grouped { expr }) => self.simplify_binary(*expr.clone()),
            _ => expr.clone(),
        };

        match simplified {
            Expr::Term(Term::Int_lit { val }) => IRExpr::Int(val),
            Expr::Term(Term::Ident { name }) => IRExpr::Var(name),
            Expr::Term(Term::Grouped { expr }) => self.simplify_expr(&expr),
            other => panic!("Expression not fully simplified to Term: {:?}", other),
        }
    }

    fn simplify_block(
        &mut self,
        stmts: &[Stmt],
        output: &mut Vec<IRStmt>,
        vars: &mut HashMap<String, IRExpr>,
    ) {
        for stmt in stmts {
            self.simplify_stmt(stmt, output, vars);
        }
    }

    pub fn simplify_stmt(
        &mut self,
        stmt: &Stmt,
        stmts: &mut Vec<IRStmt>,
        vars: &mut HashMap<String, IRExpr>,
    ) {
        match stmt {
            Stmt::Scope { stmts: scope_stmts } => {
                let mut inner_output = Vec::new();
                let mut inner_vars = vars.clone();

                self.simplify_block(scope_stmts, &mut inner_output, &mut inner_vars);

                stmts.push(IRStmt::Scope {
                    stmts: inner_output,
                });
            }
            Stmt::Var { name, expr } => {
                let expr_simple = self.simplify_expr(expr);
                vars.insert(name.clone(), expr_simple.clone());
                stmts.push(IRStmt::Let {
                    name: name.clone(),
                    expr: expr_simple,
                });
            }
            Stmt::Return { expr } => {
                let expr_simple = self.simplify_expr(expr);
                stmts.push(IRStmt::Return(expr_simple));
            }
            other => panic!("Statement not supported: {:?}", other),
        }
    }

    pub fn simplify_fn(&mut self, function: &Node_Fucntion) -> IRFunction {
        self.vars.clear();
        let mut stmts = Vec::new();
        let mut vars = HashMap::new();
        self.simplify_block(&function.stmts, &mut stmts, &mut vars);
        if !stmts.iter().any(|s| matches!(s, IRStmt::Return(_))) {
            for (var, val) in vars.drain() {
                stmts.push(IRStmt::Let {
                    name: var,
                    expr: val,
                });
            }
        }

        self.vars = vars;

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
