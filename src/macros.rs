use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;
use itertools::Itertools;
use value::Value;
use scope::Scope;
use function::{Function, Macro};

fn progn(args: Vec<Rc<Value>>, scope: Rc<Scope>) -> Value {
    args.into_iter()
        .map(|e| e.eval(&scope))
        .last()
        .unwrap_or(Value::Nil)
}

fn if_macro(args: Vec<Rc<Value>>, scope: Rc<Scope>) -> Value {
    let mut iter = args.into_iter();
    let condition = iter.next()
        .expect("Expected if condition")
        .eval(&scope);
    let when_true = iter.next()
        .expect("Expected statement to execute when true");
    let when_false = Value::progn(Value::list_rc(iter));

    if let Value::Boolean(condition) = condition {
        if condition {
            when_true.eval(&scope)
        } else {
            when_false.eval(&scope)
        }
    } else {
        panic!("Expected boolean condition, got: {:?}", condition);
    }
}

fn set(args: Vec<Rc<Value>>, scope: Rc<Scope>) -> Value {
    if args.len() % 2 != 0 {
        panic!("Uneven symbol and value pairs");
    }

    for (symbol, value) in args.into_iter().tuples() {
        scope.set_value(symbol.as_symbol().expect("Expected symbol").to_string(),
                                value.eval(&scope));
    }

    Value::Nil
}

fn let_block(args: Vec<Rc<Value>>, parent_scope: Rc<Scope>) -> Value {
    let scope = parent_scope.clone().push();

    let mut iter = args.into_iter();
    let vars = iter.next()
        .expect("Expected variables list")
        .iter_cons();

    for var in vars {
        if let Value::Symbol(sym) = var.deref() {
            scope.set_value(sym.to_string(), Value::Nil)
        } else if let Some((symbol, value)) = var.as_symbol_value_pair() {
            scope.clone().set_value(symbol.to_string(),
                                    value.eval(&parent_scope));
        } else {
            panic!("Expected symbol or symbol and value pair");
        }
    }

    iter.map(|e| e.eval(&scope))
        .last()
        .unwrap_or(Value::Nil)
}

pub fn defun(args: Vec<Rc<Value>>, parent_scope: Rc<Scope>) -> Value {
    let mut iter = args.into_iter();
    let name = iter.next()
        .and_then(|e| e.as_symbol().map(|s| s.to_string()))
        .expect("Expected function name");
    let params = iter.next().expect("Expected parameter definitions");

    let function = Function::define(name.clone(), &params,
                                    Rc::new(Value::list_rc(iter)),
                                    parent_scope.clone());

    parent_scope.set_value(name, Value::Function(Rc::new(function)));

    Value::Nil
}

pub fn defmacro(args: Vec<Rc<Value>>, parent_scope: Rc<Scope>) -> Value {
    let mut iter = args.into_iter();
    let name = iter.next()
        .and_then(|e| e.as_symbol().map(|s| s.to_string()))
        .expect("Expected macro name");
    let params = iter.next().expect("Expected parameter definitions");

    let func = Macro::define(name.clone(), &params,
                              Rc::new(Value::list_rc(iter)),
                              parent_scope.clone());

    parent_scope.set_value(name, Value::Macro(Rc::new(func)));

    Value::Nil
}

pub fn quote(args: Vec<Rc<Value>>, _scope: Rc<Scope>) -> Value {
    assert!(args.len() == 1, "Expected only one argument");

    args.into_iter().next().unwrap().deref().clone()
}

pub fn register(scope: &mut HashMap<String, Value>) {
    scope.insert("set".to_string(),
                 Value::NativeMacro("set".to_string(), set));
    scope.insert("let".to_string(),
                 Value::NativeMacro("let".to_string(), let_block));
    scope.insert("if".to_string(),
                 Value::NativeMacro("if".to_string(), if_macro));
    scope.insert("defun".to_string(),
                 Value::NativeMacro("defun".to_string(), defun));
    scope.insert("defmacro".to_string(),
                 Value::NativeMacro("defmacro".to_string(), defmacro));
    scope.insert("progn".to_string(),
                 Value::NativeMacro("progn".to_string(), progn));
    scope.insert("quote".to_string(),
                 Value::NativeMacro("quote".to_string(), quote));
}
