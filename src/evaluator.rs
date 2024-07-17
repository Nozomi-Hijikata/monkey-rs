use crate::ast::{Expr, Node, Program, Stmt};
use crate::object::{Integer, Null, Object};

pub fn eval_program(program: &Program) -> Box<dyn Object> {
    eval(program)
}

fn eval(node: &dyn Node) -> Box<dyn Object> {
    node.eval()
}

impl Node for Program {
    fn eval(&self) -> Box<dyn Object> {
        let mut result: Box<dyn Object> = Box::new(Null);
        for stmt in &self.statements {
            result = eval(stmt.as_ref());
        }
        result
    }
}

impl Node for Stmt {
    fn eval(&self) -> Box<dyn Object> {
        match self {
            Stmt::Let {
                ref name,
                ref value,
            } => {
                println!("Let statement: {} = {:?}", name, value);
                value.eval()
            }
            Stmt::Return { ref return_value } => {
                println!("Return statement: {:?}", return_value);
                return_value.eval()
            }
            Stmt::Expr { ref expression } => {
                println!("Expression statement: {:?}", expression);
                expression.eval()
            }
            Stmt::Block { ref statements } => {
                println!("Block statement: {:?}", statements);
                let mut result: Box<dyn Object> = Box::new(Null);
                for stmt in statements {
                    result = eval(stmt.as_ref());
                }
                result
            }
        }
    }
}

impl Node for Expr {
    fn eval(&self) -> Box<dyn Object> {
        match self {
            Expr::Number(n) => Box::new(Integer { value: *n }),
            Expr::Identifier(ident) => {
                println!("Identifier expression: {}", ident);
                Box::new(Null)
            }
            Expr::Boolean(b) => {
                println!("Boolean expression: {:?}", b);
                Box::new(Null)
            }
            Expr::InfixOp {
                ref left,
                ref operator,
                ref right,
            } => {
                println!("Infix expression: {:?} {:?} {:?}", left, operator, right);
                Box::new(Null)
            }
            Expr::PrefixOp {
                ref operator,
                ref right,
            } => {
                println!("Prefix expression: {:?}{:?}", operator, right);
                Box::new(Null)
            }
            Expr::If {
                ref condition,
                ref consequence,
                ref alternative,
            } => {
                println!(
                    "If expression: {:?} {:?} {:?}",
                    condition, consequence, alternative
                );
                Box::new(Null)
            }
            Expr::FuncLit {
                ref parameters,
                ref body,
            } => {
                println!("Function literal expression: {:?} {:?}", parameters, body);
                Box::new(Null)
            }
            Expr::Call {
                ref function,
                ref arguments,
            } => {
                println!("Call expression: {:?} {:?}", function, arguments);
                Box::new(Null)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    fn assert_is_integer(object: &Box<dyn Object>, expected_value: i64) {
        if let Some(integer) = object.as_any().downcast_ref::<Integer>() {
            assert_eq!(integer.value, expected_value);
        } else {
            panic!("Expected Integer object");
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let program = parse_program("5;").unwrap();
        let results = eval_program(&program);
        assert_is_integer(&results, 5);

        let program = parse_program("10;").unwrap();
        let results = eval_program(&program);
        assert_is_integer(&results, 10);
    }
}
