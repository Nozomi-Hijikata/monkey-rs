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
    FuncLit {
        parameters: Vec<String>,
        body: Stmt,
    },
    // TODO:
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
    Eq,
    NotEq,
    Lt,
    Gt,
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
                    s.push_str(p);
                }
                s.push_str(") ");
                s.push_str(&format!("{:?}", body));
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
