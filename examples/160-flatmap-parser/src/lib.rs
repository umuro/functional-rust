// Example 160: FlatMap Parser
// flat_map / and_then: monadic chaining of parsers

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
        while let Ok((v, rest)) = parser(rem) { results.push(v); rem = rest; }
        Ok((results, rem))
    })
}

fn map<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> B + 'a {
    Box::new(move |input: &'a str| {
        let (v, rest) = parser(input)?;
        Ok((f(v), rest))
    })
}

// ============================================================
// Approach 1: and_then / bind — monadic chaining
// ============================================================

fn and_then<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> Parser<'a, B> + 'a {
    Box::new(move |input: &'a str| {
        let (value, rest) = parser(input)?;
        (f(value))(rest)
    })
}

// ============================================================
// Approach 2: Context-sensitive — length-prefixed string "3:abc"
// ============================================================

fn parse_nat<'a>() -> Parser<'a, usize> {
    map(
        many1(satisfy(|c| c.is_ascii_digit(), "digit")),
        |digits| digits.iter().fold(0usize, |acc, &d| acc * 10 + (d as usize - '0' as usize)),
    )
}

fn length_prefixed<'a>() -> Parser<'a, &'a str> {
    and_then(parse_nat(), |n| {
        Box::new(move |input: &'a str| {
            if input.starts_with(':') {
                let rest = &input[1..];
                if rest.len() >= n {
                    Ok((&rest[..n], &rest[n..]))
                } else {
                    Err("Not enough characters".to_string())
                }
            } else {
                Err("Expected ':'".to_string())
            }
        })
    })
}

// ============================================================
// Approach 3: Conditional parsing based on tag
// ============================================================

fn conditional_parser<'a>() -> Parser<'a, String> {
    and_then(
        satisfy(|c| c == 'i' || c == 's', "type tag"),
        |tag_char| {
            if tag_char == 'i' {
                map(
                    many1(satisfy(|c| c.is_ascii_digit(), "digit")),
                    |chars| chars.into_iter().collect(),
                )
            } else {
                map(
                    many1(satisfy(|c| c.is_ascii_lowercase(), "letter")),
                    |chars| chars.into_iter().collect(),
                )
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_then_basic() {
        let p = and_then(
            satisfy(|c| c.is_ascii_digit(), "digit"),
            |d| map(satisfy(|c| c.is_ascii_digit(), "digit"), move |d2| format!("{}{}", d, d2)),
        );
        assert_eq!(p("12x"), Ok(("12".to_string(), "x")));
    }

    #[test]
    fn test_length_prefixed() {
        let p = length_prefixed();
        assert_eq!(p("3:abcrest"), Ok(("abc", "rest")));
    }

    #[test]
    fn test_length_prefixed_5() {
        let p = length_prefixed();
        assert_eq!(p("5:helloworld"), Ok(("hello", "world")));
    }

    #[test]
    fn test_length_prefixed_too_short() {
        let p = length_prefixed();
        assert!(p("5:hi").is_err());
    }

    #[test]
    fn test_conditional_int() {
        let p = conditional_parser();
        assert_eq!(p("i42rest"), Ok(("42".to_string(), "rest")));
    }

    #[test]
    fn test_conditional_string() {
        let p = conditional_parser();
        assert_eq!(p("sabcREST"), Ok(("abc".to_string(), "REST")));
    }

    #[test]
    fn test_conditional_invalid_tag() {
        let p = conditional_parser();
        assert!(p("x123").is_err());
    }

    #[test]
    fn test_and_then_error_propagation() {
        let p = and_then(
            satisfy(|c| c.is_ascii_digit(), "digit"),
            |_| satisfy(|c| c.is_ascii_uppercase(), "upper"),
        );
        assert!(p("1a").is_err()); // second parser fails
    }
}
