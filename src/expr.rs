use std::fmt;
use value::Value;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Float(f64),
    Integer(i64),
    String(String),
    Symbol(String),
    Sexpr(Vec<Expr>),
    TemplateExpr(Box<Expr>),
    TemplateListExpr(Box<Expr>)
}

impl Expr {
    pub fn as_symbol(self) -> Option<String> {
        if let Expr::Symbol(sym) = self {
            Some(sym)
        } else {
            None
        }
    }

    pub fn as_sexpr(self) -> Option<Vec<Expr>> {
        if let Expr::Sexpr(expressions) = self {
            Some(expressions)
        } else {
            None
        }
    }

    pub fn as_pair(self) -> Option<(Expr, Expr)> {
        if let Expr::Sexpr(expressions) = self {
            if expressions.len() == 2 {
                let mut iter = expressions.into_iter();
                return Some((iter.next().unwrap(),
                             iter.next().unwrap()))
            }
        }

        None
    }

    pub fn as_symbol_value_pair(self) -> Option<(String, Expr)> {
        if let Some((left, right)) = self.as_pair() {
            if let Some(symbol) = left.as_symbol() {
                return Some((symbol, right));
            }
        }

        None
    }

    pub fn as_string(self) -> Option<String> {
        if let Expr::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn into_value(self) -> Value {
        match self {
            Expr::Integer(i) => Value::Integer(i),
            Expr::Float(f) => Value::Float(f),
            Expr::String(s) => Value::String(s),
            Expr::Symbol(sym) => Value::Symbol(sym),
            Expr::Sexpr(exprs) => {
                Value::list(exprs.into_iter().map(|e| e.into_value()))
            },
            Expr::TemplateExpr(_) | Expr::TemplateListExpr(_) => {
                panic!("Comma not inside backquote");
            }
        }
    }

    pub fn progn(expressions: Vec<Expr>) -> Expr {
        let mut sexpr = vec![Expr::Symbol("progn".to_string())];
        sexpr.extend(expressions);
        Expr::Sexpr(sexpr)
    }

    pub fn quote(expr: Expr) -> Expr {
        Expr::Sexpr(vec![Expr::Symbol("quote".to_string()), expr])
    }

    pub fn template(expr: Expr) -> Expr {
        match expr {
            Expr::Symbol(s) => Expr::quote(Expr::Symbol(s)),
            Expr::Sexpr(children) => {
                let mut sexpr = vec![Expr::symbol("append")];

                for child in children.into_iter() {
                    let child = if let Expr::TemplateListExpr(e) = child {
                        e.as_ref().clone()
                    } else {
                        Expr::Sexpr(vec![Expr::symbol("list"),
                                         Expr::template(child)])
                    };
                    sexpr.push(child);
                }

                sexpr.push(Expr::symbol("nil"));

                Expr::Sexpr(sexpr)
            },
            Expr::TemplateExpr(e) => e.as_ref().clone(),
            Expr::TemplateListExpr(_) => {
                panic!("Cannot expand list at top level of template")
            },
            _ => expr
        }
    }

    pub fn symbol(symbol: &str) -> Expr {
        Expr::Symbol(symbol.to_string())
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;

        match self {
            Integer(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            String(s) => write!(f, "{:?}", s),
            Symbol(s) => write!(f, "{}", s),
            Sexpr(expressions) => {
                if expressions.len() == 2 && expressions[0] == Expr::Symbol("quote".to_string()) {

                    write!(f, "'{}", expressions[1])
                } else {
                    write!(f, "({})", expressions.into_iter()
                           .map(|e| format!("{}", e)).join(" "))
                }
            },
            Expr::TemplateExpr(e) => write!(f, ",{}", e),
            Expr::TemplateListExpr(e) => write!(f, ",@{}", e)
        }
    }
}
