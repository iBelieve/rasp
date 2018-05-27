use std::collections::HashMap;
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

fn equal(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("Expected two arguments");
    }

    Value::Boolean(args[0] == args[1])
}

fn plus(args: Vec<Value>) -> Value {
    args.into_iter()
        .reduce(|a, b| a + b)
        .expect("Expected at least two arguments")
}

fn list(args: Vec<Value>) -> Value {
    Value::list(args.into_iter())
}

fn append(args: Vec<Value>) -> Value {
    Value::list_rc(args.into_iter()
                   .flat_map(|value| value.as_list().expect("Not a proper list")))
}

pub fn register(scope: &mut HashMap<String, Value>) {
    scope.insert("println".to_string(),
                 Value::NativeFunction("println".to_string(), println));
    scope.insert("list".to_string(),
                 Value::NativeFunction("list".to_string(), list));
    scope.insert("append".to_string(),
                 Value::NativeFunction("append".to_string(), append));
    scope.insert("=".to_string(),
                 Value::NativeFunction("equal".to_string(), equal));
    scope.insert("+".to_string(),
                 Value::NativeFunction("plus".to_string(), plus));
}
