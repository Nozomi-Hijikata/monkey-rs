use crate::ast::{Expr, Node, Opcode, Program, Stmt};
use crate::environment::Environment;
use crate::object::{Boolean, Error, Function, Integer, Null, Object, ObjectRef, ReturnValue};
use crate::{box_it, downcast_ref};
use std::fmt;

pub fn eval_program(program: &Program, env: &mut Environment) -> Result<ObjectRef, String> {
    Ok(program.eval(env))
}

fn is_error(object: &ObjectRef) -> bool {
    downcast_ref!(object, Error).is_some()
}

fn eval(node: &dyn Node, env: &mut Environment) -> ObjectRef {
    node.eval(env)
}

impl Node for Program {
    fn eval(&self, env: &mut Environment) -> ObjectRef {
        let mut result: ObjectRef = box_it!(Null);
        for stmt in &self.statements {
            result = eval(stmt.as_ref(), env);
            if let Some(return_value) = downcast_ref!(result, ReturnValue) {
                return return_value.value.clone();
            }

            if let Some(_) = downcast_ref!(result, Error) {
                return result;
            }
        }
        result
    }
}

impl Node for Stmt {
    fn eval(&self, env: &mut Environment) -> ObjectRef {
        match self {
            Stmt::Let {
                ref name,
                ref value,
            } => {
                let value = eval(value.as_ref(), env);
                if is_error(&value) {
                    return value;
                }
                env.set(name.clone(), value)
            }
            Stmt::Return { ref return_value } => {
                let value = eval(return_value.as_ref(), env);
                if is_error(&value) {
                    return value;
                }
                box_it!(ReturnValue { value })
            }
            Stmt::Expr { ref expression } => eval(expression.as_ref(), env),
            Stmt::Block { ref statements } => {
                let mut result: ObjectRef = box_it!(Null);
                for stmt in statements {
                    result = eval(stmt.as_ref(), env);
                    if let Some(_) = downcast_ref!(result, ReturnValue) {
                        return result;
                    } else if let Some(_) = downcast_ref!(result, Error) {
                        return result;
                    }
                }
                result
            }
        }
    }
}

impl Node for Expr {
    fn eval(&self, env: &mut Environment) -> ObjectRef {
        match self {
            Expr::Number(n) => box_it!(Integer { value: *n }),
            Expr::Identifier(ident) => {
                let value = eval_identifier_expression(ident, env);
                if is_error(&value) {
                    return value;
                }
                value
            }
            Expr::Boolean(b) => eval_native_boolean(b),
            Expr::InfixOp {
                ref left,
                ref operator,
                ref right,
            } => {
                let left_value = eval(left.as_ref(), env);
                if is_error(&left_value) {
                    return left_value;
                }
                let right_value = eval(right.as_ref(), env);
                if is_error(&right_value) {
                    return right_value;
                }
                eval_infix_expression(operator, &left_value, &right_value)
            }
            Expr::PrefixOp {
                ref operator,
                ref right,
            } => {
                let right_value = eval(right.as_ref(), env);
                if is_error(&right_value) {
                    return right_value;
                }
                eval_prefix_expression(operator, &right_value)
            }
            Expr::If {
                ref condition,
                ref consequence,
                ref alternative,
            } => {
                let condition_value = eval(condition.as_ref(), env);
                if is_truthy(&condition_value) {
                    eval(consequence.as_ref(), env)
                } else {
                    match alternative {
                        Some(alt) => eval(alt.as_ref(), env),
                        None => box_it!(Null),
                    }
                }
            }
            Expr::FuncLit {
                ref parameters,
                ref body,
            } => {
                box_it!(Function {
                    parameters: parameters.clone(),
                    body: body.clone(),
                    env: env.clone(),
                })
            }
            Expr::Call {
                ref function,
                ref arguments,
            } => {
                let function = eval(function.as_ref(), env);
                if is_error(&function) {
                    return function;
                }
                let args = eval_expressions(arguments, env);
                if args.len() == 1 && is_error(&args[0]) {
                    return args[0].clone();
                }
                apply_function(downcast_ref!(function, Function).unwrap(), args.as_slice())
            }
        }
    }
}

