#![allow(clippy::result_unit_err)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
// Example 156: Optional Parser
// opt: make a parser optional, returns Option<T>

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,
{
    let desc = desc.to_string();
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        _ => Err(format!("Expected {}", desc)),
    })
}

fn tag<'a>(expected: &str) -> Parser<'a, &'a str> {
    let exp = expected.to_string();
    Box::new(move |input: &'a str| {
        if input.starts_with(&exp) {
            Ok((&input[..exp.len()], &input[exp.len()..]))
        } else {
            Err(format!("Expected \"{}\"", exp))
        }
    })
}

// ============================================================
// Approach 1: opt — wrap result in Option, always succeeds
// ============================================================

fn opt<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok((value, rest)) => Ok((Some(value), rest)),
        Err(_) => Ok((None, input)),
    })
}

// ============================================================
// Approach 2: with_default — provide a fallback value
// ============================================================

fn with_default<'a, T: Clone + 'a>(default: T, parser: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok(result) => Ok(result),
        Err(_) => Ok((default.clone(), input)),
    })
}

// ============================================================
// Approach 3: peek — check without consuming
// ============================================================

fn peek<'a, T: Clone + 'a>(parser: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match parser(input) {
        Ok((value, _)) => Ok((Some(value), input)), // don't advance!
        Err(_) => Ok((None, input)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_some() {
        let p = opt(tag("+"));
        let (val, rest) = p("+42").unwrap();
        assert_eq!(val, Some("+"));
        assert_eq!(rest, "42");
    }

    #[test]
    fn test_opt_none() {
        let p = opt(tag("+"));
        let (val, rest) = p("42").unwrap();
        assert_eq!(val, None);
        assert_eq!(rest, "42");
    }

    #[test]
    fn test_opt_always_succeeds() {
        let p = opt(tag("xyz"));
        assert!(p("abc").is_ok());
        assert!(p("").is_ok());
    }

    #[test]
    fn test_with_default_present() {
        let p = with_default('+', satisfy(|c| c == '+' || c == '-', "sign"));
        assert_eq!(p("-5"), Ok(('-', "5")));
    }

    #[test]
    fn test_with_default_absent() {
        let p = with_default('+', satisfy(|c| c == '+' || c == '-', "sign"));
        assert_eq!(p("5"), Ok(('+', "5")));
    }

    #[test]
    fn test_peek_success_no_consume() {
        let p = peek(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (val, rest) = p("123").unwrap();
        assert_eq!(val, Some('1'));
        assert_eq!(rest, "123"); // NOT consumed
    }

    #[test]
    fn test_peek_failure() {
        let p = peek(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (val, rest) = p("abc").unwrap();
        assert_eq!(val, None);
        assert_eq!(rest, "abc");
    }
}
