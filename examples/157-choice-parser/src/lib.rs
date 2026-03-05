// Example 157: Choice Parser
// alt / choice: try parsers in order, return first success

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

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

fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where F: Fn(char) -> bool + 'a {
    let desc = desc.to_string();
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        _ => Err(format!("Expected {}", desc)),
    })
}

// ============================================================
// Approach 1: alt — try two parsers
// ============================================================

fn alt<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match p1(input) {
        Ok(result) => Ok(result),
        Err(_) => p2(input),
    })
}

// ============================================================
// Approach 2: choice — try a list of parsers
// ============================================================

fn choice<'a, T: 'a>(parsers: Vec<Parser<'a, T>>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| {
        for parser in &parsers {
            if let Ok(result) = parser(input) {
                return Ok(result);
            }
        }
        Err("No parser matched".to_string())
    })
}

// ============================================================
// Approach 3: alt with error accumulation
// ============================================================

fn alt_err<'a, T: 'a>(p1: Parser<'a, T>, p2: Parser<'a, T>) -> Parser<'a, T> {
    Box::new(move |input: &'a str| match p1(input) {
        Ok(result) => Ok(result),
        Err(e1) => match p2(input) {
            Ok(result) => Ok(result),
            Err(e2) => Err(format!("{} or {}", e1, e2)),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alt_first() {
        let p = alt(tag("true"), tag("false"));
        assert_eq!(p("true!"), Ok(("true", "!")));
    }

    #[test]
    fn test_alt_second() {
        let p = alt(tag("true"), tag("false"));
        assert_eq!(p("false!"), Ok(("false", "!")));
    }

    #[test]
    fn test_alt_neither() {
        let p = alt(tag("true"), tag("false"));
        assert!(p("maybe").is_err());
    }

    #[test]
    fn test_choice_first() {
        let p = choice(vec![tag("a"), tag("b"), tag("c")]);
        assert_eq!(p("abc"), Ok(("a", "bc")));
    }

    #[test]
    fn test_choice_last() {
        let p = choice(vec![tag("x"), tag("y"), tag("z")]);
        assert_eq!(p("zoo"), Ok(("z", "oo")));
    }

    #[test]
    fn test_choice_none() {
        let p = choice(vec![tag("x"), tag("y")]);
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_choice_empty() {
        let p: Parser<&str> = choice(vec![]);
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_alt_err_accumulates() {
        let p = alt_err(
            satisfy(|c| c.is_ascii_digit(), "digit"),
            satisfy(|c| c.is_ascii_alphabetic(), "letter"),
        );
        let err = p("!x").unwrap_err();
        assert!(err.contains("digit"));
        assert!(err.contains("letter"));
    }
}
