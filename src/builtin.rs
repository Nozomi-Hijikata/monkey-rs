use crate::evaluator::new_error;
use crate::object::{Builtin, Integer, Object, ObjectRef, StringObj};
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
    } else {
        return new_error(format_args!(
            "argument to `len` not supported, got {}",
            args[0].object_type().as_str()
        ));
    }
}

lazy_static! {
    pub static ref BUILTINS: HashMap<String, Builtin> = {
        let mut builtins = HashMap::new();
        builtins.insert("len".to_string(), Builtin { func: len_builtin });
        builtins
    };
}

pub fn get_builtin(name: &str) -> Option<Builtin> {
    BUILTINS.get(name).cloned()
}
