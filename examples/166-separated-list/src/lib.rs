#![allow(clippy::all)]
// Example 166: Separated List
// separated_list0, separated_list1: comma-separated values

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

fn many1<'a, T: 'a>(p: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut rem) = p(input)?;
        let mut v = vec![first];
        while let Ok((val, r)) = p(rem) {
            v.push(val);
            rem = r;
        }
        Ok((v, rem))
    })
}

// ============================================================
// Approach 1: separated_list0 — zero or more items
// ============================================================

fn separated_list0<'a, T: 'a, S: 'a>(
    sep: Parser<'a, S>,
    item: Parser<'a, T>,
) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut remaining) = match item(input) {
            Err(_) => return Ok((vec![], input)),
            Ok(r) => r,
        };
        let mut results = vec![first];
        loop {
            let after_sep = match sep(remaining) {
                Err(_) => break,
                Ok((_, r)) => r,
            };
            match item(after_sep) {
                Ok((val, rest)) => {
                    results.push(val);
                    remaining = rest;
                }
                Err(_) => break, // backtrack: don't consume trailing sep
            }
        }
        Ok((results, remaining))
    })
}

// ============================================================
// Approach 2: separated_list1 — one or more
// ============================================================

fn separated_list1<'a, T: 'a, S: 'a>(
    sep: Parser<'a, S>,
    item: Parser<'a, T>,
) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (results, rest) = separated_list0_inner(&sep, &item, input)?;
        if results.is_empty() {
            Err("Expected at least one item".to_string())
        } else {
            Ok((results, rest))
        }
    })
}

fn separated_list0_inner<'a, T, S>(
    sep: &(dyn Fn(&'a str) -> ParseResult<'a, S>),
    item: &(dyn Fn(&'a str) -> ParseResult<'a, T>),
    input: &'a str,
) -> ParseResult<'a, Vec<T>> {
    let (first, mut remaining) = match item(input) {
        Err(_) => return Ok((vec![], input)),
        Ok(r) => r,
    };
    let mut results = vec![first];
    loop {
        let after_sep = match sep(remaining) {
            Err(_) => break,
            Ok((_, r)) => r,
        };
        match item(after_sep) {
            Ok((val, rest)) => {
                results.push(val);
                remaining = rest;
            }
            Err(_) => break,
        }
    }
    Ok((results, remaining))
}

// ============================================================
// Approach 3: With trailing separator allowed
// ============================================================

fn separated_list_trailing<'a, T: 'a, S: 'a>(
    sep: Parser<'a, S>,
    item: Parser<'a, T>,
) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut remaining) = match item(input) {
            Err(_) => return Ok((vec![], input)),
            Ok(r) => r,
        };
        let mut results = vec![first];
        loop {
            let after_sep = match sep(remaining) {
                Err(_) => break,
                Ok((_, r)) => r,
            };
            match item(after_sep) {
                Ok((val, rest)) => {
                    results.push(val);
                    remaining = rest;
                }
                Err(_) => {
                    remaining = after_sep;
                    break;
                } // consume trailing sep
            }
        }
        Ok((results, remaining))
    })
}

/// Comma with optional whitespace
fn comma<'a>() -> Parser<'a, char> {
    Box::new(|input: &'a str| {
        let trimmed = input.trim_start();
        match trimmed.chars().next() {
            Some(',') => Ok((',', trimmed[1..].trim_start())),
            _ => Err("Expected ','".to_string()),
        }
    })
}

/// Digit string
fn digit_str<'a>() -> Parser<'a, String> {
    Box::new(|input: &'a str| {
        let p = many1(satisfy(|c| c.is_ascii_digit(), "digit"));
        let (chars, rest) = p(input)?;
        Ok((chars.into_iter().collect(), rest))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sep_list0_multiple() {
        let p = separated_list0(comma(), digit_str());
        let (v, r) = p("1, 2, 3").unwrap();
        assert_eq!(v, vec!["1", "2", "3"]);
        assert_eq!(r, "");
    }

    #[test]
    fn test_sep_list0_empty() {
        let p = separated_list0(comma(), digit_str());
        let (v, _) = p("").unwrap();
        assert!(v.is_empty());
    }

    #[test]
    fn test_sep_list0_single() {
        let p = separated_list0(comma(), digit_str());
        let (v, _) = p("42").unwrap();
        assert_eq!(v, vec!["42"]);
    }

    #[test]
    fn test_sep_list1_success() {
        let p = separated_list1(comma(), digit_str());
        let (v, _) = p("1, 2").unwrap();
        assert_eq!(v, vec!["1", "2"]);
    }

    #[test]
    fn test_sep_list1_empty_fails() {
        let p = separated_list1(comma(), digit_str());
        assert!(p("").is_err());
    }

    #[test]
    fn test_trailing_sep() {
        let p = separated_list_trailing(comma(), digit_str());
        let (v, rest) = p("1, 2, ").unwrap();
        assert_eq!(v, vec!["1", "2"]);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_no_trailing() {
        let p = separated_list0(comma(), digit_str());
        // Should not consume trailing comma
        let (v, rest) = p("1, 2, abc").unwrap();
        assert_eq!(v, vec!["1", "2"]);
        assert_eq!(rest, ", abc"); // comma before abc not consumed (backtrack)
    }
}
