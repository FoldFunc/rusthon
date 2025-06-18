use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/py.pest"]
struct PyParser;

#[derive(Debug)]
pub enum Statement {
    Assignment(String, Expr),
    Print(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Var(String),
    Str(String),
    Num(i64),
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
        _ => panic!("Unexpected rule in parser: {:?}", pair.as_rule()),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::string => {
            let s = pair.as_str();
            let trimmed = &s[1..s.len() - 1]; // Remove quotes
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
        .filter_map(|pair| {
            match pair.as_rule() {
                Rule::statement => {
                    let inner = pair.into_inner().next().unwrap();
                    Some(parse_statement(inner))
                }
                _ => None, // skip EOI etc.
            }
        })
        .collect()
}

fn generate_rust(stmt: &Statement) -> String {
    match stmt {
        Statement::Assignment(name, expr) => {
            format!("let {} = {};", name, generate_expr(expr))
        }
        Statement::Print(expr) => match expr {
            Expr::Str(s) => format!("println!(\"{}\");", s),
            _ => format!("println!(\"{{}}\", {});", generate_expr(expr)),
        },
    }
}

fn generate_expr(expr: &Expr) -> String {
    match expr {
        Expr::Str(s) => format!("\"{}\"", s),
        Expr::Num(n) => n.to_string(),
        Expr::Var(v) => v.clone(),
    }
}

fn main() {
    let code = r#"
        x = 42
        name = "Oh hell yeah it works"
        print("Hello dear")
        print(name)
        print(x)
    "#;

    let statements = parse_program(code);

    let mut rust_code = String::from("fn main() {\n");
    for stmt in &statements {
        rust_code.push_str("    ");
        rust_code.push_str(&generate_rust(stmt));
        rust_code.push('\n');
    }
    rust_code.push_str("}\n");

    std::fs::write("output.rs", rust_code).expect("Failed to write output.rs");

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
