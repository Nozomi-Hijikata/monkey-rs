use crate::{environment::Environment, object::Object};
use std::fmt::{Debug, Error, Formatter};

pub trait Node {
    fn eval(&self, env: &mut Environment) -> Box<dyn Object>;
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<Stmt>>,
}

#[derive(Clone)]
pub enum Stmt {
    Let { name: String, value: Box<Expr> },
    Return { return_value: Box<Expr> },
    Expr { expression: Box<Expr> },
    Block { statements: Vec<Box<Stmt>> },
}

#[derive(Clone)]
pub enum Expr {
    Number(i64),
    Identifier(String),
    Boolean(bool),
    InfixOp {
        left: Box<Expr>,
        operator: Opcode,
        right: Box<Expr>,
    },
    PrefixOp {
        operator: Opcode,
        right: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        consequence: Box<Stmt>,
        alternative: Option<Box<Stmt>>,
    },
    FuncLit {
        parameters: Vec<Box<Expr>>,
        body: Box<Stmt>,
    },
    Call {
        function: String,
        arguments: Vec<Box<Expr>>,
    },
    // TODO:
    // String literal
    // Array Literal
    // Array Index Expression
    // Hash literal
    // Hash Index Expression
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Bang,
    Eq,
    NotEq,
    Lt,
    Gt,
}

impl Debug for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Stmt::*;
        match *self {
            Let {
                ref name,
                ref value,
            } => write!(fmt, "let {} = {:?}", name, value),
            Return { ref return_value } => write!(fmt, "return {:?}", return_value),
            Expr { ref expression } => write!(fmt, "{:?}", expression),
            Block { ref statements } => {
                writeln!(fmt, "{{")?;
                for stmt in statements {
                    writeln!(fmt, "  {:?}", stmt)?;
                }
                write!(fmt, "}}")
            }
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Identifier(ref s) => write!(fmt, "{}", s),
            Boolean(b) => write!(fmt, "{:?}", b),
            InfixOp {
                ref left,
                ref operator,
                ref right,
            } => write!(fmt, "({:?} {:?} {:?})", left, operator, right),
            PrefixOp {
                ref operator,
                ref right,
            } => write!(fmt, "({:?}{:?})", operator, right),
            If {
                ref condition,
                ref consequence,
                ref alternative,
            } => {
                let mut s = String::new();
                s.push_str(&format!("if ({:?}) ", condition));
                s.push_str(&format!("{:?}", consequence));
                if let Some(ref alt) = *alternative {
                    s.push_str(&format!(" else {:?}", alt));
                }
                write!(fmt, "{}", s)
            }
            FuncLit {
                ref parameters,
                ref body,
            } => {
                let mut s = String::new();
                s.push_str("fn(");
                for (i, p) in parameters.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{:?}", p));
                }
                s.push_str(") ");
                s.push_str(&format!("{:?}", body));
                write!(fmt, "{}", s)
            }
            Call {
                ref function,
                ref arguments,
            } => {
                let mut s = String::new();
                s.push_str(&format!("{}(", function));
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{:?}", arg));
                }
                s.push_str(")");
                write!(fmt, "{}", s)
            }
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Bang => write!(fmt, "!"),
            Eq => write!(fmt, "=="),
            NotEq => write!(fmt, "!="),
            Lt => write!(fmt, "<"),
            Gt => write!(fmt, ">"),
        }
    }
}

impl Opcode {
    pub fn as_str(&self) -> &str {
        use self::Opcode::*;
        match *self {
            Mul => "*",
            Div => "/",
            Add => "+",
            Sub => "-",
            Bang => "!",
            Eq => "==",
            NotEq => "!=",
            Lt => "<",
            Gt => ">",
        }
    }
}

impl ToString for Stmt {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
