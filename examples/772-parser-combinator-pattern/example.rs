// 772. Parser Combinator Pattern (nom-Style)
// Parsers as functions: &str → Result<(&str, T), E>

pub type Input<'a> = &'a str;
pub type PResult<'a, T> = Result<(Input<'a>, T), String>;

// ── Primitives ─────────────────────────────────────────────────────────────────

/// Match a literal tag
pub fn tag<'a>(prefix: &'static str) -> impl Fn(Input<'a>) -> PResult<'a, &'a str> {
    move |s| {
        if s.starts_with(prefix) {
            Ok((&s[prefix.len()..], &s[..prefix.len()]))
        } else {
            Err(format!("expected {prefix:?}"))
        }
    }
}

/// Match one char
pub fn char_p(c: char) -> impl Fn(Input<'_>) -> PResult<'_, char> {
    move |s| {
        if s.starts_with(c) {
            Ok((&s[c.len_utf8()..], c))
        } else {
            Err(format!("expected '{c}'"))
        }
    }
}

/// Take chars while predicate holds
pub fn take_while<F>(pred: F) -> impl Fn(Input<'_>) -> PResult<'_, &str>
where F: Fn(char) -> bool {
    move |s| {
        let end = s.find(|c| !pred(c)).unwrap_or(s.len());
        Ok((&s[end..], &s[..end]))
    }
}

pub fn alpha(s: Input<'_>) -> PResult<'_, &str> {
    take_while(|c: char| c.is_alphabetic())(s)
}

pub fn digits(s: Input<'_>) -> PResult<'_, &str> {
    take_while(|c: char| c.is_ascii_digit())(s)
}

// ── Combinators ────────────────────────────────────────────────────────────────

/// Apply f to output of p
pub fn map<'a, T, U, P, F>(p: P, f: F) -> impl Fn(Input<'a>) -> PResult<'a, U>
where
    P: Fn(Input<'a>) -> PResult<'a, T>,
    F: Fn(T) -> U,
{
    move |s| p(s).map(|(rest, v)| (rest, f(v)))
}

/// Run p1 then p2, return pair
pub fn pair<'a, A, B, P1, P2>(p1: P1, p2: P2) -> impl Fn(Input<'a>) -> PResult<'a, (A, B)>
where
    P1: Fn(Input<'a>) -> PResult<'a, A>,
    P2: Fn(Input<'a>) -> PResult<'a, B>,
{
    move |s| {
        let (rest, a) = p1(s)?;
        let (rest, b) = p2(rest)?;
        Ok((rest, (a, b)))
    }
}

/// Run p1 then p2, return p1's value (p2 is discarded)
pub fn terminated<'a, A, B, P1, P2>(p1: P1, p2: P2) -> impl Fn(Input<'a>) -> PResult<'a, A>
where
    P1: Fn(Input<'a>) -> PResult<'a, A>,
    P2: Fn(Input<'a>) -> PResult<'a, B>,
{
    move |s| {
        let (rest, a) = p1(s)?;
        let (rest, _) = p2(rest)?;
        Ok((rest, a))
    }
}

/// Match delimited by opener/closer
pub fn delimited<'a, O, T, C, P1, P, P2>(open: P1, p: P, close: P2)
    -> impl Fn(Input<'a>) -> PResult<'a, T>
where
    P1: Fn(Input<'a>) -> PResult<'a, O>,
    P:  Fn(Input<'a>) -> PResult<'a, T>,
    P2: Fn(Input<'a>) -> PResult<'a, C>,
{
    move |s| {
        let (rest, _) = open(s)?;
        let (rest, v) = p(rest)?;
        let (rest, _) = close(rest)?;
        Ok((rest, v))
    }
}

/// Zero or more repetitions
pub fn many0<'a, T, P>(p: P) -> impl Fn(Input<'a>) -> PResult<'a, Vec<T>>
where P: Fn(Input<'a>) -> PResult<'a, T> {
    move |mut s| {
        let mut acc = Vec::new();
        loop {
            match p(s) {
                Ok((rest, v)) => { acc.push(v); s = rest; }
                Err(_) => break,
            }
        }
        Ok((s, acc))
    }
}

/// p separated by sep
pub fn sep_by<'a, T, S, P, Sep>(p: P, sep: Sep) -> impl Fn(Input<'a>) -> PResult<'a, Vec<T>>
where
    P: Fn(Input<'a>) -> PResult<'a, T>,
    Sep: Fn(Input<'a>) -> PResult<'a, S>,
{
    move |s| {
        let (mut s, first) = match p(s) {
            Ok(r) => r,
            Err(_) => return Ok((s, vec![])),
        };
        let mut acc = vec![first];
        loop {
            let s2 = match sep(s) {
                Ok((rest, _)) => rest,
                Err(_) => break,
            };
            match p(s2) {
                Ok((rest, v)) => { acc.push(v); s = rest; }
                Err(_) => break,
            }
        }
        Ok((s, acc))
    }
}

// ── Application: key=value list parser ────────────────────────────────────────

fn key_value(s: Input<'_>) -> PResult<'_, (&str, &str)> {
    pair(
        terminated(alpha, char_p('=')),
        take_while(|c| c != ',' && c != '\n'),
    )(s)
}

fn kv_list(s: Input<'_>) -> PResult<'_, Vec<(&str, &str)>> {
    sep_by(key_value, char_p(','))(s)
}

fn main() {
    let input = "name=Alice,age=30,city=Berlin";
    let (rest, pairs) = kv_list(input).expect("parse failed");
    println!("Parsed {} pairs, rest={rest:?}:", pairs.len());
    for (k, v) in &pairs { println!("  {k} = {v}"); }

    // Combinator composition demo
    let bracketed = delimited(char_p('['), digits, char_p(']'));
    let (rest, n) = bracketed("[42]remainder").unwrap();
    println!("\n[42] parsed: {n:?}, rest={rest:?}");

    let many_tags = many0(terminated(tag("ab"), char_p(',')));
    let (rest, vs) = many_tags("ab,ab,ab,xy").unwrap();
    println!("many 'ab,': {vs:?}, rest={rest:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_match() {
        assert_eq!(tag("hello")("hello world"), Ok((" world", "hello")));
    }
    #[test]
    fn tag_fail() {
        assert!(tag("hello")("world").is_err());
    }
    #[test]
    fn kv_list_parses() {
        let (rest, pairs) = kv_list("k=v,x=y").unwrap();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0], ("k", "v"));
        assert_eq!(rest, "");
    }
    #[test]
    fn delimited_brackets() {
        let p = delimited(char_p('['), digits, char_p(']'));
        let (rest, v) = p("[123]done").unwrap();
        assert_eq!(v, "123");
        assert_eq!(rest, "done");
    }
    #[test]
    fn many0_collects() {
        let p = many0(terminated(tag("x"), char_p(',')));
        let (rest, vs) = p("x,x,x,y").unwrap();
        assert_eq!(vs.len(), 3);
        assert_eq!(rest, "y");
    }
}
