#![allow(clippy::all)]
//! # Parser Combinator Pattern
//!
//! Building parsers from small composable pieces.

/// Parser result
pub type ParseResult<'a, T> = Option<(T, &'a str)>;

/// Parser type alias
pub type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

// ══════════════════════════════════════════════════════════════════════════════
// Primitive Parsers
// ══════════════════════════════════════════════════════════════════════════════

/// Parse a specific character
pub fn char_p(c: char) -> impl Fn(&str) -> ParseResult<char> {
    move |input| {
        input
            .chars()
            .next()
            .filter(|&ch| ch == c)
            .map(|ch| (ch, &input[ch.len_utf8()..]))
    }
}

/// Parse any character satisfying a predicate
pub fn satisfy<F>(pred: F) -> impl Fn(&str) -> ParseResult<char>
where
    F: Fn(char) -> bool,
{
    move |input| {
        input
            .chars()
            .next()
            .filter(|&c| pred(c))
            .map(|c| (c, &input[c.len_utf8()..]))
    }
}

/// Parse a specific string
pub fn string_p(s: &str) -> impl Fn(&str) -> ParseResult<&str> + '_ {
    move |input| {
        if input.starts_with(s) {
            Some((&input[..s.len()], &input[s.len()..]))
        } else {
            None
        }
    }
}

/// Parse one or more digits
pub fn digits(input: &str) -> ParseResult<&str> {
    let end = input
        .char_indices()
        .take_while(|(_, c)| c.is_ascii_digit())
        .last()
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(0);

    if end > 0 {
        Some((&input[..end], &input[end..]))
    } else {
        None
    }
}

/// Parse whitespace
pub fn whitespace(input: &str) -> ParseResult<&str> {
    let end = input
        .char_indices()
        .take_while(|(_, c)| c.is_whitespace())
        .last()
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(0);

    Some((&input[..end], &input[end..]))
}

// ══════════════════════════════════════════════════════════════════════════════
// Combinators
// ══════════════════════════════════════════════════════════════════════════════

/// Map over parser result
pub fn map<'a, A, B, P, F>(parser: P, f: F) -> impl Fn(&'a str) -> ParseResult<'a, B>
where
    P: Fn(&'a str) -> ParseResult<'a, A>,
    F: Fn(A) -> B,
{
    move |input| parser(input).map(|(a, rest)| (f(a), rest))
}

/// Sequence two parsers
pub fn then<'a, A, B, PA, PB>(p1: PA, p2: PB) -> impl Fn(&'a str) -> ParseResult<'a, (A, B)>
where
    PA: Fn(&'a str) -> ParseResult<'a, A>,
    PB: Fn(&'a str) -> ParseResult<'a, B>,
{
    move |input| {
        let (a, rest) = p1(input)?;
        let (b, rest) = p2(rest)?;
        Some(((a, b), rest))
    }
}

/// Try first parser, if fails try second
pub fn or<'a, A, P1, P2>(p1: P1, p2: P2) -> impl Fn(&'a str) -> ParseResult<'a, A>
where
    P1: Fn(&'a str) -> ParseResult<'a, A>,
    P2: Fn(&'a str) -> ParseResult<'a, A>,
{
    move |input| p1(input).or_else(|| p2(input))
}

/// Parse zero or more
pub fn many<'a, A, P>(parser: P) -> impl Fn(&'a str) -> ParseResult<'a, Vec<A>>
where
    P: Fn(&'a str) -> ParseResult<'a, A>,
{
    move |mut input| {
        let mut results = Vec::new();
        while let Some((item, rest)) = parser(input) {
            results.push(item);
            input = rest;
        }
        Some((results, input))
    }
}

/// Parse one or more
pub fn many1<'a, A, P>(parser: P) -> impl Fn(&'a str) -> ParseResult<'a, Vec<A>>
where
    P: Fn(&'a str) -> ParseResult<'a, A>,
{
    move |input| {
        let (first, mut rest) = parser(input)?;
        let mut results = vec![first];
        while let Some((item, new_rest)) = parser(rest) {
            results.push(item);
            rest = new_rest;
        }
        Some((results, rest))
    }
}

/// Parse with separator
pub fn sep_by<'a, A, S, PA, PS>(parser: PA, sep: PS) -> impl Fn(&'a str) -> ParseResult<'a, Vec<A>>
where
    PA: Fn(&'a str) -> ParseResult<'a, A>,
    PS: Fn(&'a str) -> ParseResult<'a, S>,
{
    move |input| {
        let Some((first, mut rest)) = parser(input) else {
            return Some((Vec::new(), input));
        };
        let mut results = vec![first];
        while let Some((_, after_sep)) = sep(rest) {
            if let Some((item, new_rest)) = parser(after_sep) {
                results.push(item);
                rest = new_rest;
            } else {
                break;
            }
        }
        Some((results, rest))
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Example: Simple expression parser
// ══════════════════════════════════════════════════════════════════════════════

pub fn parse_number(input: &str) -> ParseResult<i64> {
    digits(input).and_then(|(s, rest)| s.parse().ok().map(|n| (n, rest)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_p() {
        assert_eq!(char_p('a')("abc"), Some(('a', "bc")));
        assert_eq!(char_p('a')("xyz"), None);
    }

    #[test]
    fn test_string_p() {
        assert_eq!(string_p("hello")("hello world"), Some(("hello", " world")));
        assert_eq!(string_p("hello")("hi"), None);
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits("123abc"), Some(("123", "abc")));
        assert_eq!(digits("abc"), None);
    }

    #[test]
    fn test_map() {
        let num = map(digits, |s| s.parse::<i32>().unwrap());
        assert_eq!(num("42x"), Some((42, "x")));
    }

    #[test]
    fn test_then() {
        let parser = then(char_p('a'), char_p('b'));
        assert_eq!(parser("abc"), Some((('a', 'b'), "c")));
    }

    #[test]
    fn test_or() {
        let parser = or(char_p('a'), char_p('b'));
        assert_eq!(parser("abc"), Some(('a', "bc")));
        assert_eq!(parser("bcd"), Some(('b', "cd")));
    }

    #[test]
    fn test_many() {
        let parser = many(char_p('a'));
        assert_eq!(parser("aaab"), Some((vec!['a', 'a', 'a'], "b")));
        assert_eq!(parser("bbb"), Some((vec![], "bbb")));
    }

    #[test]
    fn test_sep_by() {
        let parser = sep_by(parse_number, char_p(','));
        assert_eq!(parser("1,2,3"), Some((vec![1, 2, 3], "")));
    }
}
