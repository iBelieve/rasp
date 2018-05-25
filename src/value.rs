use expr::Expr;
use scope::Scope;
use std::ops::Add;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    NativeFunction(fn(Vec<Value>) -> Value),
    NativeMacro(fn(Vec<Expr>, Rc<Scope>) -> Value),
    Function { params: Vec<String>, expr: Expr, parent_scope: Rc<Scope> },
    Nil
}

impl Value {
    #[allow(dead_code)]
    fn is_nil(&self) -> bool {
        if let Value::Nil = self {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn expect(self, message: &str) -> Value {
        if self.is_nil() {
            panic!("{}", message);
        }

        self
    }

    #[allow(dead_code)]
    fn unwrap(self) -> Value {
        self.expect("Expected non-nil value")
    }

    pub fn call(self, args: Vec<Expr>, scope: Rc<Scope>) -> Value {
        use self::Value::*;

        match self {
            Nil => panic!("Cannot call nil function"),
            NativeFunction(func) => {
                let args = args.into_iter()
                    .map(|e| e.eval(scope.clone()))
                    .collect();
                func(args)
            },
            NativeMacro(func) => {
                func(args, scope)
            },
            Function { params, expr, parent_scope } => {
                assert!(args.len() == params.len(), "Unexpected number of arguments");

                let fn_scope = parent_scope.push();

                for (param, arg) in params.into_iter().zip(args.into_iter()) {
                    fn_scope.set_value(param, arg.eval(scope.clone()));
                }

                expr.eval(fn_scope)
            }
            _ => panic!("Expected function")
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, other: Value) -> Value {
        use self::Value::*;

        match (self, other) {
            (Integer(a), Integer(b)) => Value::Integer(a + b),
            (Integer(a), Float(b)) => Value::Float(a as f64 + b),
            (Float(a), Integer(b)) => Value::Float(a + b as f64),
            (Float(a), Float(b)) => Value::Float(a + b),
            (a, b) => panic!("Unable to add {:?} and {:?}", a, b)
        }
    }
}

pub trait Reduce<T> {
    fn reduce<F>(self, f: F) -> Option<T>
        where Self: Sized,
              F: FnMut(T, T) -> T;
}

impl<T, I> Reduce<T> for I where I: Iterator<Item=T> {
    #[inline]
    fn reduce<F>(mut self, f: F) -> Option<T>
        where Self: Sized,
              F: FnMut(T, T) -> T,
    {
        self.next().map(|first| self.fold(first, f))
    }
}
