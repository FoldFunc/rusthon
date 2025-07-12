use crate::parser_ast::parser::{Stmt, Expr};
use std::{collections::HashMap, error::{self, Error}};
enum ErrorType {
    Error000,
    Error001,
    Error002,
    Error003,
}
fn store_var_names(ast: &[Stmt]) -> HashMap<&String, &Expr> {
    let mut vars = HashMap::new();

    for stmt in ast {
        if let Stmt::VarDecl { name, value } = stmt {
            vars.insert(name, value);
        }
    }
    vars
}

fn store_commands(ast: &[Stmt]) -> Vec<&Stmt> {
    let mut commands = Vec::new();

    for stmt in ast {
        match stmt {
            Stmt::Return(_) => {
                commands.push(stmt);
            }
            _ => {}
        }
    }

    commands
}

fn print_vars(vars: &HashMap<&String, &Expr>) {
    for (k, v) in vars {
        match v {
            Expr::Number(n) => println!("name {:?}: value {}", k, n),
            _ => println!("name {:?}: value (not a number)", k),
        }
    }
}
fn print_commands(commands: &Vec<&Stmt>) {
    for stmt in commands {
        match stmt {
            Stmt::Return(expr) => match expr {
                Expr::Number(n) => println!("Return statement with value: {}", n),
                Expr::Ident(s) => println!("Return statement with var: {}", s),
                Expr::Binary { left, op, right } => println!("This abomination: {:?}\t{:?}\t{:?}", left, op, right),
            },
            _ => println!("Unhandled stmt"),
        }
    }
}
fn valid_return_variable(vars: &HashMap<&String, &Expr>, commands: &Vec<&Stmt>) -> Vec<ErrorType> {
    let mut errors: Vec<ErrorType> = Vec::new();
    for stmt in commands {
        match stmt {
            Stmt::Return(expr) => match expr {
                Expr::Number(n) => {
                    if *n > 255 || *n < 0 {
                        errors.push(ErrorType::Error001);
                    }
                }
                Expr::Ident(s) => {
                    let mut ok = false;
                    for (k, _v) in vars.clone() {
                        if s.to_string() == k.to_string() {
                            ok = true;
                        }
                    }
                    if !ok {
                        errors.push(ErrorType::Error002);
                    }
                }
                Expr::Binary { left: _, op: _, right: _ } => {
                        errors.push(ErrorType::Error003);
                }
            }
            _ => println!("No need to wory bout that one mate"),
        }
    }
    if errors.len() == 0 {
        errors.push(ErrorType::Error000);
    }
    return errors;
}
fn print_errors_return_stmt(errors: &Vec<ErrorType>) {
    for error in errors {
        match error {
            ErrorType::Error001 => {
                eprintln!("\x1b[31mTo big or to small value of exit code in return statement.\x1b[0m");
                std::process::exit(1);
            }
            ErrorType::Error002 => {
                eprintln!("\x1b[31mInvalid variable name in return statement\x1b[0m");
                std::process::exit(2);
            }
            ErrorType::Error003 => {
                eprintln!("\x1b[31mFor now imposible to add error handling to this crap.\x1b[0m");
                std::process::exit(3);
            }
            ErrorType::Error000 => {
                println!("\x1b[32mEverything good.\x1b[0m");
            }
        }
    }

}
pub fn find_errors(ast: &[Stmt]) {
    println!("----- Errors -----");
    let vars = store_var_names(ast);
    let commands = store_commands(ast);
    print_vars(&vars);
    print_commands(&commands);
    let errors_from_return_wrong = valid_return_variable(&vars, &commands);
    let _ = print_errors_return_stmt(&errors_from_return_wrong);

}

