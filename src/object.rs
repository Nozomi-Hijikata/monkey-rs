use std::any::Any;
#[allow(dead_code)]
pub trait Object {
    fn as_any(&self) -> &dyn Any;
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub type ObjectRef = Box<dyn Object>;

const INTEGER_OBJ: &str = "INTEGER";
const NULL_OBJ: &str = "NULL";
const BOOLEAN_OBJ: &str = "BOOLEAN";
// const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
// const ERROR_OBJ: &str = "ERROR";
// const FUNCTION_OBJ: &str = "FUNCTION";
// const STRING_OBJ: &str = "STRING";
// const ARRAY_OBJ: &str = "ARRAY";
// const BUILTIN_OBJ: &str = "BUILTIN";
// const HASH_OBJ: &str = "HASH";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ObjectType {
    Integer,
    Null,
    Boolean,
    // ReturnValue,
    // Error,
    // Function,
    // String,
    // Array,
    // Builtin,
    // Hash,
}

#[allow(dead_code)]
impl ObjectType {
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Integer => INTEGER_OBJ,
            ObjectType::Null => NULL_OBJ,
            ObjectType::Boolean => BOOLEAN_OBJ,
            // ObjectType::ReturnValue => RETURN_VALUE_OBJ,
            // ObjectType::Error => ERROR_OBJ,
            // ObjectType::Function => FUNCTION_OBJ,
            // ObjectType::String => STRING_OBJ,
            // ObjectType::Array => ARRAY_OBJ,
            // ObjectType::Builtin => BUILTIN_OBJ,
            // ObjectType::Hash => HASH_OBJ,
        }
    }
}

pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

pub struct Null;

impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
