use expr::Expr;
use scope::Scope;
use std::ops::Add;
use std::rc::Rc;
use std::ops::Deref;
use std::fmt;
use function::Function;

#[derive(PartialEq, Clone)]
pub enum Value {
    Float(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    NativeFunction(String, fn(Vec<Value>) -> Value),
    NativeMacro(String, fn(Vec<Expr>, Rc<Scope>) -> Value),
    Function(Rc<Function>),
    Symbol(String),
    Sexpr(Rc<Vec<Expr>>),
    Cons(Rc<Value>, Rc<Value>),
    Nil
}

impl Value {
    pub fn flatten_list(&self) -> Vec<Rc<Value>> {
        if let Value::Nil = self {
            Vec::new()
        } else if let Value::Cons(left, right) = self {
            let mut list = vec![left.clone()];
            list.extend(right.flatten_list());
            list
        } else {
            panic!("Not a valid list");
        }
    }


    pub fn as_string(self) -> String {
        match self {
            Value::String(s) => s,
            _ => panic!("Expected string, got: {:?}", self)
        }
    }

    pub fn as_keyword_symbol(&self) -> Option<String> {
        if let Value::Symbol(sym) = self {
            if sym.starts_with(":") {
                return Some(sym[1..].to_string());
            }
        }

        None
    }

    pub fn call(self, args: Vec<Expr>, scope: Rc<Scope>) -> Value {
        use self::Value::*;

        match self {
            Nil => panic!("Cannot call nil function"),
            NativeFunction(_name, func) => {
                let args = args.into_iter()
                    .map(|e| e.eval(scope.clone()))
                    .collect();
                func(args)
            },
            NativeMacro(_name, func) => {
                func(args, scope)
            },
            Function(func) => {
                let args = args.into_iter()
                    .map(|e| e.eval(scope.clone()))
                    .collect();
                func.call(args)
            }
            _ => panic!("Expected function")
        }
    }

    pub fn list(mut values: impl Iterator<Item=Value>) -> Value {
        if let Some(value) = values.next() {
            Value::Cons(Rc::new(value), Rc::new(Value::list(values)))
        } else {
            Value::Nil
        }
    }

    pub fn list_rc(mut values: impl Iterator<Item=Rc<Value>>) -> Value {
        if let Some(value) = values.next() {
            Value::Cons(value, Rc::new(Value::list_rc(values)))
        } else {
            Value::Nil
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            Integer(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
            Boolean(b) => write!(f, "{}", b),
            Cons(left, right) => {
                write!(f, "({}", left)?;

                let mut next = right.clone();

                loop {
                    if let Value::Nil = next.clone().deref() {
                        break;
                    } else if let Value::Cons(left, right) = next.clone().deref() {
                        write!(f, " {}", left)?;
                        next = right.clone();
                    } else {
                        write!(f, " . {}", next)?;
                        break;
                    }
                }

                write!(f, ")")
            },
            _ => fmt::Debug::fmt(self, f)
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;
        match self {
            Integer(n) => write!(f, "{:?}", n),
            Float(n) => write!(f, "{:?}", n),
            String(s) => write!(f, "{:?}", s),
            Boolean(b) => write!(f, "{:?}", b),
            Symbol(s) => write!(f, "{}", s),
            Sexpr(e) => write!(f, "{}", Expr::Sexpr(e.to_vec())),
            Cons(left, right) => {
                write!(f, "({:?}", left)?;

                let mut next = right.clone();

                loop {
                    if let Value::Nil = next.clone().deref() {
                        break;
                    } else if let Value::Cons(left, right) = next.clone().deref() {
                        write!(f, " {:?}", left)?;
                        next = right.clone();
                    } else {
                        write!(f, " . {:?}", next)?;
                        break;
                    }
                }

                write!(f, ")")
            },
            Nil => write!(f, "nil"),
            Function(func) => write!(f, "<function {}>", func.name),
            NativeFunction(name, _) => write!(f, "<function {}>", name),
            NativeMacro(name, _) => write!(f, "<macro {}>", name)
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
