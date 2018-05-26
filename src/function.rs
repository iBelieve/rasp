use std::rc::Rc;
use std::collections::HashMap;
use itertools::Itertools;
use expr::Expr;
use value::Value;
use scope::Scope;


#[derive(PartialEq)]
pub struct Params {
    required_params: Vec<String>,
    optional_params: Vec<(String, Expr)>,
    keyword_params: Vec<(String, Option<Expr>)>,
    rest_param: Option<String>,
}

impl Params {
    pub fn parse(sexpr: Expr) -> Params {
        let params = sexpr.as_sexpr().expect("Expected parameter list");

        let mut required_params = Vec::new();
        let mut optional_params = Vec::new();
        let mut keyword_params = HashMap::new();
        let mut rest_param = None;

        let mut iter = params.into_iter();

        while let Some(param) = iter.next() {
            let (name, expr) = if let Some((symbol, value)) = param.clone().as_symbol_value_pair() {
                (symbol, Some(value))
            } else if let Some (symbol) = param.clone().as_symbol() {
                (symbol, None)
            } else {
                panic!("Expected paramater or parameter and default value, got: {:?}", param);
            };

            if name.starts_with(":") {
                keyword_params.insert(name[1..].to_string(), expr);
            } else if !keyword_params.is_empty() {
                panic!("Keyword parameters must be defined after positional parameters");
            } else if name.starts_with("...") {
                if rest_param.is_some() {
                    panic!("Only one rest parameter may be defined");
                } else {
                    rest_param = Some(name[3..].to_string());
                }
            } else if rest_param.is_some() {
                panic!("The rest parameter must be at the end of positional parameters");
            } else if let Some(expr) = expr {
                optional_params.push((name, expr));
            } else if !optional_params.is_empty() {
                panic!("Optional parameters must be defined after optional parameters");
            } else {
                required_params.push(name);
            }
        }

        Params {
            required_params,
            optional_params,
            keyword_params: keyword_params.into_iter().collect(),
            rest_param
        }
    }

    pub fn apply(&self, scope: Rc<Scope>, args: Vec<Value>) {
        let mut iter = args.into_iter();

        let mut required_args = Vec::new();
        let mut optional_args = Vec::new();
        let mut keyword_args = HashMap::new();
        let mut rest_args = Vec::new();

        while let Some(arg) = iter.next() {
            if let Some(name) = arg.as_keyword_symbol() {
                if keyword_args.contains_key(&name) {
                    panic!("Duplicate keyword argument: {}", name);
                } else {
                    let value = iter.next().expect("Keyword argument missing value");
                    keyword_args.insert(name, value);
                }
            } else if !keyword_args.is_empty() {
                panic!("Unexpected value after keyword argument: {:?}", arg);
            } else if required_args.len() < self.required_params.len() {
                required_args.push(arg);
            } else if optional_args.len() < self.optional_params.len() {
                optional_args.push(arg);
            } else if self.rest_param.is_some() {
                rest_args.push(arg);
            } else {
                panic!("Unexpected additional argument: {:?}", arg);
            }
        }

        if required_args.len() < self.required_params.len() {
            let mut missing_params = self.required_params.iter().skip(required_args.len());
            panic!("Missing required arguments: {}", missing_params.join(", "));
        }

        let optional_args_count = optional_args.len();

        for (name, value) in self.required_params.iter().zip(required_args) {
            scope.set_value(name.to_string(), value);
        }

        for (param, value) in self.optional_params.iter().zip(optional_args) {
            let (name, _expr) = param;
            scope.set_value(name.to_string(), value);
        }

        for (name, expr) in self.optional_params.iter().skip(optional_args_count) {
            scope.set_value(name.to_string(), expr.clone().eval(scope.clone()));
        }

        if let Some(ref rest_param) = self.rest_param {
            scope.set_value(rest_param.clone(), Value::List(Rc::new(rest_args)));
        }

        for (name, expr) in &self.keyword_params {
            if !keyword_args.contains_key(name) {
                if let Some(expr) = expr {
                    keyword_args.insert(name.clone(),
                                        expr.clone().eval(scope.clone()));
                } else {
                    panic!("Mising required keyword argument: {}", name);
                }
            }
        }

        for (name, value) in keyword_args.into_iter() {
            scope.set_value(name, value);
        }
    }
}

#[derive(PartialEq)]
pub struct Function {
    pub name: String,
    params: Params,
    expr: Expr,
    parent_scope: Rc<Scope>
}

impl Function {
    pub fn define(name: String, params: Expr, body: Vec<Expr>,
                  parent_scope: Rc<Scope>) -> Self {
        Function {
            name,
            params: Params::parse(params),
            expr: Expr::progn(body),
            parent_scope
        }
    }

    pub fn call(&self, args: Vec<Value>) -> Value {
        let scope = self.parent_scope.clone().push();
        self.params.apply(scope.clone(), args);
        self.expr.clone().eval(scope)
    }
}
