// Example 171: CSV Parser
// Complete CSV parser using combinators (handles quotes, escaping)

type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ============================================================
// Approach 1: Unquoted field
// ============================================================

fn unquoted_field(input: &str) -> ParseResult<String> {
    let end = input
        .find(|c: char| c == ',' || c == '\n' || c == '\r')
        .unwrap_or(input.len());
    Ok((input[..end].trim().to_string(), &input[end..]))
}

// ============================================================
// Approach 2: Quoted field with escaped quotes ("")
// ============================================================

fn quoted_field(input: &str) -> ParseResult<String> {
    if !input.starts_with('"') {
        return Err("Expected opening quote".to_string());
    }
    let mut result = String::new();
    let mut chars = input[1..].chars();
    let mut consumed = 1; // opening quote
    loop {
        match chars.next() {
            None => return Err("Unterminated quoted field".to_string()),
            Some('"') => {
                consumed += 1;
                // Check for escaped quote ""
                match chars.next() {
                    Some('"') => {
                        result.push('"');
                        consumed += 1;
                    }
                    _ => {
                        // End of quoted field (we over-consumed one char, but use byte math)
                        return Ok((result, &input[consumed..]));
                    }
                }
            }
            Some(c) => {
                result.push(c);
                consumed += c.len_utf8();
            }
        }
    }
}

// ============================================================
// Approach 3: Full CSV parser
// ============================================================

fn field(input: &str) -> ParseResult<String> {
    if input.starts_with('"') {
        quoted_field(input)
    } else {
        unquoted_field(input)
    }
}

fn row(input: &str) -> ParseResult<Vec<String>> {
    let (first, mut rest) = field(input)?;
    let mut fields = vec![first];
    while rest.starts_with(',') {
        let (f, r) = field(&rest[1..])?;
        fields.push(f);
        rest = r;
    }
    Ok((fields, rest))
}

fn line_ending(input: &str) -> ParseResult<()> {
    if input.starts_with("\r\n") {
        Ok(((), &input[2..]))
    } else if input.starts_with('\n') {
        Ok(((), &input[1..]))
    } else if input.is_empty() {
        Ok(((), ""))
    } else {
        Err("Expected line ending".to_string())
    }
}

fn csv(input: &str) -> ParseResult<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    let mut remaining = input;
    while !remaining.is_empty() {
        let (r, rest) = row(remaining)?;
        rows.push(r);
        let ((), rest) = line_ending(rest)?;
        remaining = rest;
    }
    Ok((rows, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unquoted_field() {
        assert_eq!(
            unquoted_field("hello,world"),
            Ok(("hello".into(), ",world"))
        );
    }

    #[test]
    fn test_quoted_field() {
        assert_eq!(
            quoted_field("\"hello,world\""),
            Ok(("hello,world".into(), ""))
        );
    }

    #[test]
    fn test_escaped_quotes() {
        assert_eq!(
            quoted_field("\"say \"\"hi\"\"\""),
            Ok(("say \"hi\"".into(), ""))
        );
    }

    #[test]
    fn test_quoted_with_newline() {
        assert_eq!(
            quoted_field("\"line1\nline2\""),
            Ok(("line1\nline2".into(), ""))
        );
    }

    #[test]
    fn test_row() {
        let (r, _) = row("a,b,c").unwrap();
        assert_eq!(r, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_row_quoted() {
        let (r, _) = row("\"x,y\",z").unwrap();
        assert_eq!(r, vec!["x,y", "z"]);
    }

    #[test]
    fn test_csv() {
        let (rows, _) = csv("a,b\n1,2\n3,4").unwrap();
        assert_eq!(rows, vec![vec!["a", "b"], vec!["1", "2"], vec!["3", "4"],]);
    }

    #[test]
    fn test_csv_crlf() {
        let (rows, _) = csv("a,b\r\n1,2").unwrap();
        assert_eq!(rows, vec![vec!["a", "b"], vec!["1", "2"]]);
    }

    #[test]
    fn test_empty_field() {
        let (r, _) = row(",a,").unwrap();
        assert_eq!(r, vec!["", "a", ""]);
    }

    #[test]
    fn test_unterminated_quote() {
        assert!(quoted_field("\"hello").is_err());
    }
}
