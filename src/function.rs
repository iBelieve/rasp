use std::rc::Rc;
use value::Value;
use scope::Scope;
use params::Params;


#[derive(PartialEq)]
pub struct Function {
    pub name: String,
    params: Params,
    expr: Value,
    parent_scope: Rc<Scope>
}

impl Function {
    pub fn define(name: String, params: &Value, body: Rc<Value>,
                  parent_scope: Rc<Scope>) -> Self {
        Function {
            name,
            params: Params::parse(params),
            expr: Value::progn(body),
            parent_scope
        }
    }

    pub fn call(&self, args: Vec<Value>) -> Value {
        let scope = self.parent_scope.clone().push();
        self.params.apply(&scope, args);
        self.expr.clone().eval(&scope)
    }
}

#[derive(PartialEq)]
pub struct Macro {
    pub name: String,
    params: Params,
    expr: Value,
    parent_scope: Rc<Scope>
}

impl Macro {
    pub fn define(name: String, params: &Value, body: Rc<Value>,
                  parent_scope: Rc<Scope>) -> Self {
        Macro {
            name,
            params: Params::parse(params),
            expr: Value::progn(body),
            parent_scope
        }
    }

    pub fn call(&self, args: Vec<Value>) -> Value {
        let scope = self.parent_scope.clone().push();
        self.params.apply(&scope, args);
        self.expr.clone().eval(&scope)
    }
}
