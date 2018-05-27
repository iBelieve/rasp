#![feature(arbitrary_self_types)]

extern crate itertools;
#[macro_use]
extern crate nom;
extern crate runtime_fmt;

mod expr;
mod value;
mod params;
mod function;
mod scope;
mod parser;
mod macros;
mod functions;

pub use parser::parse;

use value::Value;
use scope::Scope;

pub fn read(expr: &str) -> Value {
    let mut exprs: Vec<Value> = parse(expr)
        .expect("Unable to parse input")
        .into_iter()
        .map(|e| e.into_value())
        .collect();

    if exprs.len() == 0 {
        Value::Nil
    } else if exprs.len() == 1 {
        exprs.remove(0)
    } else {
        Value::progn(Value::list(exprs.into_iter()))
    }
}

pub fn read_and_eval(expr: &str) -> Value {
    eval(read(expr))
}

pub fn eval(expr: Value) -> Value {
    expr.eval(&Scope::root())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    pub fn eval_set() {
        assert_eq!(read_and_eval("(set a 2 b 3)\
                                  (+ a b)"),
                   Value::Integer(5));
    }

    #[test]
    pub fn eval_let() {
        assert_eq!(read_and_eval("(let ((a 2) (b 3))\
                                    (+ a b))"),
                   Value::Integer(5));
    }

    #[test]
    pub fn eval_after_let() {
        assert_eq!(read_and_eval("(set a 10 b 20)\
                                  (let ((a 2)\
                                        (b 3))\
                                    (+ a b)) (+ a b)"),
                   Value::Integer(30));
    }

    #[test]
    pub fn eval_defun() {
        assert_eq!(read_and_eval("(defun plus (a b)\
                                    (+ a b))\
                                  (plus 4 6)"),
                   Value::Integer(10));
    }

    #[test]
    pub fn eval_addition() {
        assert_eq!(read_and_eval("(+ 1 2.4)"),
                   Value::Float(3.4));
    }

    #[test]
    pub fn eval_nested_addition() {
        assert_eq!(read_and_eval("(+ (+ 3.1 1) 2.4)"),
                   Value::Float(6.5));
    }
}
