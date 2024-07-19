use crate::ast::{Expr, Node, Opcode, Program, Stmt};
use crate::object::{Boolean, Integer, Null, ObjectRef};
use crate::{box_it, downcast_ref};

pub fn eval_program(program: &Program) -> Result<ObjectRef, String> {
    Ok(program.eval())
}

fn eval(node: &dyn Node) -> ObjectRef {
    node.eval()
}

impl Node for Program {
    fn eval(&self) -> ObjectRef {
        let mut result: ObjectRef = box_it!(Null);
        for stmt in &self.statements {
            result = eval(stmt.as_ref());
        }
        result
    }
}

impl Node for Stmt {
    fn eval(&self) -> ObjectRef {
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
            Stmt::Expr { ref expression } => eval(expression.as_ref()),
            Stmt::Block { ref statements } => {
                let mut result: ObjectRef = box_it!(Null);
                for stmt in statements {
                    result = eval(stmt.as_ref());
                }
                result
            }
        }
    }
}

impl Node for Expr {
    fn eval(&self) -> ObjectRef {
        match self {
            Expr::Number(n) => box_it!(Integer { value: *n }),
            Expr::Identifier(ident) => {
                println!("Identifier expression: {}", ident);
                box_it!(Null)
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
                let condition_value = eval(condition.as_ref());
                if is_truthy(&condition_value) {
                    eval(consequence.as_ref())
                } else {
                    match alternative {
                        Some(alt) => eval(alt.as_ref()),
                        None => box_it!(Null),
                    }
                }
            }
            Expr::FuncLit {
                ref parameters,
                ref body,
            } => {
                println!("Function literal expression: {:?} {:?}", parameters, body);
                box_it!(Null)
            }
            Expr::Call {
                ref function,
                ref arguments,
            } => {
                println!("Call expression: {:?} {:?}", function, arguments);
                box_it!(Null)
            }
        }
    }
}

fn eval_infix_expression(operator: &Opcode, left: &ObjectRef, right: &ObjectRef) -> ObjectRef {
    if let (Some(left_int), Some(right_int)) =
        (downcast_ref!(left, Integer), downcast_ref!(right, Integer))
    {
        eval_integer_infix_expression(operator, left_int, right_int)
    } else {
        match operator {
            Opcode::Eq | Opcode::NotEq => eval_boolean_infix_expression(operator, left, right),
            _ => box_it!(Null),
        }
    }
}

fn eval_integer_infix_expression(operator: &Opcode, left: &Integer, right: &Integer) -> ObjectRef {
    match operator {
        Opcode::Add => box_it!(Integer {
            value: left.value + right.value,
        }),
        Opcode::Sub => box_it!(Integer {
            value: left.value - right.value,
        }),
        Opcode::Mul => box_it!(Integer {
            value: left.value * right.value,
        }),
        Opcode::Div => box_it!(Integer {
            value: left.value / right.value,
        }),
        Opcode::Eq => eval_native_boolean(&(left.value == right.value)),
        Opcode::NotEq => eval_native_boolean(&(left.value != right.value)),
        Opcode::Lt => eval_native_boolean(&(left.value < right.value)),
        Opcode::Gt => eval_native_boolean(&(left.value > right.value)),
        _ => box_it!(Null),
    }
}

fn eval_boolean_infix_expression(
    operator: &Opcode,
    left: &ObjectRef,
    right: &ObjectRef,
) -> ObjectRef {
    if let (Some(left_bool), Some(right_bool)) =
        (downcast_ref!(left, Boolean), downcast_ref!(right, Boolean))
    {
        match operator {
            Opcode::Eq => eval_native_boolean(&(left_bool.value == right_bool.value)),
            Opcode::NotEq => eval_native_boolean(&(left_bool.value != right_bool.value)),
            _ => box_it!(Null),
        }
    } else {
        box_it!(Null)
    }
}

fn eval_prefix_expression(operator: &Opcode, right: &ObjectRef) -> ObjectRef {
    match operator {
        Opcode::Bang => eval_bang_operator_expression(right),
        Opcode::Sub => match downcast_ref!(right, Integer) {
            Some(integer) => box_it!(Integer {
                value: -integer.value,
            }),
            _ => box_it!(Null),
        },
        _ => box_it!(Null),
    }
}

fn eval_bang_operator_expression(right: &ObjectRef) -> ObjectRef {
    match downcast_ref!(right, Boolean) {
        Some(boolean) => {
            if boolean.value {
                box_it!(Boolean { value: false })
            } else {
                box_it!(Boolean { value: true })
            }
        }
        _ => box_it!(Boolean { value: false }),
    }
}

fn is_truthy(object: &ObjectRef) -> bool {
    if let Some(boolean) = downcast_ref!(object, Boolean) {
        return boolean.value;
    }
    match downcast_ref!(object, Null) {
        Some(_) => false,
        _ => true,
    }
}

// TODO: TRUE, FALSE, NULLは使い回しできるようにする
fn eval_native_boolean(input: &bool) -> ObjectRef {
    if *input {
        box_it!(Boolean { value: true })
    } else {
        box_it!(Boolean { value: false })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_program;

    fn assert_is_integer(object: &ObjectRef, expected_value: i64) {
        if let Some(integer) = downcast_ref!(object, Integer) {
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
            let results = eval_program(&program).unwrap();
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
            ("5 + 10 > 4 + 5;", true),
            ("5 + 10 < 4 + 5;", false),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let results = eval_program(&program).unwrap();
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
            let results = eval_program(&program).unwrap();
            assert_eq!(results.inspect(), expected.to_string());
        }
    }

    #[test]
    fn test_eval_if_expression() {
        let tests = vec![
            ("if(true) {1;};", Some(1)),
            ("if (true) { 1; } else { 2; };", Some(1)),
            ("if (false) { 10; };", None),
            ("if (1) { 10; };", Some(10)),
            ("if (1 < 2) { 10; };", Some(10)),
            ("if (1 > 2) { 10; };", None),
            ("if (1 > 2) { 10; } else { 20; };", Some(20)),
            ("if (1 < 2) { 10; } else { 20; };", Some(10)),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let results = eval_program(&program);
            match results {
                Ok(result) => match expected {
                    Some(value) => assert_is_integer(&result, value),
                    None => assert_eq!(result.inspect(), "null"),
                },
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}
