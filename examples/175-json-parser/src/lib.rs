// Example 175: Complete JSON Parser
// Full JSON parser: null, bool, number, string, array, object
// This is the capstone example using all parser combinator primitives

type ParseResult<'a, T> = Result<(T, &'a str), String>;

#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Json::Null => write!(f, "null"),
            Json::Bool(b) => write!(f, "{}", b),
            Json::Number(n) => {
                if *n == (*n as i64) as f64 && n.abs() < 1e15 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Json::Str(s) => write!(f, "\"{}\"", s),
            Json::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Json::Object(entries) => {
                write!(f, "{{")?;
                for (i, (k, v)) in entries.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

// ============================================================
// JSON String parser
// ============================================================

fn parse_json_string(input: &str) -> ParseResult<String> {
    let s = input.trim_start();
    if !s.starts_with('"') {
        return Err("Expected '\"'".to_string());
    }
    let mut result = String::new();
    let mut chars = s[1..].chars();
    let mut consumed = 1;
    loop {
        match chars.next() {
            None => return Err("Unterminated string".to_string()),
            Some('"') => {
                consumed += 1;
                return Ok((result, &s[consumed..]));
            }
            Some('\\') => {
                consumed += 1;
                match chars.next() {
                    Some('n') => {
                        result.push('\n');
                        consumed += 1;
                    }
                    Some('t') => {
                        result.push('\t');
                        consumed += 1;
                    }
                    Some('r') => {
                        result.push('\r');
                        consumed += 1;
                    }
                    Some('"') => {
                        result.push('"');
                        consumed += 1;
                    }
                    Some('\\') => {
                        result.push('\\');
                        consumed += 1;
                    }
                    Some('/') => {
                        result.push('/');
                        consumed += 1;
                    }
                    Some('u') => {
                        // Unicode escape \uXXXX
                        let mut hex = String::new();
                        for _ in 0..4 {
                            match chars.next() {
                                Some(h) if h.is_ascii_hexdigit() => {
                                    hex.push(h);
                                    consumed += 1;
                                }
                                _ => return Err("Invalid unicode escape".to_string()),
                            }
                        }
                        consumed += 1; // the 'u'
                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                            if let Some(c) = char::from_u32(code) {
                                result.push(c);
                            }
                        }
                    }
                    Some(c) => {
                        result.push('\\');
                        result.push(c);
                        consumed += c.len_utf8();
                    }
                    None => return Err("Unexpected end of escape".to_string()),
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
// JSON Number parser
// ============================================================

fn parse_json_number(input: &str) -> ParseResult<Json> {
    let s = input.trim_start();
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut pos = 0;
    // optional minus
    if pos < len && bytes[pos] == b'-' {
        pos += 1;
    }
    // integer part
    if pos < len && bytes[pos] == b'0' {
        pos += 1;
    } else {
        if pos >= len || !bytes[pos].is_ascii_digit() {
            return Err("Expected digit".to_string());
        }
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
    }
    // fractional part
    if pos < len && bytes[pos] == b'.' {
        pos += 1;
        if pos >= len || !bytes[pos].is_ascii_digit() {
            return Err("Expected digit after '.'".to_string());
        }
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
    }
    // exponent
    if pos < len && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        pos += 1;
        if pos < len && (bytes[pos] == b'+' || bytes[pos] == b'-') {
            pos += 1;
        }
        if pos >= len || !bytes[pos].is_ascii_digit() {
            return Err("Expected digit in exponent".to_string());
        }
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
    }
    let n: f64 = s[..pos]
        .parse()
        .map_err(|e: std::num::ParseFloatError| e.to_string())?;
    Ok((Json::Number(n), &s[pos..]))
}

// ============================================================
// Main JSON parser (recursive)
// ============================================================

fn parse_json(input: &str) -> ParseResult<Json> {
    let s = input.trim_start();
    if s.is_empty() {
        return Err("Unexpected EOF".to_string());
    }
    match s.as_bytes()[0] {
        b'n' => parse_keyword(s, "null", Json::Null),
        b't' => parse_keyword(s, "true", Json::Bool(true)),
        b'f' => parse_keyword(s, "false", Json::Bool(false)),
        b'"' => {
            let (str_val, rest) = parse_json_string(s)?;
            Ok((Json::Str(str_val), rest))
        }
        b'[' => parse_array(s),
        b'{' => parse_object(s),
        b'-' | b'0'..=b'9' => parse_json_number(s),
        c => Err(format!("Unexpected character: '{}'", c as char)),
    }
}

fn parse_keyword<'a>(input: &'a str, kw: &str, value: Json) -> ParseResult<'a, Json> {
    if input.starts_with(kw) {
        Ok((value, &input[kw.len()..]))
    } else {
        Err(format!("Expected \"{}\"", kw))
    }
}

fn parse_array(input: &str) -> ParseResult<Json> {
    let mut remaining = input[1..].trim_start(); // skip '['
    if remaining.starts_with(']') {
        return Ok((Json::Array(vec![]), &remaining[1..]));
    }
    let mut items = Vec::new();
    loop {
        let (value, rest) = parse_json(remaining)?;
        items.push(value);
        let rest = rest.trim_start();
        if rest.starts_with(',') {
            remaining = rest[1..].trim_start();
        } else if rest.starts_with(']') {
            return Ok((Json::Array(items), &rest[1..]));
        } else {
            return Err("Expected ',' or ']'".to_string());
        }
    }
}

fn parse_object(input: &str) -> ParseResult<Json> {
    let mut remaining = input[1..].trim_start(); // skip '{'
    if remaining.starts_with('}') {
        return Ok((Json::Object(vec![]), &remaining[1..]));
    }
    let mut entries = Vec::new();
    loop {
        let (key, rest) = parse_json_string(remaining)?;
        let rest = rest.trim_start();
        if !rest.starts_with(':') {
            return Err("Expected ':'".to_string());
        }
        let (value, rest) = parse_json(&rest[1..])?;
        entries.push((key, value));
        let rest = rest.trim_start();
        if rest.starts_with(',') {
            remaining = rest[1..].trim_start();
        } else if rest.starts_with('}') {
            return Ok((Json::Object(entries), &rest[1..]));
        } else {
            return Err("Expected ',' or '}'".to_string());
        }
    }
}

// ============================================================
// Convenience: parse full JSON string
// ============================================================

fn parse(input: &str) -> Result<Json, String> {
    let (value, rest) = parse_json(input)?;
    if rest.trim().is_empty() {
        Ok(value)
    } else {
        Err(format!("Unexpected trailing content: {:?}", rest.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        assert_eq!(parse("null"), Ok(Json::Null));
    }

    #[test]
    fn test_true() {
        assert_eq!(parse("true"), Ok(Json::Bool(true)));
    }

    #[test]
    fn test_false() {
        assert_eq!(parse("false"), Ok(Json::Bool(false)));
    }

    #[test]
    fn test_integer() {
        assert_eq!(parse("42"), Ok(Json::Number(42.0)));
    }

    #[test]
    fn test_negative_float() {
        assert_eq!(parse("-3.14"), Ok(Json::Number(-3.14)));
    }

    #[test]
    fn test_scientific() {
        assert_eq!(parse("1e10"), Ok(Json::Number(1e10)));
    }

    #[test]
    fn test_string() {
        assert_eq!(parse("\"hello\""), Ok(Json::Str("hello".into())));
    }

    #[test]
    fn test_string_escapes() {
        assert_eq!(
            parse("\"hello\\nworld\""),
            Ok(Json::Str("hello\nworld".into()))
        );
    }

    #[test]
    fn test_string_tab() {
        assert_eq!(parse("\"a\\tb\""), Ok(Json::Str("a\tb".into())));
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(parse("[]"), Ok(Json::Array(vec![])));
    }

    #[test]
    fn test_array() {
        assert_eq!(
            parse("[1, 2, 3]"),
            Ok(Json::Array(vec![
                Json::Number(1.0),
                Json::Number(2.0),
                Json::Number(3.0),
            ]))
        );
    }

    #[test]
    fn test_nested_array() {
        assert_eq!(
            parse("[[1], [2]]"),
            Ok(Json::Array(vec![
                Json::Array(vec![Json::Number(1.0)]),
                Json::Array(vec![Json::Number(2.0)]),
            ]))
        );
    }

    #[test]
    fn test_empty_object() {
        assert_eq!(parse("{}"), Ok(Json::Object(vec![])));
    }

    #[test]
    fn test_object() {
        assert_eq!(
            parse("{\"a\": 1, \"b\": true}"),
            Ok(Json::Object(vec![
                ("a".into(), Json::Number(1.0)),
                ("b".into(), Json::Bool(true)),
            ]))
        );
    }

    #[test]
    fn test_nested() {
        assert_eq!(
            parse("{\"data\": [1, {\"x\": null}]}"),
            Ok(Json::Object(vec![(
                "data".into(),
                Json::Array(vec![
                    Json::Number(1.0),
                    Json::Object(vec![("x".into(), Json::Null)]),
                ])
            ),]))
        );
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(
            parse("  {  \"a\"  :  1  }  "),
            Ok(Json::Object(vec![("a".into(), Json::Number(1.0)),]))
        );
    }

    #[test]
    fn test_complex_json() {
        let input = r#"{"name": "test", "values": [1, 2.5, true, null, "hello"]}"#;
        let json = parse(input).unwrap();
        match json {
            Json::Object(entries) => assert_eq!(entries.len(), 2),
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_unterminated_string() {
        assert!(parse("\"hello").is_err());
    }

    #[test]
    fn test_unterminated_array() {
        assert!(parse("[1, 2").is_err());
    }

    #[test]
    fn test_invalid_json() {
        assert!(parse("xyz").is_err());
    }

    #[test]
    fn test_display_roundtrip() {
        let input = r#"{"a": [1, true, null]}"#;
        let json = parse(input).unwrap();
        let output = format!("{}", json);
        let reparsed = parse(&output).unwrap();
        assert_eq!(json, reparsed);
    }
}
