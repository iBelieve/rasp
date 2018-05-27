use nom::{Needed, recognize_float, digit};
use nom::types::CompleteStr;
use expr::Expr;
use template::Template;

named!(float<CompleteStr, f64>,
       flat_map!(call!(recognize_float), parse_to!(f64)));

named!(integer<CompleteStr, i64>, flat_map!(
    do_parse!(
        res: recognize!(tuple!(
            opt!(alt!(char!('+') | char!('-'))),
            value!((), digit),
            opt!(tuple!(
                alt!(char!('e') | char!('E')),
                opt!(alt!(char!('+') | char!('-'))),
                digit
            ))
        )) >>
            not!(char!('.')) >>
            (res)
    ),
    parse_to!(i64)
));

named!(escaped_symbol<CompleteStr, String>, delimited!(
    char!('|'),
    escaped_transform!(is_not!("\\|"), '\\', take!(1)),
    char!('|')
));

named!(simple_symbol<CompleteStr, String>,
       escaped_transform!(is_not!(" \t\n\r\\\"'`()#|;"), '\\', take!(1)));

named!(symbol<CompleteStr, String>, alt!(escaped_symbol | simple_symbol));

named!(string<CompleteStr, String>, delimited!(
    char!('"'),
    escaped_transform!(is_not!("\\\""), '\\', alt!(
        char!('a') => { |_| '\x07' } |
        char!('b') => { |_| '\x08' } |
        char!('f') => { |_| '\x0c' } |
        char!('n') => { |_| '\n'   } |
        char!('r') => { |_| '\r'   } |
        char!('t') => { |_| '\t'   } |
        char!('v') => { |_| '\x0b' } |
        one_of!(r#"'"?\"#)
    )),
    char!('"')
));

named!(sexpr<CompleteStr, Vec<Expr>>,
       ws!(delimited!(char!('('), many0!(expr), char!(')'))));

named!(quote<CompleteStr, Expr>, preceded!(char!('\''), expr));

named!(backquote<CompleteStr, Template>, preceded!(char!('`'), template));

named!(comma_list<CompleteStr, Template>, do_parse!(
    char!(',') >>
        char!('@') >>
        value: template >>
        (value)
));

named!(comma<CompleteStr, Template>, do_parse!(
    char!(',') >>
        not!(char!('@')) >>
        value: template >>
        (value)
));

named!(template_sexpr<CompleteStr, Vec<Template>>,
       ws!(delimited!(char!('('), many0!(template), char!(')'))));

named!(template_quote<CompleteStr, Template>, preceded!(char!('\''), template));

named!(template<CompleteStr, Template>, alt!(
    integer        => { |i| Template::Integer(i) } |
    float          => { |f| Template::Float(f) } |
    string         => { |s| Template::String(s) } |
    template_sexpr => { |e| Template::Sexpr(e) } |
    template_quote => { |e| Template::quote(e) } |
    backquote      => { |e| Template::Template(Box::new(e)) } |
    comma          => { |e| Template::TemplateExpr(Box::new(e)) } |
    comma_list     => { |e| Template::TemplateListExpr(Box::new(e)) } |
    symbol         => { |s| Template::Symbol(s) }
));


named!(expr<CompleteStr, Expr>, alt!(
    integer    => { |i| Expr::Integer(i) } |
    float      => { |f| Expr::Float(f) } |
    string     => { |s| Expr::String(s) } |
    sexpr      => { |e| Expr::Sexpr(e) } |
    quote      => { |e| Expr::quote(e) } |
    backquote  => { |t: Template| t.compile() } |
    comma      => { |_| panic!("Comma not inside backquote") } |
    comma_list => { |_| panic!("Comma not inside backquote") } |
    symbol     => { |s| Expr::Symbol(s) }
));

named!(root<CompleteStr, Vec<Expr>>, ws!(many0!(expr)));

pub fn parse(string: &str) -> Result<Vec<Expr>, String> {
    match root(CompleteStr(string)) {
        Ok((i, o)) => {
            if !i.is_empty() {
                panic!("Expected EOF, got: {}", i);
            } else {
                Ok(o)
            }
        },
        Err(e) => {
            Err(format!("{}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn parse_integer() {
        assert_eq!(
            integer(CompleteStr("12 test")),
            Result::Ok((CompleteStr(" test"), 12))
        );
    }

    #[test]
    fn parse_float() {
        assert_eq!(
            float(CompleteStr("12.4 test")),
            Result::Ok((CompleteStr(" test"), 12.4))
        );
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            string(CompleteStr(r#""test\nstring""#)),
            Result::Ok((CompleteStr(""), "test\nstring".to_string()))
        );
    }

    #[test]
    fn parse_sexpr() {
        assert_eq!(
            sexpr(CompleteStr(r#"(12.4 "string here\n")"#)),
            Result::Ok((CompleteStr(""), vec![Expr::Float(12.4),
                                              Expr::String("string here\n".to_string())]))
        );
    }

    #[test]
    fn parse_file() {
        assert_eq!(
            parse(r#"(println "{}" (+ 1 2.3))"#),
            Result::Ok(vec![Expr::Sexpr(vec![Expr::Symbol("println".to_string()),
                                             Expr::String("{}".to_string()),
                                             Expr::Sexpr(vec![Expr::Symbol("+".to_string()),
                                                              Expr::Integer(1),
                                                              Expr::Float(2.3)])])])
        );
    }

    #[test]
    fn parse_template() {
        println!("{}", parse(r#"`(println ,var)"#).unwrap()[0]);
        assert_eq!(
            parse(r#"`(println ,var)"#),
            Result::Ok(vec![Expr::Sexpr(
                vec![Expr::symbol("append"),
                     Expr::Sexpr(vec![Expr::symbol("list"),
                                      Expr::quote(Expr::symbol("println"))]),
                     Expr::Sexpr(vec![Expr::symbol("list"),
                                      Expr::symbol("var")]),
                     Expr::symbol("nil")]
            )])
        );
    }
}
