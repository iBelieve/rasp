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
    NativeMacro(String, fn(Vec<Rc<Value>>, Rc<Scope>) -> Value),
    Function(Rc<Function>),
    Symbol(String),
    Cons(Rc<Value>, Rc<Value>),
    Nil
}

impl Value {
    pub fn as_list(&self) -> Option<Vec<Rc<Value>>> {
        if let Value::Nil = self {
            Some(Vec::new())
        } else if let Value::Cons(left, right) = self {
            let mut list = vec![left.clone()];
            list.extend(right.as_list()?);
            Some(list)
        } else {
            None
        }
    }

    pub fn iter_cons(self: Rc<Self>) -> ConsIter {
        ConsIter::from_cons(self)
    }

    pub fn as_symbol(&self) -> Option<&str> {
        if let Value::Symbol(sym) = self {
            Some(sym)
        } else {
            None
        }
    }

    pub fn as_pair(&self) -> Option<(Rc<Value>, Rc<Value>)> {
        if let Value::Cons(left, right) = self.deref() {
            if let Value::Cons(right, nil) = right.deref() {
                if let Value::Nil = nil.deref() {
                    return Some((left.clone(), right.clone()));
                }
            }
        }

        None
    }

    pub fn as_symbol_value_pair(&self) -> Option<(&str, Rc<Value>)> {
        if let Value::Cons(left, right) = self {
            if let Value::Cons(right, nil) = right.deref() {
                if let Value::Nil = nil.deref() {
                    if let Value::Symbol(symbol) = left.deref() {
                        return Some((&symbol, right.clone()));
                    }
                }
            }
        }

        None
    }

    pub fn as_string(self) -> String {
        match self {
            Value::String(s) => s,
            _ => panic!("Expected string, got: {:?}", self)
        }
    }

    pub fn as_keyword_symbol(&self) -> Option<&str> {
        if let Value::Symbol(sym) = self {
            if sym.starts_with(":") {
                return Some(&sym[1..]);
            }
        }

        None
    }

    pub fn progn(body: impl Into<Rc<Value>>) -> Value {
        Value::Cons(Rc::new(Value::symbol("progn")),
                    body.into())
    }

    pub fn symbol(symbol: &str) -> Value {
        Value::Symbol(symbol.to_string())
    }


    pub fn eval(&self, scope: &Rc<Scope>) -> Value {
        match self {
            Value::Symbol(sym) => scope.get_value(sym),
            Value::Cons(left, params) => {
                let left = left.eval(scope);

                left.call(params.clone(), scope)
            },
            _ => self.clone()
        }
    }


    pub fn call(&self, args: Rc<Value>, scope: &Rc<Scope>) -> Value {
        use self::Value::*;

        match self {
            Nil => panic!("Cannot call nil function"),
            NativeFunction(_name, func) => {
                let args = args.iter_cons()
                    .map(|e| e.eval(&scope))
                    .collect();
                func(args)
            },
            NativeMacro(_name, func) => {
                func(args.as_list().expect("Unable to evaluate improper list"),
                     scope.clone())
            },
            Function(func) => {
                let args = args.iter_cons()
                    .map(|e| e.eval(scope))
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

impl From<Vec<Rc<Value>>> for Value {
    fn from(list: Vec<Rc<Value>>) -> Self {
        Value::list_rc(list.into_iter())
    }
}

impl From<Vec<Value>> for Value {
    fn from(list: Vec<Value>) -> Self {
        Value::list(list.into_iter())
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
                if *left.deref() == Value::symbol("quote") {
                    if let Some(list) = right.as_list() {
                        if list.len() == 1 {
                            return write!(f, "'{}", list[0]);
                        }
                    }
                }

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

pub struct ConsIter {
    cons: Option<Rc<Value>>
}

impl ConsIter {
    fn from_cons(value: Rc<Value>) -> Self {
        if let Value::Cons(_, _) = value.deref() {
            ConsIter { cons: Some(value) }
        } else if let Value::Nil = value.deref() {
            ConsIter { cons: None }
        } else {
            panic!("Not a list");
        }
    }
}

impl Iterator for ConsIter {
    type Item = Rc<Value>;

    // next() is the only required method
    fn next(&mut self) -> Option<Rc<Value>> {
        if let Some(cons) = self.cons.take() {
            if let Value::Cons(left, right) = cons.deref() {
                match right.deref() {
                    Value::Cons(_, _) => {
                        self.cons = Some(right.clone());
                        Some(left.clone())
                    },
                    Value::Nil => {
                        self.cons = None;
                        Some(left.clone())
                    },
                    _ => {
                        self.cons = None;
                        Some(cons.clone())
                    }
                }
            } else {
                unreachable!("Should only be iterating cons");
            }
        } else {
            None
        }
    }
}
