use std::rc::Rc;
use std::collections::BTreeMap;
use itertools::Itertools;
use expr::Expr;
use value::Value;
use scope::Scope;

fn progn(args: Vec<Expr>, scope: Rc<Scope>) -> Value {
    args.into_iter()
        .map(|e| e.eval(scope.clone()))
        .last()
        .unwrap_or(Value::Nil)
}

fn set(args: Vec<Expr>, scope: Rc<Scope>) -> Value {
    if args.len() % 2 != 0 {
        panic!("Uneven symbol and value pairs");
    }

    for (symbol, value) in args.into_iter().tuples() {
        scope.clone().set_value(symbol.as_symbol(),
                                value.eval(scope.clone()));
    }

    Value::Nil
}

fn let_block(args: Vec<Expr>, parent_scope: Rc<Scope>) -> Value {
    let scope = parent_scope.clone().push();

    let mut iter = args.into_iter();
    let vars = iter.next()
        .expect("Expected variables list")
        .as_sexpr();

    for var in vars {
        match var {
            Expr::Symbol(sym) => {
                scope.set_value(sym, Value::Nil)
            },
            Expr::Sexpr(var_val) => {
                assert!(var_val.len() == 2,
                        "Expected symbol or symbol and value pair");

                let mut iter = var_val.into_iter();
                let symbol = iter.next().unwrap().as_symbol();
                let value = iter.next().unwrap();

                scope.clone().set_value(symbol, value.eval(parent_scope.clone()));
            },
            _ => {
                panic!("Expected symbol or symbol and value pair");
            }
        }
    }

    iter.map(|e| e.eval(scope.clone()))
        .last()
        .unwrap_or(Value::Nil)
}

pub fn defun(args: Vec<Expr>, parent_scope: Rc<Scope>) -> Value {
    let mut iter = args.into_iter();
    let name = iter.next()
        .expect("Expected function name")
        .as_symbol();
    let params: Vec<String> = iter.next()
        .expect("Expected parameters list")
        .as_sexpr()
        .into_iter()
        .map(|e| e.as_symbol())
        .collect();

    parent_scope.set_value(name.clone(), Value::Function {
        name,
        params,
        expr: Expr::progn(iter.collect()),
        parent_scope: parent_scope.clone()
    });

    Value::Nil
}

pub fn quote(args: Vec<Expr>, scope: Rc<Scope>) -> Value {
    assert!(args.len() == 1, "Expected only one argument");

    let expr = args.into_iter().next().unwrap();

    if let Expr::Symbol(s) = expr {
        Value::Symbol(s)
    } else if let Expr::Sexpr(s) = expr {
        Value::Sexpr(s)
    } else {
        expr.eval(scope)
    }
}

pub fn register(scope: &mut BTreeMap<String, Value>) {
    scope.insert("set".to_string(),
                 Value::NativeMacro("set".to_string(), set));
    scope.insert("let".to_string(),
                 Value::NativeMacro("let".to_string(), let_block));
    scope.insert("defun".to_string(),
                 Value::NativeMacro("defun".to_string(), defun));
    scope.insert("progn".to_string(),
                 Value::NativeMacro("progn".to_string(), progn));
    scope.insert("quote".to_string(),
                 Value::NativeMacro("quote".to_string(), quote));
}
