use std::fmt;
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Empty,
    Condition {
        body: Vec<Stmt>,
    },
    Fn {
        name: String,
        body: Vec<Stmt>,
    },
    If {
        condition: bool, // 0 - do, 1 - no do
        body: Vec<Stmt>,
    },
    Elif {
        condition: bool, // 0 - do, 1 - no do
        body: Vec<Stmt>,
    },
    Else {
        body: Vec<Stmt>,
    },
    ReVar {
        name: String,
        typee: String,
        val: String,
    },
    ReVarQuick {
        name: String,
        val: String,
    },
    Ret {
        val: String,
    },
    Var {
        name: String,
        typee: String,
        val: String,
    },
    VarQuick {
        name: String,
        val: String,
    },
}
impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}
impl Stmt {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, ident: usize) -> fmt::Result {
        let pad = "  ".repeat(ident);
        match self {
            Stmt::If { condition, body } => {
                writeln!(f, "{}If:", pad)?;
                writeln!(f, "{}{}Condition: {}", pad, pad, condition)?;
                for stmt in body {
                    stmt.fmt_with_indent(f, ident + 1)?;
                }
            }
            Stmt::Elif { condition, body } => {
                writeln!(f, "{}Elif:", pad)?;
                writeln!(f, "{}{}Condition: {}", pad, pad, condition)?;
                for stmt in body {
                    stmt.fmt_with_indent(f, ident + 1)?;
                }
            }
            Stmt::Else { body } => {
                writeln!(f, "{}Else:", pad)?;
                for stmt in body {
                    stmt.fmt_with_indent(f, ident + 1)?;
                }
            }
            Stmt::Condition { body } => {
                writeln!(f, "{}Condition:", pad)?;
                for stmt in body {
                    stmt.fmt_with_indent(f, ident + 1)?;
                }
            }
            Stmt::Fn { name, body } => {
                writeln!(f, "{}Fn: {}", pad, name)?;
                for stmt in body {
                    stmt.fmt_with_indent(f, ident + 1)?;
                }
            }
            Stmt::Ret { val } => {
                writeln!(f, "{}Ret: {}", pad, val)?;
            }
            Stmt::Var { name, typee, val } => {
                writeln!(
                    f,
                    "{}Var:\n{}{}name:{}\n{}{}type:{}\n{}{}val:{}",
                    pad, pad, pad, name, pad, pad, typee, pad, pad, val
                )?;
            }
            Stmt::VarQuick { name, val } => {
                writeln!(
                    f,
                    "{}Var:\n{}{}name:{}\n{}{}val:{}",
                    pad, pad, pad, name, pad, pad, val
                )?;
            }
            Stmt::ReVarQuick { name, val } => {
                writeln!(
                    f,
                    "{}Re assign quick:\n{}{}name:{}\n{}{}val:{}",
                    pad, pad, pad, name, pad, pad, val
                )?;
            }
            Stmt::ReVar { name, typee, val } => {
                writeln!(
                    f,
                    "{}Re assign:\n{}{}name:{}\n{}{}type: {}{}{}val:{}",
                    pad, pad, pad, name, pad, pad, typee, pad, pad, val
                )?;
            }
            _ => writeln!(f, " ")?,
        }
        Ok(())
    }
}