fn eval_infix_expression(operator: &Opcode, left: &ObjectRef, right: &ObjectRef) -> ObjectRef {
    if left.object_type() != right.object_type() {
        return new_error(format_args!(
            "type mismatch: {} {} {}",
            left.object_type().as_str(),
            operator.as_str(),
            right.object_type().as_str()
        ));
    }
    if let (Some(left_int), Some(right_int)) =
        (downcast_ref!(left, Integer), downcast_ref!(right, Integer))
    {
        eval_integer_infix_expression(operator, left_int, right_int)
    } else {
        match operator {
            Opcode::Eq | Opcode::NotEq => eval_boolean_infix_expression(operator, left, right),
            _ => new_error(format_args!(
                "unknown operator: {} {} {}",
                left.object_type().as_str(),
                operator.as_str(),
                right.object_type().as_str()
            )),
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
        _ => new_error(format_args!(
            "unknown operator: INTEGER {} INTEGER",
            operator.as_str()
        )),
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
            _ => new_error(format_args!(
                "unknown operator: BOOLEAN {} BOOLEAN",
                operator.as_str()
            )),
        }
    } else {
        new_error(format_args!(
            "unknown operator: {} {} {}",
            left.object_type().as_str(),
            operator.as_str(),
            right.object_type().as_str()
        ))
    }
}

fn eval_prefix_expression(operator: &Opcode, right: &ObjectRef) -> ObjectRef {
    match operator {
        Opcode::Bang => eval_bang_operator_expression(right),
        Opcode::Sub => match downcast_ref!(right, Integer) {
            Some(integer) => box_it!(Integer {
                value: -integer.value,
            }),
            _ => new_error(format_args!(
                "unknown operator: -{}",
                right.object_type().as_str()
            )),
        },
        _ => new_error(format_args!(
            "unknown operator: {}{}",
            operator.as_str(),
            right.object_type().as_str()
        )),
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

fn eval_identifier_expression(name: &str, env: &Environment) -> ObjectRef {
    match env.get(name) {
        Some(value) => value,
        None => new_error(format_args!("identifier not found: {}", name)),
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

fn new_error(args: fmt::Arguments) -> ObjectRef {
    let message = format!("{}", args);
    box_it!(Error { message })
}

fn eval_expressions(expressions: &[Box<Expr>], env: &mut Environment) -> Vec<ObjectRef> {
    let mut result = Vec::new();
    for expr in expressions {
        let evaluated = eval(expr.as_ref(), env);
        if is_error(&evaluated) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    result
}

fn apply_function(function: &Function, args: &[ObjectRef]) -> ObjectRef {
    let mut extended_env = Environment::new_enclosed(&function.env);
    for (param, arg) in function.parameters.iter().zip(args.iter()) {
        if let Expr::Identifier(name) = param.as_ref() {
            extended_env.set(name.clone(), arg.clone());
        } else {
            return new_error(format_args!("invalid parameter: {:?}", param));
        }
    }
    let evaluated = eval(function.body.as_ref(), &mut extended_env);
    if let Some(return_value) = downcast_ref!(evaluated, ReturnValue) {
        return return_value.value.clone();
    }
    evaluated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{object::Function, parser::parse_program};

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
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
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
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
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
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
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
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env);
            match results {
                Ok(result) => match expected {
                    Some(value) => assert_is_integer(&result, value),
                    None => assert_eq!(result.inspect(), "null"),
                },
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    #[test]
    fn test_eval_return_statement() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                "if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    };
                    return 1;
                };",
                10,
            ),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
            assert_is_integer(&results, expected);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true;", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5;", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; };",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    };
                    return 1;
                };
                ",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar;", "identifier not found: foobar"),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env);
            match results {
                Ok(result) => {
                    if let Some(_) = downcast_ref!(result, Error) {
                        assert_eq!(result.inspect(), expected);
                    } else {
                        panic!("Expected error object");
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    #[test]
    fn test_let_statement() {
        let tests = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
            assert_is_integer(&results, expected);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(function) = downcast_ref!(&results, Function) {
            assert_eq!(function.inspect(), "fn(x) {\n  (x + 2)\n}");
            assert_eq!(function.object_type().as_str(), "FUNCTION");
            assert_eq!(function.parameters.len(), 1);
        } else {
            panic!("Expected Function object");
        }
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5);", 5),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
            assert_is_integer(&results, expected);
        }
    }

    #[test]
    fn test_closures() {
        let input = "
        let newAdder = fn(x) {
            fn(y) { x + y; };
        };
        let addTwo = newAdder(2);
        addTwo(3);
        ";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        assert_is_integer(&results, 5);
    }
}
