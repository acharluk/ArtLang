use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    variables: RefCell<HashMap<String, Value>>,
}

impl Environment {
    pub fn new_global() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            parent: None,
            variables: RefCell::new(HashMap::new()),
        }))
    }

    pub fn new_child(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            parent: Some(parent.clone()),
            variables: RefCell::new(HashMap::new()),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.variables.borrow().get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&self, name: &str, value: Value) {
        self.variables.borrow_mut().insert(name.to_string(), value);
    }

    pub fn assign(&self, name: &str, value: Value) {
        if self.variables.borrow().contains_key(name) {
            self.variables.borrow_mut().insert(name.to_string(), value);
        } else if let Some(parent) = &self.parent {
            parent.borrow().assign(name, value);
        } else {
            self.variables.borrow_mut().insert(name.to_string(), value);
        }
    }
}
