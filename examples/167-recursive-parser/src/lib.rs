// Example 167: Recursive Parser
// Recursive parsers using function pointers and Rc for recursive grammars

use std::rc::Rc;

type ParseResult<'a, T> = Result<(T, &'a str), String>;

// Recursive data type
#[derive(Debug, Clone, PartialEq)]
enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

// ============================================================
// Approach 1: Direct recursive functions (simplest)
// ============================================================

fn parse_sexp(input: &str) -> ParseResult<Sexp> {
    // Try atom first
    if let Some(c) = input.chars().next() {
        if c.is_ascii_lowercase() {
            let end = input
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .count();
            let byte_end: usize = input.chars().take(end).map(|c| c.len_utf8()).sum();
            return Ok((
                Sexp::Atom(input[..byte_end].to_string()),
                &input[byte_end..],
            ));
        }
    }
    // Try list
    parse_sexp_list(input)
}

fn parse_sexp_list(input: &str) -> ParseResult<Sexp> {
    if !input.starts_with('(') {
        return Err("Expected '('".to_string());
    }
    let mut remaining = &input[1..];
    let mut items = Vec::new();
    loop {
        remaining = remaining.trim_start();
        if remaining.starts_with(')') {
            return Ok((Sexp::List(items), &remaining[1..]));
        }
        let (item, rest) = parse_sexp(remaining)?;
        items.push(item);
        remaining = rest;
    }
}

// ============================================================
// Approach 2: Rc-based recursive parser type
// ============================================================

type RcParser<'a, T> = Rc<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn rc_fix<'a, T: 'a>(f: impl Fn(RcParser<'a, T>) -> RcParser<'a, T> + 'a) -> RcParser<'a, T> {
    use std::cell::RefCell;
    let parser: Rc<RefCell<Option<RcParser<'a, T>>>> = Rc::new(RefCell::new(None));
    let parser_clone = parser.clone();
    let lazy: RcParser<'a, T> = Rc::new(move |input: &'a str| {
        let p = parser_clone.borrow();
        (p.as_ref().unwrap())(input)
    });
    let actual = f(lazy);
    *parser.borrow_mut() = Some(actual.clone());
    actual
}

// ============================================================
// Approach 3: Nested parentheses counter using fix
// ============================================================

fn nested_parens(input: &str) -> ParseResult<usize> {
    if input.starts_with('(') {
        let (depth, rest) = nested_parens(&input[1..])?;
        if rest.starts_with(')') {
            Ok((depth + 1, &rest[1..]))
        } else {
            Err("Expected ')'".to_string())
        }
    } else {
        Ok((0, input)) // base case
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
    fn test_flat_list() {
        let (sexp, rest) = parse_sexp("(a b c)").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            sexp,
            Sexp::List(vec![
                Sexp::Atom("a".into()),
                Sexp::Atom("b".into()),
                Sexp::Atom("c".into()),
            ])
        );
    }

    #[test]
    fn test_nested_list() {
        let (sexp, _) = parse_sexp("(a (b c))").unwrap();
        assert_eq!(
            sexp,
            Sexp::List(vec![
                Sexp::Atom("a".into()),
                Sexp::List(vec![Sexp::Atom("b".into()), Sexp::Atom("c".into())]),
            ])
        );
    }

    #[test]
    fn test_nested_parens_3() {
        assert_eq!(nested_parens("((()))"), Ok((3, "")));
    }

    #[test]
    fn test_nested_parens_1() {
        assert_eq!(nested_parens("()"), Ok((1, "")));
    }

    #[test]
    fn test_nested_parens_0() {
        assert_eq!(nested_parens(""), Ok((0, "")));
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(parse_sexp("()"), Ok((Sexp::List(vec![]), "")));
    }

    #[test]
    fn test_rc_parser() {
        let p: RcParser<Sexp> = rc_fix(|self_p: RcParser<Sexp>| {
            Rc::new(move |input: &str| {
                if let Some(c) = input.chars().next() {
                    if c.is_ascii_lowercase() {
                        let end: usize = input
                            .chars()
                            .take_while(|c| c.is_ascii_alphanumeric())
                            .map(|c| c.len_utf8())
                            .sum();
                        return Ok((Sexp::Atom(input[..end].to_string()), &input[end..]));
                    }
                }
                if !input.starts_with('(') {
                    return Err("Expected".into());
                }
                let mut rem = &input[1..];
                let mut items = Vec::new();
                loop {
                    rem = rem.trim_start();
                    if rem.starts_with(')') {
                        return Ok((Sexp::List(items), &rem[1..]));
                    }
                    let (item, rest) = self_p(rem)?;
                    items.push(item);
                    rem = rest;
                }
            })
        });
        assert_eq!(
            p("(a b)"),
            Ok((
                Sexp::List(vec![Sexp::Atom("a".into()), Sexp::Atom("b".into())]),
                ""
            ))
        );
    }
}
