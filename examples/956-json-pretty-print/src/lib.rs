// 956: JSON Pretty Print
// Recursive pretty-printer: OCaml uses Buffer, Rust builds String

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// Approach 1: Pretty-print with indentation (recursive, builds String)
fn escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

fn pretty_print(j: &JsonValue, indent: usize) -> String {
    let pad = " ".repeat(indent * 2);
    let pad2 = " ".repeat((indent + 1) * 2);
    match j {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(true) => "true".to_string(),
        JsonValue::Bool(false) => "false".to_string(),
        JsonValue::Number(n) => {
            if n.fract() == 0.0 && n.is_finite() {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        JsonValue::Str(s) => format!("\"{}\"", escape_string(s)),
        JsonValue::Array(items) if items.is_empty() => "[]".to_string(),
        JsonValue::Array(items) => {
            let inner: Vec<String> = items
                .iter()
                .map(|item| format!("{}{}", pad2, pretty_print(item, indent + 1)))
                .collect();
            format!("[\n{}\n{}]", inner.join(",\n"), pad)
        }
        JsonValue::Object(pairs) if pairs.is_empty() => "{}".to_string(),
        JsonValue::Object(pairs) => {
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}\"{}\": {}",
                        pad2,
                        escape_string(k),
                        pretty_print(v, indent + 1)
                    )
                })
                .collect();
            format!("{{\n{}\n{}}}", inner.join(",\n"), pad)
        }
    }
}

// Approach 2: Compact (single-line) printer
fn compact(j: &JsonValue) -> String {
    match j {
        JsonValue::Null => "null".to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Number(n) => {
            if n.fract() == 0.0 && n.is_finite() {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        JsonValue::Str(s) => format!("\"{}\"", escape_string(s)),
        JsonValue::Array(items) => {
            let inner: Vec<String> = items.iter().map(compact).collect();
            format!("[{}]", inner.join(","))
        }
        JsonValue::Object(pairs) => {
            let inner: Vec<String> = pairs
                .iter()
                .map(|(k, v)| format!("\"{}\":{}", escape_string(k), compact(v)))
                .collect();
            format!("{{{}}}", inner.join(","))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitives() {
        assert_eq!(pretty_print(&JsonValue::Null, 0), "null");
        assert_eq!(pretty_print(&JsonValue::Bool(true), 0), "true");
        assert_eq!(pretty_print(&JsonValue::Bool(false), 0), "false");
        assert_eq!(pretty_print(&JsonValue::Number(42.0), 0), "42");
        assert_eq!(pretty_print(&JsonValue::Str("hi".into()), 0), "\"hi\"");
    }

    #[test]
    fn test_escape() {
        let s = JsonValue::Str("hello \"world\"\nnewline".to_string());
        assert_eq!(pretty_print(&s, 0), "\"hello \\\"world\\\"\\nnewline\"");
    }

    #[test]
    fn test_empty_array_object() {
        assert_eq!(pretty_print(&JsonValue::Array(vec![]), 0), "[]");
        assert_eq!(pretty_print(&JsonValue::Object(vec![]), 0), "{}");
    }

    #[test]
    fn test_compact_no_newlines() {
        let json = JsonValue::Object(vec![
            ("a".to_string(), JsonValue::Number(1.0)),
            ("b".to_string(), JsonValue::Bool(false)),
        ]);
        let c = compact(&json);
        assert!(!c.contains('\n'));
        assert!(c.contains("\"a\":1"));
        assert!(c.contains("\"b\":false"));
    }

    #[test]
    fn test_nested_pretty() {
        let json = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Array(vec![JsonValue::Number(2.0), JsonValue::Number(3.0)]),
        ]);
        let p = pretty_print(&json, 0);
        assert!(p.contains('\n'));
        assert!(p.starts_with('['));
        assert!(p.ends_with(']'));
    }
}
