pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

const INTEGER_OBJ: &str = "INTEGER";
// const BOOLEAN_OBJ: &str = "BOOLEAN";
// const NULL_OBJ: &str = "NULL";
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
    // Boolean,
    // Null,
    // ReturnValue,
    // Error,
    // Function,
    // String,
    // Array,
    // Builtin,
    // Hash,
}

impl ObjectType {
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Integer => INTEGER_OBJ,
            // ObjectType::Boolean => BOOLEAN_OBJ,
            // ObjectType::Null => NULL_OBJ,
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
    fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
