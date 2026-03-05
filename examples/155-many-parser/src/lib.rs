// Example 155: Many Parser
// many0 and many1: parse zero or more / one or more

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where F: Fn(char) -> bool + 'a {
    let desc = desc.to_string();
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
            _ => Err(format!("Expected {}", desc)),
        }
    })
}

// ============================================================
// Approach 1: many0 — zero or more, always succeeds
// ============================================================

fn many0<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |mut input: &'a str| {
        let mut results = Vec::new();
        while let Ok((value, rest)) = parser(input) {
            results.push(value);
            input = rest;
        }
        Ok((results, input))
    })
}

// ============================================================
// Approach 2: many1 — one or more, fails if zero
// ============================================================

fn many1<'a, T: 'a>(parser: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut remaining) = parser(input)?;
        let mut results = vec![first];
        while let Ok((value, rest)) = parser(remaining) {
            results.push(value);
            remaining = rest;
        }
        Ok((results, remaining))
    })
}

// ============================================================
// Approach 3: many_till — parse until terminator succeeds
// ============================================================

fn many_till<'a, T: 'a, U: 'a>(
    parser: Parser<'a, T>,
    stop: Parser<'a, U>,
) -> Parser<'a, (Vec<T>, U)> {
    Box::new(move |mut input: &'a str| {
        let mut results = Vec::new();
        loop {
            if let Ok((term, rest)) = stop(input) {
                return Ok(((results, term), rest));
            }
            let (value, rest) = parser(input)?;
            results.push(value);
            input = rest;
        }
    })
}

/// Collect many0 chars into a String
fn many0_str<'a>(parser: Parser<'a, char>) -> Parser<'a, String> {
    Box::new(move |mut input: &'a str| {
        let mut s = String::new();
        while let Ok((c, rest)) = parser(input) {
            s.push(c);
            input = rest;
        }
        Ok((s, input))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_many0_some() {
        let p = many0(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (digits, rest) = p("123abc").unwrap();
        assert_eq!(digits, vec!['1', '2', '3']);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_many0_none() {
        let p = many0(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (digits, rest) = p("abc").unwrap();
        assert!(digits.is_empty());
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_many0_empty_input() {
        let p = many0(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (digits, rest) = p("").unwrap();
        assert!(digits.is_empty());
        assert_eq!(rest, "");
    }

    #[test]
    fn test_many1_some() {
        let p = many1(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (digits, rest) = p("123abc").unwrap();
        assert_eq!(digits, vec!['1', '2', '3']);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_many1_none_fails() {
        let p = many1(satisfy(|c| c.is_ascii_digit(), "digit"));
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_many_till() {
        let letter = satisfy(|c| c.is_ascii_alphabetic(), "letter");
        let dot = satisfy(|c| c == '.', "dot");
        let p = many_till(letter, dot);
        let ((letters, term), rest) = p("abc.rest").unwrap();
        assert_eq!(letters, vec!['a', 'b', 'c']);
        assert_eq!(term, '.');
        assert_eq!(rest, "rest");
    }

    #[test]
    fn test_many0_str() {
        let p = many0_str(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (s, rest) = p("456xy").unwrap();
        assert_eq!(s, "456");
        assert_eq!(rest, "xy");
    }

    #[test]
    fn test_many1_single() {
        let p = many1(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (digits, rest) = p("5abc").unwrap();
        assert_eq!(digits, vec!['5']);
        assert_eq!(rest, "abc");
    }
}
