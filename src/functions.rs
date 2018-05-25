use std::collections::BTreeMap;
use value::{Value, Reduce};

fn println(_args: Vec<Value>) -> Value {
    Value::Nil
}

fn plus(args: Vec<Value>) -> Value {
    args.into_iter()
        .reduce(|a, b| a + b)
        .expect("Expected at least one argument")
}

pub fn register(scope: &mut BTreeMap<String, Value>) {
    scope.insert("println".to_string(), Value::NativeFunction(println));
    scope.insert("+".to_string(), Value::NativeFunction(plus));
}
