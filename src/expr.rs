use std::rc::Rc;
use scope::Scope;
use value::Value;

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
