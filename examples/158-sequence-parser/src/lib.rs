// Example 158: Sequence Parser
// pair, preceded, terminated, delimited: sequence combinators

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where F: Fn(char) -> bool + 'a {
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
// Approach 1: pair — run two parsers, return both results
// ============================================================

fn pair<'a, A: 'a, B: 'a>(p1: Parser<'a, A>, p2: Parser<'a, B>) -> Parser<'a, (A, B)> {
    Box::new(move |input: &'a str| {
        let (v1, rest) = p1(input)?;
        let (v2, remaining) = p2(rest)?;
        Ok(((v1, v2), remaining))
    })
}

// ============================================================
// Approach 2: preceded, terminated — discard one side
// ============================================================

fn preceded<'a, A: 'a, B: 'a>(prefix: Parser<'a, A>, p: Parser<'a, B>) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, rest) = prefix(input)?;
        p(rest)
    })
}

fn terminated<'a, A: 'a, B: 'a>(p: Parser<'a, A>, suffix: Parser<'a, B>) -> Parser<'a, A> {
    Box::new(move |input: &'a str| {
        let (value, rest) = p(input)?;
        let (_, remaining) = suffix(rest)?;
        Ok((value, remaining))
    })
}

// ============================================================
// Approach 3: delimited — discard both sides, keep middle
// ============================================================

fn delimited<'a, A: 'a, B: 'a, C: 'a>(
    open: Parser<'a, A>,
    p: Parser<'a, B>,
    close: Parser<'a, C>,
) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, r1) = open(input)?;
        let (value, r2) = p(r1)?;
        let (_, r3) = close(r2)?;
        Ok((value, r3))
    })
}

/// Triple — three parsers in sequence
fn triple<'a, A: 'a, B: 'a, C: 'a>(
    p1: Parser<'a, A>,
    p2: Parser<'a, B>,
    p3: Parser<'a, C>,
) -> Parser<'a, (A, B, C)> {
    Box::new(move |input: &'a str| {
        let (v1, r1) = p1(input)?;
        let (v2, r2) = p2(r1)?;
        let (v3, r3) = p3(r2)?;
        Ok(((v1, v2, v3), r3))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair() {
        let p = pair(
            satisfy(|c| c.is_ascii_lowercase(), "letter"),
            satisfy(|c| c.is_ascii_digit(), "digit"),
        );
        assert_eq!(p("a1"), Ok((('a', '1'), "")));
    }

    #[test]
    fn test_pair_fail_first() {
        let p = pair(
            satisfy(|c| c.is_ascii_lowercase(), "letter"),
            satisfy(|c| c.is_ascii_digit(), "digit"),
        );
        assert!(p("1a").is_err());
    }

    #[test]
    fn test_preceded() {
        let p = preceded(tag("("), satisfy(|c| c.is_ascii_lowercase(), "letter"));
        assert_eq!(p("(a)"), Ok(('a', ")")));
    }

    #[test]
    fn test_terminated() {
        let p = terminated(satisfy(|c| c.is_ascii_lowercase(), "letter"), tag(";"));
        assert_eq!(p("a;rest"), Ok(('a', "rest")));
    }

    #[test]
    fn test_delimited() {
        let p = delimited(
            tag("("),
            satisfy(|c| c.is_ascii_lowercase(), "letter"),
            tag(")"),
        );
        assert_eq!(p("(x)rest"), Ok(('x', "rest")));
    }

    #[test]
    fn test_delimited_fail_close() {
        let p = delimited(
            tag("("),
            satisfy(|c| c.is_ascii_lowercase(), "letter"),
            tag(")"),
        );
        assert!(p("(x]").is_err());
    }

    #[test]
    fn test_triple() {
        let p = triple(
            satisfy(|c| c.is_ascii_lowercase(), "letter"),
            satisfy(|c| c.is_ascii_digit(), "digit"),
            satisfy(|c| c.is_ascii_lowercase(), "letter"),
        );
        assert_eq!(p("a1b"), Ok((('a', '1', 'b'), "")));
    }
}
