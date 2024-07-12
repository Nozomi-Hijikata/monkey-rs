use std::fmt::{Debug, Error, Formatter};
pub enum Expr {
    Number(i32),
    Identifier(String),
    Boolean(bool),
    InfixOp(Box<Expr>, Opcode, Box<Expr>),
    PrefixOp(Opcode, Box<Expr>),
    // TODO:
    // IfExpression
    // Function Literal
    // Statement
    // Block Statement
    // Expression Statement
    // Call expression
    // String literal
    // Array Literal
    // Hash literal
    // Index Expression
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
