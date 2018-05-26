use std::collections::BTreeMap;
use value::{Value, Reduce};

fn println(args: Vec<Value>) -> Value {
    use runtime_fmt::{FormatBuf, Param, _print};

    let mut iter = args.iter();
    let format_str = iter.next()
        .expect("Expected at least one argument for the format string")
        .clone()
        .as_string();
    let values: Vec<Param> = iter.map(|v| Param::normal(v)).collect();

    FormatBuf::new(&format_str, &values)
        .map(|mut x| x.newln().with(_print))
        .expect("Invalid format string or arguments");

    Value::Nil
}

fn plus(args: Vec<Value>) -> Value {
    args.into_iter()
        .reduce(|a, b| a + b)
        .expect("Expected at least one argument")
}

pub fn register(scope: &mut BTreeMap<String, Value>) {
    scope.insert("println".to_string(),
                 Value::NativeFunction("println".to_string(), println));
    scope.insert("+".to_string(),
                 Value::NativeFunction("plus".to_string(), plus));
}
