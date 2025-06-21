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
    MathAssigment(String, MathStmt),
    Assignment(String, Expr),
    Print(Expr),
    Printf(Vec<FormatString>),
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Str(String),
    Num(i64),
    Input(Box<Expr>), // Represents input(...)
}
#[derive(Debug)]
pub enum MathStmt {
    MathStmt(String),
}
#[derive(Debug)]
pub enum FormatString {
    Interpolation(String),
    TextChunk(String),
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::assigment_math => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = parse_math_expr(inner.next().unwrap());
            println!("ident: {:?}", ident);
            println!("expr: {:?}", expr);
            Statement::MathAssigment(ident, expr)
        }
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
fn parse_math_expr(pair: Pair<Rule>) -> MathStmt {
    match pair.as_rule() {
        Rule::math_stmt => {
            let s = pair.as_str();
            println!("");
            println!("s: {}", s);
            let trimmed = &s[0..s.len() - 0];
            println!("");
            println!("trimmed: {:?}", trimmed);
            return MathStmt::MathStmt(trimmed.to_string());
        }
        _ => unreachable!("Invalid"),
    };
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
        Rule::input_call => {
            let _ = prepend_to_file("output.rs", "use input_macro::input;");
            let mut inner = pair.into_inner();
            inner.next(); // skip "input"
            let prompt = parse_expr(inner.next().unwrap());
            Expr::Input(Box::new(prompt))
        }
        _ => panic!("Unexpected expr rule: {:?}", pair.as_rule()),
    }
}

fn parse_program(source: &str) -> (Vec<Statement>, bool) {
    let parsed = PyParser::parse(Rule::code, source)
        .expect("Failed to parse")
        .next()
        .unwrap();

    let mut has_input = false;

    let statements = parsed
        .into_inner()
        .filter_map(|pair| match pair.as_rule() {
            Rule::statement => {
                let inner = pair.into_inner().next().unwrap();
                println!("inner: {:?}", inner);
                let stmt = parse_statement(inner);
                if statement_has_input(&stmt) {
                    has_input = true;
                }
                Some(stmt)
            }
            _ => None,
        })
        .collect();

    (statements, has_input)
}

fn generate_rust(stmt: &Statement) -> String {
    match stmt {
        Statement::MathAssigment(name, expr) => {
            format!("{} = {};", name, generate_math_expr(expr))
        }
        Statement::Assignment(name, expr) => {
            format!("let mut {} = {};", name, generate_expr(expr))
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
fn generate_math_expr(expr: &MathStmt) -> String {
    match expr {
        MathStmt::MathStmt(s) => format!("{}", s),
    }
}
fn generate_expr(expr: &Expr) -> String {
    match expr {
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Num(n) => n.to_string(),
        Expr::Var(v) => v.clone(),
        Expr::Input(prompt_expr) => {
            format!("input!({})", generate_expr(prompt_expr))
        }
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
fn prepend_to_file(path: &str, text: &str) -> std::io::Result<()> {
    // Read existing content
    let original = fs::read_to_string(path)?;

    // Combine new text + old content
    let new_content = format!("{}{}", text, original);

    // Write it back, overwriting the file
    fs::write(path, new_content)?;

    Ok(())
}
fn statement_has_input(stmt: &Statement) -> bool {
    match stmt {
        Statement::Assignment(_, expr) => expr_has_input(expr),
        Statement::Print(expr) => expr_has_input(expr),
        Statement::Printf(parts) => parts.iter().any(|part| match part {
            FormatString::Interpolation(_expr_str) => false,
            FormatString::TextChunk(_) => false,
        }),
        Statement::MathAssigment(_, _) => false,
    }
}

fn expr_has_input(expr: &Expr) -> bool {
    match expr {
        Expr::Input(_) => true,
        _ => false,
    }
}

fn main() {
    let _ = std::process::Command::new("cargo")
        .arg("new")
        .arg("output")
        .status()
        .expect("Failed to create a folder");
    let code = get_file_name();
    let (statements, has_input) = parse_program(&code);
    let mut tabs = 0;
    let mut rust_code = String::from("");
    let mut toml_code = String::from("");
    if has_input {
        toml_code += &String::from(
            "[package]\nname = \"output\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\ninput_macro_fold_func = \"0.1.0\"",
        );
        rust_code += &String::from("use input_macro_fold_func::input;\nfn main() {\n")
    } else {
        toml_code += &String::from(
            "[package]\nname = \"output\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]",
        );
        rust_code += &String::from("fn main() {\n");
    }
    tabs += 1;
    for stmt in &statements {
        let spaces = " ".repeat(tabs * 4);
        rust_code.push_str(&spaces);
        rust_code.push_str(&generate_rust(stmt));
        rust_code.push('\n');
    }
    rust_code.push_str("}\n");

    fs::write("output/src/main.rs", rust_code).expect("Failed to write output.rs");
    fs::write("output/Cargo.toml", toml_code).expect("Failed to write Cargo.toml");
    let _ = std::process::Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        .arg("output/Cargo.toml")
        .status()
        .expect("Failed to go to the output directory");
}
