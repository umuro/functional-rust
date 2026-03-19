#![allow(clippy::all)]
// Example 173: Lisp / S-expression Parser
// S-expressions: atoms, numbers, strings, lists, quote

type ParseResult<'a, T> = Result<(T, &'a str), String>;

#[derive(Debug, Clone, PartialEq)]
enum Sexp {
    Atom(String),
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    List(Vec<Sexp>),
}

impl std::fmt::Display for Sexp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sexp::Atom(s) => write!(f, "{}", s),
            Sexp::Number(n) => write!(f, "{}", n),
            Sexp::Str(s) => write!(f, "\"{}\"", s),
            Sexp::Bool(true) => write!(f, "#t"),
            Sexp::Bool(false) => write!(f, "#f"),
            Sexp::Nil => write!(f, "nil"),
            Sexp::List(items) => {
                write!(f, "(")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ")")
            }
        }
    }
}

fn is_atom_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "_-+*/?!.#".contains(c)
}

// ============================================================
// Approach 1: Parse atom / number / boolean / nil
// ============================================================

fn parse_atom(input: &str) -> ParseResult<Sexp> {
    let s = input.trim_start();
    let end = s.find(|c: char| !is_atom_char(c)).unwrap_or(s.len());
    if end == 0 {
        return Err("Expected atom".to_string());
    }
    let token = &s[..end];
    let rest = &s[end..];
    let sexp = match token {
        "nil" => Sexp::Nil,
        "#t" | "true" => Sexp::Bool(true),
        "#f" | "false" => Sexp::Bool(false),
        _ => match token.parse::<f64>() {
            Ok(n) => Sexp::Number(n),
            Err(_) => Sexp::Atom(token.to_string()),
        },
    };
    Ok((sexp, rest))
}

// ============================================================
// Approach 2: Parse string with escape sequences
// ============================================================

fn parse_string(input: &str) -> ParseResult<Sexp> {
    let s = input.trim_start();
    if !s.starts_with('"') {
        return Err("Expected string".to_string());
    }
    let mut result = String::new();
    let mut chars = s[1..].chars();
    let mut consumed = 1;
    loop {
        match chars.next() {
            None => return Err("Unterminated string".to_string()),
            Some('"') => {
                consumed += 1;
                return Ok((Sexp::Str(result), &s[consumed..]));
            }
            Some('\\') => {
                consumed += 1;
                match chars.next() {
                    Some('n') => {
                        result.push('\n');
                        consumed += 1;
                    }
                    Some('t') => {
                        result.push('\t');
                        consumed += 1;
                    }
                    Some('"') => {
                        result.push('"');
                        consumed += 1;
                    }
                    Some('\\') => {
                        result.push('\\');
                        consumed += 1;
                    }
                    Some(c) => {
                        result.push('\\');
                        result.push(c);
                        consumed += c.len_utf8();
                    }
                    None => return Err("Unexpected EOF in escape".to_string()),
                }
            }
            Some(c) => {
                result.push(c);
                consumed += c.len_utf8();
            }
        }
    }
}

// ============================================================
// Approach 3: Full S-expression parser
// ============================================================

fn parse_sexp(input: &str) -> ParseResult<Sexp> {
    let s = input.trim_start();
    if s.is_empty() {
        return Err("Unexpected EOF".to_string());
    }
    match s.chars().next().unwrap() {
        '(' => parse_list(s),
        '\'' => {
            let (val, rest) = parse_sexp(&s[1..])?;
            Ok((Sexp::List(vec![Sexp::Atom("quote".into()), val]), rest))
        }
        '"' => parse_string(s),
        _ => parse_atom(s),
    }
}

fn parse_list(input: &str) -> ParseResult<Sexp> {
    let mut remaining = &input[1..]; // skip '('
    let mut items = Vec::new();
    loop {
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            return Err("Unterminated list".to_string());
        }
        if remaining.starts_with(')') {
            return Ok((Sexp::List(items), &remaining[1..]));
        }
        let (item, rest) = parse_sexp(remaining)?;
        items.push(item);
        remaining = rest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        assert_eq!(parse_sexp("hello"), Ok((Sexp::Atom("hello".into()), "")));
    }

    #[test]
    fn test_number() {
        assert_eq!(parse_sexp("42"), Ok((Sexp::Number(42.0), "")));
    }

    #[test]
    fn test_float() {
        assert_eq!(parse_sexp("3.14"), Ok((Sexp::Number(3.14), "")));
    }

    #[test]
    fn test_string() {
        assert_eq!(parse_sexp("\"hi\""), Ok((Sexp::Str("hi".into()), "")));
    }

    #[test]
    fn test_string_escape() {
        let (sexp, _) = parse_sexp("\"hello\\nworld\"").unwrap();
        assert_eq!(sexp, Sexp::Str("hello\nworld".into()));
    }

    #[test]
    fn test_nil() {
        assert_eq!(parse_sexp("nil"), Ok((Sexp::Nil, "")));
    }

    #[test]
    fn test_bool() {
        assert_eq!(parse_sexp("#t"), Ok((Sexp::Bool(true), "")));
        assert_eq!(parse_sexp("#f"), Ok((Sexp::Bool(false), "")));
    }

    #[test]
    fn test_list() {
        let (sexp, _) = parse_sexp("(+ 1 2)").unwrap();
        assert_eq!(
            sexp,
            Sexp::List(vec![
                Sexp::Atom("+".into()),
                Sexp::Number(1.0),
                Sexp::Number(2.0),
            ])
        );
    }

    #[test]
    fn test_nested() {
        let (sexp, _) = parse_sexp("(define (f x) (* x x))").unwrap();
        match sexp {
            Sexp::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], Sexp::Atom("define".into()));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_quote() {
        let (sexp, _) = parse_sexp("'hello").unwrap();
        assert_eq!(
            sexp,
            Sexp::List(vec![Sexp::Atom("quote".into()), Sexp::Atom("hello".into())])
        );
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(parse_sexp("()"), Ok((Sexp::List(vec![]), "")));
    }

    #[test]
    fn test_display() {
        let (sexp, _) = parse_sexp("(+ 1 2)").unwrap();
        assert_eq!(format!("{}", sexp), "(+ 1 2)");
    }
}
