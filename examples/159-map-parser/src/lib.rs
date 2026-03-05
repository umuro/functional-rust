// Example 159: Map Parser
// map: transform parser output functorially

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

fn many1<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut rem) = parser(input)?;
        let mut results = vec![first];
        while let Ok((v, rest)) = parser(rem) {
            results.push(v);
            rem = rest;
        }
        Ok((results, rem))
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
// Approach 1: map — transform parsed value
// ============================================================

fn map<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> B + 'a {
    Box::new(move |input: &'a str| {
        let (value, rest) = parser(input)?;
        Ok((f(value), rest))
    })
}

// ============================================================
// Approach 2: map2 — combine two parser results
// ============================================================

fn map2<'a, A: 'a, B: 'a, C: 'a, F>(
    p1: Parser<'a, A>, p2: Parser<'a, B>, f: F,
) -> Parser<'a, C>
where F: Fn(A, B) -> C + 'a {
    Box::new(move |input: &'a str| {
        let (v1, rest) = p1(input)?;
        let (v2, rem) = p2(rest)?;
        Ok((f(v1, v2), rem))
    })
}

// ============================================================
// Approach 3: map_const — ignore result, return fixed value
// ============================================================

fn map_const<'a, A: 'a, B: Clone + 'a>(parser: Parser<'a, A>, value: B) -> Parser<'a, B> {
    Box::new(move |input: &'a str| {
        let (_, rest) = parser(input)?;
        Ok((value.clone(), rest))
    })
}

/// Parse natural number: one or more digits → u64
fn parse_nat<'a>() -> Parser<'a, u64> {
    map(
        many1(satisfy(|c| c.is_ascii_digit(), "digit")),
        |digits| digits.iter().fold(0u64, |acc, &d| acc * 10 + (d as u64 - '0' as u64)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_uppercase() {
        let p = map(satisfy(|c| c.is_ascii_lowercase(), "lower"), |c| c.to_ascii_uppercase());
        assert_eq!(p("abc"), Ok(('A', "bc")));
    }

    #[test]
    fn test_map_preserves_error() {
        let p = map(satisfy(|c| c.is_ascii_digit(), "digit"), |c| c as u32);
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_parse_nat() {
        let p = parse_nat();
        assert_eq!(p("42rest"), Ok((42, "rest")));
    }

    #[test]
    fn test_parse_nat_zero() {
        let p = parse_nat();
        assert_eq!(p("0"), Ok((0, "")));
    }

    #[test]
    fn test_parse_nat_fail() {
        let p = parse_nat();
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_map2() {
        let p = map2(
            satisfy(|c| c.is_ascii_digit(), "digit"),
            satisfy(|c| c.is_ascii_digit(), "digit"),
            |a, b| format!("{}{}", a, b),
        );
        assert_eq!(p("12x"), Ok(("12".to_string(), "x")));
    }

    #[test]
    fn test_map_const_true() {
        let p = map_const(tag("true"), true);
        assert_eq!(p("true!"), Ok((true, "!")));
    }

    #[test]
    fn test_map_const_false() {
        let p = map_const(tag("false"), false);
        assert_eq!(p("false"), Ok((false, "")));
    }

    #[test]
    fn test_map_chain() {
        // map(map(p, f), g) == map(p, |x| g(f(x)))
        let p = map(
            map(satisfy(|c| c.is_ascii_digit(), "digit"), |c| c as u32 - '0' as u32),
            |n| n * 2,
        );
        assert_eq!(p("5"), Ok((10, "")));
    }
}
