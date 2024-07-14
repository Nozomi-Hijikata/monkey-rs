use std::fmt::{Debug, Error, Formatter};

pub enum Stmt {
    LetStmt { name: String, value: Box<Expr> },
    ReturnStmt { return_value: Box<Expr> },
    ExprStmt { expression: Box<Expr> },
    BlockStmt { statements: Vec<Stmt> },
}

pub enum Expr {
    Number(i32),
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
    IfExpr {
        condition: Box<Expr>,
        consequence: Stmt,
        alternative: Option<Stmt>,
    },
    // TODO:
    // Function Literal
    // Call expression
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
}

impl Debug for Stmt {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Stmt::*;
        match *self {
            LetStmt {
                ref name,
                ref value,
            } => write!(fmt, "let {} = {:?}", name, value),
            ReturnStmt { ref return_value } => write!(fmt, "return {:?}", return_value),
            ExprStmt { ref expression } => write!(fmt, "{:?}", expression),
            BlockStmt { ref statements } => {
                write!(fmt, "{{\n")?;
                for stmt in statements {
                    write!(fmt, "  {:?}\n", stmt)?;
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
            IfExpr {
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
        }
    }
}
