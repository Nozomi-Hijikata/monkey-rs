use crate::ast::{Expr, Node, Opcode, Program, Stmt};
use crate::object::{Boolean, Integer, Null, Object};

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
            Expr::Boolean(b) => eval_native_boolean(b),
            Expr::InfixOp {
                ref left,
                ref operator,
                ref right,
            } => {
                let left_value = eval(left.as_ref());
                let right_value = eval(right.as_ref());
                eval_infix_expression(operator, &left_value, &right_value)
            }
            Expr::PrefixOp {
                ref operator,
                ref right,
            } => {
                let right_value = eval(right.as_ref());
                eval_prefix_expression(operator, &right_value)
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

fn eval_infix_expression(
    operator: &Opcode,
    left: &Box<dyn Object>,
    right: &Box<dyn Object>,
) -> Box<dyn Object> {
    if let (Some(left_int), Some(right_int)) = (
        left.as_any().downcast_ref::<Integer>(),
        right.as_any().downcast_ref::<Integer>(),
    ) {
        eval_integer_infix_expression(operator, left_int, right_int)
    } else {
        match operator {
            Opcode::Eq | Opcode::NotEq => eval_boolean_infix_expression(operator, left, right),
            _ => Box::new(Null),
        }
    }
}

fn eval_integer_infix_expression(
    operator: &Opcode,
    left: &Integer,
    right: &Integer,
) -> Box<dyn Object> {
    match operator {
        Opcode::Add => Box::new(Integer {
            value: left.value + right.value,
        }),
        Opcode::Sub => Box::new(Integer {
            value: left.value - right.value,
        }),
        Opcode::Mul => Box::new(Integer {
            value: left.value * right.value,
        }),
        Opcode::Div => Box::new(Integer {
            value: left.value / right.value,
        }),
        Opcode::Eq => eval_native_boolean(&(left.value == right.value)),
        Opcode::NotEq => eval_native_boolean(&(left.value != right.value)),
        Opcode::Lt => eval_native_boolean(&(left.value < right.value)),
        Opcode::Gt => eval_native_boolean(&(left.value > right.value)),
        _ => Box::new(Null),
    }
}

fn eval_boolean_infix_expression(
    operator: &Opcode,
    left: &Box<dyn Object>,
    right: &Box<dyn Object>,
) -> Box<dyn Object> {
    match (
        left.as_any().downcast_ref::<Boolean>(),
        right.as_any().downcast_ref::<Boolean>(),
    ) {
        (Some(left_bool), Some(right_bool)) => match operator {
            Opcode::Eq => eval_native_boolean(&(left_bool.value == right_bool.value)),
            Opcode::NotEq => eval_native_boolean(&(left_bool.value != right_bool.value)),
            _ => Box::new(Null),
        },
        _ => Box::new(Null),
    }
}

fn eval_prefix_expression(operator: &Opcode, right: &Box<dyn Object>) -> Box<dyn Object> {
    match operator {
        Opcode::Bang => eval_bang_operator_expression(right),
        Opcode::Sub => match right.as_any().downcast_ref::<Integer>() {
            Some(integer) => Box::new(Integer {
                value: -integer.value,
            }),
            _ => Box::new(Null),
        },
        _ => Box::new(Null),
    }
}

fn eval_bang_operator_expression(right: &Box<dyn Object>) -> Box<dyn Object> {
    match right.as_any().downcast_ref::<Boolean>() {
        Some(boolean) => {
            if boolean.value {
                Box::new(Boolean { value: false })
            } else {
                Box::new(Boolean { value: true })
            }
        }
        _ => Box::new(Boolean { value: false }),
    }
}

// TODO: TRUE, FALSE, NULLは使い回しできるようにする
fn eval_native_boolean(input: &bool) -> Box<dyn Object> {
    if *input {
        Box::new(Boolean { value: true })
    } else {
        Box::new(Boolean { value: false })
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
        let tests = vec![
            ("5;", 5),
            ("10;", 10),
            ("5 + 5 + 5 + 5 - 10;", 10),
            ("2 * 2 * 2 * 2 * 2;", 32),
            ("5 * 2 + 10;", 20),
            ("5 + 2 * 10;", 25),
            ("5 * (2 + 10);", 60),
            ("-5;", -5),
            ("-10;", -10),
            ("-50 + 100 + -50;", 0),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10;", 50),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let results = eval_program(&program);
            assert_is_integer(&results, expected);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true;", true),
            ("false;", false),
            ("1 < 2;", true),
            ("1 > 2;", false),
            ("1 < 1;", false),
            ("1 > 1;", false),
            ("1 == 1;", true),
            ("1 != 1;", false),
            ("1 == 2;", false),
            ("1 != 2;", true),
            ("true == true;", true),
            ("false == false;", true),
            ("true == false;", false),
            ("true != false;", true),
            ("false != true;", true),
            ("(1 < 2) == true;", true),
            ("(1 < 2) == false;", false),
            ("(1 > 2) == true;", false),
            ("(1 > 2) == false;", true),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let results = eval_program(&program);
            assert_eq!(results.inspect(), expected.to_string());
        }
    }

    #[test]
    fn test_eval_bang_operator() {
        let tests = vec![
            ("!true;", false),
            ("!false;", true),
            ("!5;", false),
            ("!!true;", true),
            ("!!false;", false),
            ("!!5;", true),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let results = eval_program(&program);
            assert_eq!(results.inspect(), expected.to_string());
        }
    }
}
