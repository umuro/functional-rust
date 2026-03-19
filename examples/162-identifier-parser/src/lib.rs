// Example 162: Identifier Parser
// Parse identifiers: letter followed by alphanumeric/underscore

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

fn many0<'a, T: 'a>(p: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |mut input: &'a str| {
        let mut v = Vec::new();
        while let Ok((val, r)) = p(input) {
            v.push(val);
            input = r;
        }
        Ok((v, input))
    })
}

// ============================================================
// Approach 1: Direct identifier parser
// ============================================================

fn identifier<'a>() -> Parser<'a, String> {
    Box::new(|input: &'a str| {
        let start = satisfy(|c| c.is_ascii_alphabetic() || c == '_', "letter or _");
        let (first, rest) = start(input)?;
        let cont = many0(satisfy(
            |c| c.is_ascii_alphanumeric() || c == '_',
            "ident char",
        ));
        let (chars, rem) = cont(rest)?;
        let mut s = String::with_capacity(1 + chars.len());
        s.push(first);
        for c in chars {
            s.push(c);
        }
        Ok((s, rem))
    })
}

// ============================================================
// Approach 2: Zero-copy identifier (returns &str slice)
// ============================================================

fn identifier_slice<'a>() -> Parser<'a, &'a str> {
    Box::new(|input: &'a str| {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let mut end = c.len_utf8();
                for ch in chars {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        end += ch.len_utf8();
                    } else {
                        break;
                    }
                }
                Ok((&input[..end], &input[end..]))
            }
            _ => Err("Expected identifier".to_string()),
        }
    })
}

// ============================================================
// Approach 3: With reserved word checking
// ============================================================

fn identifier_not_reserved<'a>(reserved: &[&str]) -> Parser<'a, String> {
    let reserved: Vec<String> = reserved.iter().map(|s| s.to_string()).collect();
    Box::new(move |input: &'a str| {
        let (name, rest) = identifier()(input)?;
        if reserved.iter().any(|r| r == &name) {
            Err(format!("'{}' is a reserved word", name))
        } else {
            Ok((name, rest))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_simple() {
        assert_eq!(identifier()("hello world"), Ok(("hello".into(), " world")));
    }

    #[test]
    fn test_identifier_underscore_start() {
        assert_eq!(identifier()("_foo bar"), Ok(("_foo".into(), " bar")));
    }

    #[test]
    fn test_identifier_mixed() {
        assert_eq!(identifier()("x1y2z3!"), Ok(("x1y2z3".into(), "!")));
    }

    #[test]
    fn test_identifier_single_underscore() {
        assert_eq!(identifier()("_"), Ok(("_".into(), "")));
    }

    #[test]
    fn test_identifier_starts_with_digit() {
        assert!(identifier()("123").is_err());
    }

    #[test]
    fn test_identifier_slice() {
        assert_eq!(identifier_slice()("myVar = 5"), Ok(("myVar", " = 5")));
    }

    #[test]
    fn test_not_reserved_ok() {
        let p = identifier_not_reserved(&["let", "if"]);
        assert_eq!(p("myVar"), Ok(("myVar".into(), "")));
    }

    #[test]
    fn test_not_reserved_blocked() {
        let p = identifier_not_reserved(&["let", "if"]);
        assert!(p("let").is_err());
    }

    #[test]
    fn test_identifier_empty() {
        assert!(identifier()("").is_err());
    }
}
