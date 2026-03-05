// Example 165: Keyword Parser
// Keywords with word boundary checking

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

// ============================================================
// Approach 1: keyword — match string + word boundary
// ============================================================

fn keyword<'a>(kw: &str) -> Parser<'a, &'a str> {
    let kw_owned = kw.to_string();
    Box::new(move |input: &'a str| {
        if !input.starts_with(&kw_owned) {
            return Err(format!("Expected \"{}\"", kw_owned));
        }
        let rest = &input[kw_owned.len()..];
        match rest.chars().next() {
            Some(c) if is_ident_char(c) => {
                Err(format!("\"{}\" not a complete keyword (followed by '{}')", kw_owned, c))
            }
            _ => Ok((&input[..kw_owned.len()], rest)),
        }
    })
}

// ============================================================
// Approach 2: keyword mapped to a token type
// ============================================================

#[derive(Debug, Clone, PartialEq)]
enum Token {
    If, Then, Else, Let, In, Fn,
}

fn keyword_token<'a>(kw: &str, tok: Token) -> Parser<'a, Token> {
    let kw_owned = kw.to_string();
    Box::new(move |input: &'a str| {
        if !input.starts_with(&kw_owned) {
            return Err(format!("Expected \"{}\"", kw_owned));
        }
        let rest = &input[kw_owned.len()..];
        match rest.chars().next() {
            Some(c) if is_ident_char(c) => {
                Err(format!("\"{}\" not a complete keyword", kw_owned))
            }
            _ => Ok((tok.clone(), rest)),
        }
    })
}

// ============================================================
// Approach 3: any_keyword — try multiple, longest first
// ============================================================

fn any_keyword<'a>(keywords: Vec<(&str, Token)>) -> Parser<'a, Token> {
    let mut kws: Vec<(String, Token)> = keywords
        .into_iter()
        .map(|(s, t)| (s.to_string(), t))
        .collect();
    // Sort longest first to avoid prefix ambiguity
    kws.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    Box::new(move |input: &'a str| {
        for (kw, tok) in &kws {
            if input.starts_with(kw.as_str()) {
                let rest = &input[kw.len()..];
                match rest.chars().next() {
                    Some(c) if is_ident_char(c) => continue,
                    _ => return Ok((tok.clone(), rest)),
                }
            }
        }
        Err("Expected keyword".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_match() {
        assert_eq!(keyword("if")("if x"), Ok(("if", " x")));
    }

    #[test]
    fn test_keyword_boundary() {
        assert!(keyword("if")("iffy").is_err());
    }

    #[test]
    fn test_keyword_paren() {
        assert_eq!(keyword("if")("if("), Ok(("if", "(")));
    }

    #[test]
    fn test_keyword_eof() {
        assert_eq!(keyword("if")("if"), Ok(("if", "")));
    }

    #[test]
    fn test_keyword_token() {
        assert_eq!(keyword_token("let", Token::Let)("let x"), Ok((Token::Let, " x")));
    }

    #[test]
    fn test_any_keyword() {
        let p = any_keyword(vec![
            ("if", Token::If), ("in", Token::In), ("let", Token::Let),
        ]);
        assert_eq!(p("let x"), Ok((Token::Let, " x")));
        assert_eq!(p("in "), Ok((Token::In, " ")));
    }

    #[test]
    fn test_any_keyword_none() {
        let p = any_keyword(vec![("if", Token::If)]);
        assert!(p("hello").is_err());
    }

    #[test]
    fn test_keyword_no_match() {
        assert!(keyword("else")("elseif").is_err());
    }
}
