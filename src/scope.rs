use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use value::{Value};
use macros;
use functions;

#[derive(Debug, PartialEq)]
pub struct Scope {
    parent: Option<Rc<Scope>>,
    variables: RefCell<HashMap<String, Value>>
}

impl Scope {
    pub fn root() -> Rc<Scope> {
        let mut variables = HashMap::new();

        macros::register(&mut variables);
        functions::register(&mut variables);

        Rc::new(Scope {
            parent: None,
            variables: RefCell::new(variables)
        })
    }

    pub fn push(self: Rc<Self>) -> Rc<Scope> {
        Rc::new(Scope {
            parent: Some(self),
            variables: RefCell::new(HashMap::new())
        })
    }

    pub fn get_value(&self, symbol: &str) -> Value {
        if symbol == "nil" {
            Value::Nil
        } else if symbol == "true" {
            Value::Boolean(true)
        } else if symbol == "false" {
            Value::Boolean(false)
        } else if symbol.starts_with(":") {
            Value::Symbol(symbol.to_string())
        } else if self.variables.borrow().contains_key(symbol) {
            self.variables.borrow()[symbol].clone()
        } else if let Some(ref parent) = self.parent {
            parent.clone().get_value(symbol)
        } else {
            panic!("Symbol not found: {}", symbol);
        }
    }

    pub fn set_value(&self, symbol: String, value: Value) {
        self.variables.borrow_mut().insert(symbol, value);
    }
}
