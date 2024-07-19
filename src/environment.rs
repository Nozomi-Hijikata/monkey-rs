use crate::{
    box_it,
    object::{Null, ObjectRef},
};
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, ObjectRef>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }
    pub fn get(&self, name: &str) -> Option<ObjectRef> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: ObjectRef) -> ObjectRef {
        self.store
            .insert(name, value)
            .unwrap_or_else(|| box_it!(Null))
    }
}
