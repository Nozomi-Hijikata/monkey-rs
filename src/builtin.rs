use crate::evaluator::new_error;
use crate::object::{Array, Builtin, Integer, Null, ObjectRef, StringObj};
use crate::{box_it, downcast_ref};
use lazy_static::lazy_static;
use std::collections::HashMap;

fn len_builtin(args: Vec<ObjectRef>) -> ObjectRef {
    if args.len() != 1 {
        return new_error(format_args!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    if let Some(s) = downcast_ref!(args[0], StringObj) {
        return box_it!(Integer {
            value: s.value.len() as i64
        });
    } else if let Some(a) = downcast_ref!(args[0], Array) {
        return box_it!(Integer {
            value: a.elements.len() as i64
        });
    } else {
        return new_error(format_args!(
            "argument to `len` not supported, got {}",
            args[0].object_type().as_str()
        ));
    }
}

fn first_builtin(args: Vec<ObjectRef>) -> ObjectRef {
    if args.len() != 1 {
        return new_error(format_args!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    if let Some(a) = downcast_ref!(args[0], Array) {
        if a.elements.is_empty() {
            return box_it!(Null);
        }
        return a.elements[0].clone();
    }
    return new_error(format_args!(
        "argument to `first` must be ARRAY, got {}",
        args[0].object_type().as_str()
    ));
}

fn last_builtin(args: Vec<ObjectRef>) -> ObjectRef {
    if args.len() != 1 {
        return new_error(format_args!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    if let Some(a) = downcast_ref!(args[0], Array) {
        if a.elements.is_empty() {
            return box_it!(Null);
        }
        return a.elements[a.elements.len() - 1].clone();
    }
    return new_error(format_args!(
        "argument to `last` must be ARRAY, got {}",
        args[0].object_type().as_str()
    ));
}

fn rest_builtin(args: Vec<ObjectRef>) -> ObjectRef {
    if args.len() != 1 {
        return new_error(format_args!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    if let Some(a) = downcast_ref!(args[0], Array) {
        if a.elements.is_empty() {
            return box_it!(Null);
        }
        let new_elements = a.elements[1..].to_vec();
        return box_it!(Array {
            elements: new_elements
        });
    }
    return new_error(format_args!(
        "argument to `rest` must be ARRAY, got {}",
        args[0].object_type().as_str()
    ));
}

fn push_builtin(args: Vec<ObjectRef>) -> ObjectRef {
    if args.len() != 2 {
        return new_error(format_args!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        ));
    }
    if let Some(a) = downcast_ref!(args[0], Array) {
        let mut new_elements = a.elements.clone();
        new_elements.push(args[1].clone());
        return box_it!(Array {
            elements: new_elements
        });
    }
    return new_error(format_args!(
        "argument to `push` must be ARRAY, got {}",
        args[0].object_type().as_str()
    ));
}

lazy_static! {
    pub static ref BUILTINS: HashMap<String, Builtin> = {
        let mut builtins = HashMap::new();
        builtins.insert("len".to_string(), Builtin { func: len_builtin });
        builtins.insert(
            "first".to_string(),
            Builtin {
                func: first_builtin,
            },
        );
        builtins.insert("last".to_string(), Builtin { func: last_builtin });
        builtins.insert("rest".to_string(), Builtin { func: rest_builtin });
        builtins.insert("push".to_string(), Builtin { func: push_builtin });
        builtins
    };
}

pub fn get_builtin(name: &str) -> Option<Builtin> {
    BUILTINS.get(name).cloned()
}
