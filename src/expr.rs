use std::rc::Rc;
use std::fmt;
use scope::Scope;
use value::Value;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    Symbol(String),
    Sexpr(Vec<Expr>)
}

impl Expr {
    pub fn as_symbol(self) -> String {
        match self {
            Expr::Symbol(sym) => sym,
            _ => panic!("Expected symbol, got: {:?}", self)
        }
    }

    pub fn as_sexpr(self) -> Vec<Expr> {
        match self {
            Expr::Sexpr(expressions) => expressions,
            _ => panic!("Expected sexpr, got: {:?}", self)
        }
    }

    pub fn as_string(self) -> String {
        match self {
            Expr::String(s) => s,
            _ => panic!("Expected string, got: {:?}", self)
        }
    }

    pub fn eval(self, scope: Rc<Scope>) -> Value {
        match self {
            Expr::Integer(i) => Value::Integer(i),
            Expr::Float(f) => Value::Float(f),
            Expr::Boolean(b) => Value::Boolean(b),
            Expr::String(s) => Value::String(s),
            Expr::Symbol(sym) => scope.get_value(&sym),
            Expr::Sexpr(expressions) => {
                let mut iter = expressions
                    .into_iter();
                let first = iter.next()
                    .map(|e| e.eval(scope.clone()))
                    .expect("Expected at least one symbol or value in sexpr");

                first.call(iter.collect(), scope)
            }
        }
    }

    pub fn progn(expressions: Vec<Expr>) -> Expr {
        let mut sexpr = vec![Expr::Symbol("progn".to_string())];
        sexpr.extend(expressions);
        Expr::Sexpr(sexpr)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            String(s) => write!(f, "{:?}", s),
            Boolean(b) => write!(f, "{:?}", b),
            Symbol(s) => write!(f, "{}", s),
            Sexpr(expressions) => {
                write!(f, "({})", expressions.into_iter()
                       .map(|e| format!("{}", e)).join(" "))
            }
        }
    }
}
