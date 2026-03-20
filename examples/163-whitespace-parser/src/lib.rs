#![allow(clippy::all)]
// Example 163: Whitespace Parser
// Parse and skip whitespace: ws0, ws1, ws_wrap

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

// ============================================================
// Approach 1: ws0 — skip zero or more whitespace (always succeeds)
// ============================================================

fn ws0<'a>() -> Parser<'a, ()> {
    Box::new(|input: &'a str| {
        let trimmed = input.trim_start();
        Ok(((), trimmed))
    })
}

// ============================================================
// Approach 2: ws1 — require at least one whitespace
// ============================================================

fn ws1<'a>() -> Parser<'a, ()> {
    Box::new(|input: &'a str| match input.chars().next() {
        Some(c) if c.is_ascii_whitespace() => {
            let trimmed = input.trim_start();
            Ok(((), trimmed))
        }
        _ => Err("Expected whitespace".to_string()),
    })
}

// ============================================================
// Approach 3: ws_wrap — parse with surrounding whitespace
// ============================================================

fn ws_wrap<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| {
        let trimmed = input.trim_start();
        let (value, rest) = parser(trimmed)?;
        let trimmed_rest = rest.trim_start();
        Ok((value, trimmed_rest))
    })
}

/// Line comment: skip from '#' to end of line
fn line_comment<'a>() -> Parser<'a, ()> {
    Box::new(|input: &'a str| {
        if input.starts_with('#') {
            match input.find('\n') {
                Some(pos) => Ok(((), &input[pos..])),
                None => Ok(((), "")),
            }
        } else {
            Err("Expected '#'".to_string())
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws0_spaces() {
        assert_eq!(ws0()("  hello"), Ok(((), "hello")));
    }

    #[test]
    fn test_ws0_no_spaces() {
        assert_eq!(ws0()("hello"), Ok(((), "hello")));
    }

    #[test]
    fn test_ws0_empty() {
        assert_eq!(ws0()(""), Ok(((), "")));
    }

    #[test]
    fn test_ws0_tabs_newlines() {
        assert_eq!(ws0()("\t\n  x"), Ok(((), "x")));
    }

    #[test]
    fn test_ws1_success() {
        assert_eq!(ws1()("  hello"), Ok(((), "hello")));
    }

    #[test]
    fn test_ws1_fail() {
        assert!(ws1()("hello").is_err());
    }

    #[test]
    fn test_ws_wrap() {
        let p = ws_wrap(tag("hello"));
        assert_eq!(p("  hello  rest"), Ok(("hello", "rest")));
    }

    #[test]
    fn test_ws_wrap_no_spaces() {
        let p = ws_wrap(tag("hello"));
        assert_eq!(p("hello"), Ok(("hello", "")));
    }

    #[test]
    fn test_line_comment() {
        assert_eq!(line_comment()("# comment\ncode"), Ok(((), "\ncode")));
    }

    #[test]
    fn test_line_comment_eof() {
        assert_eq!(line_comment()("# comment"), Ok(((), "")));
    }

    #[test]
    fn test_line_comment_not_hash() {
        assert!(line_comment()("code").is_err());
    }
}
