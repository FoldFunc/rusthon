use crate::parser_ast::parser::{Stmt, Expr};
use std::{collections::HashMap, error::{self, Error}};

enum ErrorType {
    Error000,
    Error001,
    Error002,
    Error003,
    Error004,
    Error005,
    Error006,
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
            Stmt::VarRedecl { .. } => {
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
                Expr::Binary { left, op, right } => {
                    println!("This abomination: {:?}\t{:?}\t{:?}", left, op, right)
                }
            },
            _ => println!("Unhandled stmt"),
        }
    }
}

fn validate_expr(expr: &Expr, vars: &HashMap<&String, &Expr>, is_return: bool) -> Option<ErrorType> {
    match expr {
        Expr::Number(n) => {
            if is_return && (*n < 0 || *n > 255) {
                return Some(ErrorType::Error001);
            }
        }
        Expr::Ident(s) => {
            let found = vars.keys().any(|k| *k == s);
            if !found {
                return Some(if is_return { ErrorType::Error002 } else { ErrorType::Error005 });
            }
        }
        Expr::Binary { left, op: _, right } => {
            if let Some(e) = validate_expr(left, vars, is_return) {
                return Some(e);
            }
            if let Some(e) = validate_expr(right, vars, is_return) {
                return Some(e);
            }
        }
        _ => {}
    }
    None
}

fn valid_return_variable(vars: &HashMap<&String, &Expr>, commands: &Vec<&Stmt>) -> Vec<ErrorType> {
    let mut errors: Vec<ErrorType> = Vec::new();
    for stmt in commands {
        if let Stmt::Return(expr) = stmt {
            if let Some(e) = validate_expr(expr, vars, true) {
                errors.push(e);
            }
        }
    }
    if errors.is_empty() {
        errors.push(ErrorType::Error000);
    }
    errors
}

fn valid_redecler_variable(vars: &HashMap<&String, &Expr>, commands: &Vec<&Stmt>) -> Vec<ErrorType> {
    let mut errors: Vec<ErrorType> = Vec::new();
    for stmt in commands {
        if let Stmt::VarRedecl { value, .. } = stmt {
            if let Some(e) = validate_expr(value, vars, false) {
                errors.push(e);
            }
        }
    }
    if errors.is_empty() {
        errors.push(ErrorType::Error000);
    }
    errors
}

fn print_errors_return_stmt(errors: &Vec<ErrorType>) {
    for error in errors {
        match error {
            ErrorType::Error001 => {
                eprintln!("\x1b[31mToo big or too small value of exit code in return statement.\x1b[0m");
                std::process::exit(1);
            }
            ErrorType::Error002 => {
                eprintln!("\x1b[31mInvalid variable name in return statement\x1b[0m");
                std::process::exit(2);
            }
            ErrorType::Error003 => {
                eprintln!("\x1b[31mFor now impossible to add error handling to this crap.\x1b[0m");
                std::process::exit(3);
            }
            ErrorType::Error000 => {
                println!("\x1b[32mEverything good.\x1b[0m");
            }
            _ => println!("No need to worry bout that."),
        }
    }
}

fn print_errors_redcl_stmt(errors: &Vec<ErrorType>) {
    for error in errors {
        match error {
            ErrorType::Error004 => {
                eprintln!("\x1b[31mToo big or too small value of variable.\x1b[0m");
                std::process::exit(1);
            }
            ErrorType::Error005 => {
                eprintln!("\x1b[31mInvalid variable name.\x1b[0m");
                std::process::exit(2);
            }
            ErrorType::Error006 => {
                eprintln!("\x1b[31mFor now impossible to add error handling to this crap.\x1b[0m");
                std::process::exit(3);
            }
            ErrorType::Error000 => {
                println!("\x1b[32mEverything good.\x1b[0m");
            }
            _ => println!("No need to worry bout that."),
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
    let errors_from_var_redecl_wrong = valid_redecler_variable(&vars, &commands);
    let _ = print_errors_return_stmt(&errors_from_return_wrong);
    let _ = print_errors_redcl_stmt(&errors_from_var_redecl_wrong);
}

