use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::env;
use std::fs;
use std::process;

#[derive(Parser)]
#[grammar = "grammar/py.pest"]
struct PyParser;

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expr),
    Print(Expr),
    Printf(Vec<FormatString>),
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Str(String),
    Num(i64),
}

#[derive(Debug)]
pub enum FormatString {
    Interpolation(String),
    TextChunk(String),
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::assignment => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap());
            Statement::Assignment(ident, expr)
        }
        Rule::print => {
            let mut inner = pair.into_inner();
            let expr = parse_expr(inner.next().unwrap());
            Statement::Print(expr)
        }
        Rule::printf => {
            let mut inner = pair.into_inner();
            let format_str = inner.next().unwrap(); // Rule::format_string

            let parts = format_str
                .into_inner() // yields Rule::format_part
                .map(|format_part| {
                    let inner = format_part.into_inner().next().unwrap();
                    match inner.as_rule() {
                        Rule::interpolation => {
                            let expr = inner.into_inner().next().unwrap().as_str().to_string();
                            FormatString::Interpolation(expr)
                        }
                        Rule::raw_text => FormatString::TextChunk(inner.as_str().to_string()),
                        _ => panic!("Unexpected rule in format_part: {:?}", inner.as_rule()),
                    }
                })
                .collect();

            Statement::Printf(parts)
        }
        _ => panic!("Unexpected rule in parser: {:?}", pair.as_rule()),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::string => {
            let s = pair.as_str();
            let trimmed = &s[1..s.len() - 1];
            Expr::Str(trimmed.to_string())
        }
        Rule::number => Expr::Num(pair.as_str().parse().unwrap()),
        Rule::ident => Expr::Var(pair.as_str().to_string()),
        _ => panic!("Unexpected expr rule: {:?}", pair.as_rule()),
    }
}

fn parse_program(source: &str) -> Vec<Statement> {
    let parsed = PyParser::parse(Rule::code, source)
        .expect("Failed to parse")
        .next()
        .unwrap();

    parsed
        .into_inner()
        .filter_map(|pair| match pair.as_rule() {
            Rule::statement => {
                let inner = pair.into_inner().next().unwrap();
                Some(parse_statement(inner))
            }
            _ => None,
        })
        .collect()
}

fn generate_rust(stmt: &Statement) -> String {
    println!("stmt: {:?}", stmt);
    match stmt {
        Statement::Assignment(name, expr) => {
            format!("let {} = {};", name, generate_expr(expr))
        }

        Statement::Print(expr) => match expr {
            Expr::Str(s) => format!("println!(\"{}\");", s),
            _ => format!("println!(\"{{}}\", {});", generate_expr(expr)),
        },

        Statement::Printf(format_parts) => {
            let mut format_string = String::new();
            let mut args = Vec::new();

            for part in format_parts {
                match part {
                    FormatString::TextChunk(s) => {
                        let escaped = s.replace('{', "{{").replace('}', "}}");
                        format_string.push_str(" ");
                        format_string.push_str(&escaped);
                    }
                    FormatString::Interpolation(expr) => {
                        format_string.push_str("{}");
                        args.push(expr.clone());
                    }
                }
            }

            if args.is_empty() {
                format!("println!(\"{}\");", format_string)
            } else {
                format!("println!(\"{}\", {});", format_string, args.join(", "))
            }
        }
    }
}

fn generate_expr(expr: &Expr) -> String {
    match expr {
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Num(n) => n.to_string(),
        Expr::Var(v) => v.clone(),
    }
}

fn get_file_name() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/file.py>", args[0]);
        process::exit(1);
    }
    let file_path = &args[1];
    println!("Received file path: {}", file_path);
    fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading the file: {}", err);
        process::exit(1);
    })
}

fn main() {
    let code = get_file_name();
    let statements = parse_program(&code);
    let mut tabs = 0;
    let mut rust_code = String::from("fn main() {\n");
    tabs += 1;
    for stmt in &statements {
        let spaces = " ".repeat(tabs * 4);
        rust_code.push_str(&spaces);
        rust_code.push_str(&generate_rust(stmt));
        rust_code.push('\n');
    }
    rust_code.push_str("}\n");

    fs::write("output.rs", rust_code).expect("Failed to write output.rs");

    let status = std::process::Command::new("rustc")
        .arg("output.rs")
        .status()
        .expect("Failed to compile output.rs");

    if status.success() {
        println!("Compiled successfully");
    } else {
        eprintln!("Compilation failed");
    }
}
