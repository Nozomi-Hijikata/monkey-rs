use std::fmt::{Debug, Error, Formatter};

pub enum Stmt {
    LetStmt(Box<Expr>, Box<Expr>),
    // ReturnStmt(Expr),
    // ExprStmt(Expr),
    // TODO:
    // Return Statement
    // Expression Statement
    // Block Statement
}

pub enum Expr {
    Number(i32),
    Identifier(String),
    Boolean(bool),
    InfixOp(Box<Expr>, Opcode, Box<Expr>),
    PrefixOp(Opcode, Box<Expr>),
    // TODO:
    // IfExpression
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
            LetStmt(ref s, ref e) => write!(fmt, "let {:?} = {:?}", s, e),
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
            InfixOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            PrefixOp(op, ref e) => write!(fmt, "({:?}{:?})", op, e),
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
