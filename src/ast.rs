use std::fmt::{Debug, Error, Formatter};
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
    PrefixOp(Opcode, Box<Expr>),
    // TODO: ifexpr, Idenfier, Boolean, Function literal, Call expression, String Literal, Array
    // Literal, Hash Literal, Index Expression, Hash Index Expression
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
            Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
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
