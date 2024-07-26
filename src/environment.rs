use crate::{
    box_it,
    object::{Null, ObjectRef},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, ObjectRef>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: &Environment) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(box_it!(outer.clone())),
        }
    }

    pub fn get(&self, name: &str) -> Option<ObjectRef> {
        self.store
            .get(name)
            .cloned()
            .or_else(|| self.outer.as_ref().and_then(|outer| outer.get(name)))
    }

    pub fn set(&mut self, name: String, value: ObjectRef) -> ObjectRef {
        self.store
            .insert(name, value)
            .unwrap_or_else(|| box_it!(Null))
    }
}
