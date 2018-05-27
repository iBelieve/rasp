use expr::Expr;

#[derive(Clone)]
pub enum Template {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Symbol(String),
    Sexpr(Vec<Template>),
    TemplateExpr(Box<Template>),
    TemplateListExpr(Box<Template>),
    Template(Box<Template>)
}

impl Template {
    pub fn compile(self) -> Expr {
        self.compile_root().convert()
    }

    pub fn convert(self) -> Expr {
        match self {
            Template::Integer(i) => Expr::Integer(i),
            Template::Float(f) => Expr::Float(f),
            Template::Boolean(b) => Expr::Boolean(b),
            Template::String(s) => Expr::String(s),
            Template::Symbol(s) => Expr::Symbol(s),
            Template::Sexpr(children) => {
               Expr::Sexpr(children.into_iter().map(|t| t.convert()).collect())
            },
            Template::TemplateExpr(_) => {
                panic!("Comma not inside backquote")
            },
            Template::TemplateListExpr(_) => {
                panic!("Comma not inside backquote");
            },
            Template::Template(_) => {
                unreachable!("All templates should have been compiled");
            }
        }
    }

    pub fn compile_root(self) -> Template {
        match self {
            Template::Symbol(s) => Template::quote(Template::Symbol(s)),
            Template::Sexpr(children) => {
                let mut sexpr = vec![Template::Symbol("append".to_string())];

                for child in children.into_iter() {
                    sexpr.push(child.compile_in_sexpr());
                }

                sexpr.push(Template::Symbol("nil".to_string()));

                Template::Sexpr(sexpr)
            },
            Template::TemplateExpr(e) => e.as_ref().clone(),
            Template::TemplateListExpr(_) => {
                panic!("Cannot expand list at top level of template");
            }
            Template::Template(e) => e.compile_root(),
            _ => self
        }
    }

    fn compile_in_sexpr(self) -> Template {
        if let Template::TemplateListExpr(e) = self {
            e.as_ref().clone()
        } else {
            Template::Sexpr(vec![Template::Symbol("list".to_string()),
                                 self.compile_root()])
        }
    }

    pub fn quote(expr: Template) -> Template {
        Template::Sexpr(vec![Template::Symbol("quote".to_string()), expr])
    }
}
