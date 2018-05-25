use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;
use value::{Value};
use macros;
use functions;

#[derive(Debug, PartialEq)]
pub struct Scope {
    parent: Option<Rc<Scope>>,
    variables: RefCell<BTreeMap<String, Value>>
}

impl Scope {
    pub fn root() -> Rc<Scope> {
        let mut variables = BTreeMap::new();

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
            variables: RefCell::new(BTreeMap::new())
        })
    }

    pub fn get_value(&self, symbol: &str) -> Value {
        if self.variables.borrow().contains_key(symbol) {
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
