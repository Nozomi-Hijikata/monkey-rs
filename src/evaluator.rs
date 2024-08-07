use crate::ast::{Expr, Node, Opcode, Program, Stmt};
use crate::builtin::get_builtin;
use crate::environment::Environment;
use crate::object::{
    Array, Boolean, Builtin, Error, Function, Hash, HashPair, Hashable, Integer, Null, ObjectRef,
    ReturnValue, StringObj,
};
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
            Expr::StringLit(s) => box_it!(StringObj {
                // NOTE: Trim the double quotes from the Expr::StringLit. This is because it
                // contains the double quotes in the AST node value.
                // TODO: This should be done in the parser.
                value: s.trim_matches('"').to_string()
            }),
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
                apply_function(function, args.as_slice())
            }
            Expr::ArrayLit { ref elements } => {
                let elements = eval_expressions(elements, env);
                if elements.len() == 1 && is_error(&elements[0]) {
                    return elements[0].clone();
                }
                box_it!(Array { elements })
            }
            Expr::Index {
                ref left,
                ref index,
            } => {
                let left = eval(left.as_ref(), env);
                if is_error(&left) {
                    return left;
                }

                let index = eval(index.as_ref(), env);
                if is_error(&index) {
                    return index;
                }
                eval_index_expression(&left, &index)
            }
            Expr::HashLit { ref pairs } => eval_hash_literal(pairs, env),
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
    } else if let (Some(left_str), Some(right_str)) = (
        downcast_ref!(left, StringObj),
        downcast_ref!(right, StringObj),
    ) {
        eval_string_infix_expression(operator, left_str, right_str)
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

fn eval_string_infix_expression(
    operator: &Opcode,
    left: &StringObj,
    right: &StringObj,
) -> ObjectRef {
    match operator {
        Opcode::Add => {
            let left_str = &left.value;
            let right_str = &right.value;
            box_it!(StringObj {
                value: format!("{}{}", left_str, right_str),
            })
        }
        _ => new_error(format_args!(
            "unknown operator: STRING {} STRING",
            operator.as_str()
        )),
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
    if let Some(builtin) = get_builtin(name) {
        return box_it!(builtin);
    }
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

pub fn new_error(args: fmt::Arguments) -> ObjectRef {
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

fn apply_function(function: ObjectRef, args: &[ObjectRef]) -> ObjectRef {
    if let Some(builtin) = downcast_ref!(function, Builtin) {
        return (builtin.func)(args.to_vec());
    }

    if let Some(func) = downcast_ref!(function, Function) {
        let mut extended_env = Environment::new_enclosed(&func.env);
        for (param, arg) in func.parameters.iter().zip(args.iter()) {
            if let Expr::Identifier(name) = param.as_ref() {
                extended_env.set(name.clone(), arg.clone());
            } else {
                return new_error(format_args!("invalid parameter: {:?}", param));
            }
        }
        let evaluated = eval(func.body.as_ref(), &mut extended_env);
        if let Some(return_value) = downcast_ref!(evaluated, ReturnValue) {
            return return_value.value.clone();
        }
        return evaluated;
    }

    new_error(format_args!(
        "not a function: {:?}",
        function.object_type().as_str()
    ))
}

fn eval_index_expression(left: &ObjectRef, index: &ObjectRef) -> ObjectRef {
    if let (Some(array), Some(integer)) =
        (downcast_ref!(left, Array), downcast_ref!(index, Integer))
    {
        let idx = integer.value as usize;
        let max = array.elements.len() - 1;
        if idx >= array.elements.len() || idx > max {
            return box_it!(Null);
        }
        array.elements[idx].clone()
    } else if let Some(hash) = downcast_ref!(left, Hash) {
        eval_hash_index_expression(hash, index)
    } else {
        new_error(format_args!(
            "index operator not supported: {}[{}]",
            left.object_type().as_str(),
            index.object_type().as_str()
        ))
    }
}

fn eval_hash_literal(pairs: &[(Box<Expr>, Box<Expr>)], env: &mut Environment) -> ObjectRef {
    let mut hash = std::collections::HashMap::new();
    for (key_expr, value_expr) in pairs {
        let key = eval(key_expr.as_ref(), env);
        if is_error(&key) {
            return key;
        }

        let hash_key = if let Some(integer) = downcast_ref!(&key, Integer) {
            integer.hash_key()
        } else if let Some(boolean) = downcast_ref!(&key, Boolean) {
            boolean.hash_key()
        } else if let Some(string) = downcast_ref!(&key, StringObj) {
            string.hash_key()
        } else {
            return new_error(format_args!("unusable as hash key: {:?}", key.inspect()));
        };

        let value = eval(value_expr.as_ref(), env);
        if is_error(&value) {
            return value;
        }
        let pair = HashPair { key, value };

        hash.insert(hash_key, pair);
    }

    box_it!(Hash { pairs: hash })
}

fn eval_hash_index_expression(hash: &Hash, index: &ObjectRef) -> ObjectRef {
    let key = if let Some(integer) = downcast_ref!(index, Integer) {
        integer.hash_key()
    } else if let Some(boolean) = downcast_ref!(index, Boolean) {
        boolean.hash_key()
    } else if let Some(string) = downcast_ref!(index, StringObj) {
        string.hash_key()
    } else {
        return new_error(format_args!("unusable as hash key: {:?}", index.inspect()));
    };

    if let Some(pair) = hash.pairs.get(&key) {
        pair.value.clone()
    } else {
        box_it!(Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{object::Function, object::Object, parser::parse_program};

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
            (
                "\"Hello\" - \"World\";",
                "unknown operator: STRING - STRING",
            ),
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

    #[test]
    fn test_string_literal() {
        let input = "\"Hello, World!\";";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(string) = downcast_ref!(&results, StringObj) {
            assert_eq!(string.inspect(), "\"Hello, World!\"");
        } else {
            panic!("Expected String object");
        }
    }

    #[test]
    fn test_string_concatenation() {
        let input = "\"Hello\" + \" \" + \"World!\";";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(string) = downcast_ref!(&results, StringObj) {
            assert_eq!(string.inspect(), "\"Hello World!\"");
        } else {
            panic!("Expected String object");
        }
    }

    #[test]
    fn test_builtin_functions_with_integer() {
        let tests = vec![
            ("len(\"\");", 0),
            ("len(\"four\");", 4),
            ("len(\"hello world\");", 11),
            ("len([1,2,3]);", 3),
            ("len([]);", 0),
            ("len([1, 2 * 2, 3 + 3]);", 3),
            ("first([1, 2, 3]);", 1),
            ("last([1, 2, 3]);", 3),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env);
            match results {
                Ok(result) => {
                    if let Some(integer) = downcast_ref!(&result, Integer) {
                        assert_eq!(integer.value, expected);
                    } else {
                        panic!("Expected Integer object");
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    #[test]
    fn test_builtin_functions_with_slices() {
        let tests: Vec<(&str, &[i64])> = vec![
            ("rest([1, 2, 3]);", &[2, 3]),
            ("push([], 1);", &[1]),
            ("push([1], 2);", &[1, 2]),
            ("push([1, 2], 3);", &[1, 2, 3]),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env);
            match results {
                Ok(result) => {
                    if let Some(array) = downcast_ref!(&result, Array) {
                        for (i, element) in array.elements.iter().enumerate() {
                            if let Some(integer) = downcast_ref!(element, Integer) {
                                assert_eq!(integer.value, expected[i]);
                            } else {
                                panic!("Expected Integer object");
                            }
                        }
                    } else {
                        panic!("Expected Array object");
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    #[test]
    fn test_builtin_functions_with_null() {
        let tests = vec![("first([]);", "null"), ("last([]);", "null")];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env);
            match results {
                Ok(result) => {
                    if let Some(null) = downcast_ref!(&result, Null) {
                        assert_eq!(null.inspect(), expected);
                    } else {
                        panic!("Expected Null object");
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
    }

    #[test]
    fn test_builtin_functions_with_errors() {
        let tests = vec![
            ("len(1);", "argument to `len` not supported, got INTEGER"),
            (
                "len(\"one\", \"two\");",
                "wrong number of arguments. got=2, want=1",
            ),
            (
                "first(1);",
                "argument to `first` must be ARRAY, got INTEGER",
            ),
            ("last(1);", "argument to `last` must be ARRAY, got INTEGER"),
            ("rest(1);", "argument to `rest` must be ARRAY, got INTEGER"),
            (
                "push(1, 1);",
                "argument to `push` must be ARRAY, got INTEGER",
            ),
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
    fn test_array_literals() {
        let input = "[1, 2 * 2, 3 + 3];";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(array) = downcast_ref!(&results, Array) {
            assert_eq!(array.elements.len(), 3);
            assert_is_integer(&array.elements[0], 1);
            assert_is_integer(&array.elements[1], 4);
            assert_is_integer(&array.elements[2], 6);
        } else {
            panic!("Expected Array object");
        }
    }

    #[test]
    fn test_array_index_expressions() {
        let tests = vec![
            ("[1, 2, 3][0];", 1),
            ("[1, 2, 3][1];", 2),
            ("[1, 2, 3][2];", 3),
            ("let i = 0; [1][i];", 1),
            ("[1, 2, 3][1 + 1];", 3),
            ("let myArray = [1, 2, 3]; myArray[2];", 3),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                6,
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i];",
                2,
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
    fn test_array_index_null_object() {
        let input = "[1, 2, 3][3];";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(null) = downcast_ref!(&results, Null) {
            assert_eq!(null.inspect(), "null");
        } else {
            panic!("Expected Null object");
        }
    }

    #[test]
    fn test_eval_hash_literal_with_string() {
        let input = "
        let two = \"two\";
        {
            \"one\": 10 - 9,
            two: 1 + 1,
            \"thr\" + \"ee\": 6 / 2,
        };
        ";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(hash) = downcast_ref!(&results, Hash) {
            let expected = vec![("one", 1), ("two", 2), ("three", 3)];
            for (key, value) in expected {
                let key_object = StringObj {
                    value: key.to_string(),
                };

                let key_hash = key_object.hash_key();
                let pair = hash.pairs.get(&key_hash).unwrap();
                assert_is_integer(&pair.value, value);
            }
        } else {
            panic!("Expected Hash object");
        }
    }

    #[test]
    fn test_eval_hash_literal_with_integer() {
        let input = "{1: 1, 2: 2, 3: 3};";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(hash) = downcast_ref!(&results, Hash) {
            let expected = vec![(1, 1), (2, 2), (3, 3)];
            for (key, value) in expected {
                let key_object = Integer { value: key };
                let key_hash = key_object.hash_key();
                let pair = hash.pairs.get(&key_hash).unwrap();
                assert_is_integer(&pair.value, value);
            }
        } else {
            panic!("Expected Hash object");
        }
    }

    #[test]
    fn test_eval_hash_literal_with_boolean() {
        let input = "{true: 1, false: 0};";
        let program = parse_program(input).unwrap();
        let mut env = Environment::new();
        let results = eval_program(&program, &mut env).unwrap();
        if let Some(hash) = downcast_ref!(&results, Hash) {
            let expected = vec![(true, 1), (false, 0)];
            for (key, value) in expected {
                let key_object = Boolean { value: key };
                let key_hash = key_object.hash_key();
                let pair = hash.pairs.get(&key_hash).unwrap();
                assert_is_integer(&pair.value, value);
            }
        } else {
            panic!("Expected Hash object");
        }
    }

    #[test]
    fn test_hash_index_expressions() {
        let tests = vec![
            ("{\"foo\": 5}[\"foo\"];", 5),
            ("let key = \"foo\"; {\"foo\": 5}[key];", 5),
            ("{5: 5}[5];", 5),
            ("{true: 5}[true];", 5),
            ("{false: 5}[false];", 5),
        ];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env).unwrap();
            assert_is_integer(&results, expected);
        }
    }

    #[test]
    fn test_hash_index_expressions_with_null() {
        let tests = vec![("{\"foo\": 5}[\"bar\"];", "null"), ("{}[\"foo\"];", "null")];

        for (input, expected) in tests {
            let program = parse_program(input).unwrap();
            let mut env = Environment::new();
            let results = eval_program(&program, &mut env);
            match results {
                Ok(result) => {
                    if let Some(null) = downcast_ref!(&result, Null) {
                        assert_eq!(null.inspect(), expected);
                    } else {
                        panic!("Expected Null object");
                    }
                }
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}
