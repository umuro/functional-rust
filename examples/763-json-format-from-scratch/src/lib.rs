//! # JSON Format From Scratch
//!
//! Building a simple JSON serializer without serde.

/// JSON value representation
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
    /// Serialize to JSON string
    pub fn to_json(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && n.abs() < 1e15 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            JsonValue::String(s) => format!("\"{}\"", escape_json_string(s)),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_json()).collect();
                format!("[{}]", items.join(", "))
            }
            JsonValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", escape_json_string(k), v.to_json()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }

    /// Pretty print with indentation
    pub fn to_json_pretty(&self, indent: usize) -> String {
        self.to_json_indent(0, indent)
    }

    fn to_json_indent(&self, level: usize, indent: usize) -> String {
        let prefix = " ".repeat(level * indent);
        let inner_prefix = " ".repeat((level + 1) * indent);

        match self {
            JsonValue::Array(arr) if arr.is_empty() => "[]".to_string(),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format!("{}{}", inner_prefix, v.to_json_indent(level + 1, indent)))
                    .collect();
                format!("[\n{}\n{}]", items.join(",\n"), prefix)
            }
            JsonValue::Object(obj) if obj.is_empty() => "{}".to_string(),
            JsonValue::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}\"{}\": {}",
                            inner_prefix,
                            escape_json_string(k),
                            v.to_json_indent(level + 1, indent)
                        )
                    })
                    .collect();
                format!("{{\n{}\n{}}}", pairs.join(",\n"), prefix)
            }
            _ => self.to_json(),
        }
    }
}

/// Escape special characters in JSON strings
fn escape_json_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => result.push_str(&format!("\\u{:04x}", c as u32)),
            c => result.push(c),
        }
    }
    result
}

/// Trait for converting to JSON
pub trait ToJson {
    fn to_json_value(&self) -> JsonValue;
}

impl ToJson for bool {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Bool(*self)
    }
}

impl ToJson for i32 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(*self as f64)
    }
}

impl ToJson for f64 {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Number(*self)
    }
}

impl ToJson for String {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::String(self.clone())
    }
}

impl ToJson for &str {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::String(self.to_string())
    }
}

impl<T: ToJson> ToJson for Vec<T> {
    fn to_json_value(&self) -> JsonValue {
        JsonValue::Array(self.iter().map(|v| v.to_json_value()).collect())
    }
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json_value(&self) -> JsonValue {
        match self {
            Some(v) => v.to_json_value(),
            None => JsonValue::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        assert_eq!(JsonValue::Null.to_json(), "null");
    }

    #[test]
    fn test_bool() {
        assert_eq!(JsonValue::Bool(true).to_json(), "true");
        assert_eq!(JsonValue::Bool(false).to_json(), "false");
    }

    #[test]
    fn test_number() {
        assert_eq!(JsonValue::Number(42.0).to_json(), "42");
        assert_eq!(JsonValue::Number(3.14).to_json(), "3.14");
    }

    #[test]
    fn test_string() {
        assert_eq!(
            JsonValue::String("hello".to_string()).to_json(),
            "\"hello\""
        );
    }

    #[test]
    fn test_string_escape() {
        assert_eq!(
            JsonValue::String("a\"b".to_string()).to_json(),
            "\"a\\\"b\""
        );
        assert_eq!(
            JsonValue::String("a\nb".to_string()).to_json(),
            "\"a\\nb\""
        );
    }

    #[test]
    fn test_array() {
        let arr = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);
        assert_eq!(arr.to_json(), "[1, 2, 3]");
    }

    #[test]
    fn test_object() {
        let obj = JsonValue::Object(vec![
            ("a".to_string(), JsonValue::Number(1.0)),
            ("b".to_string(), JsonValue::Number(2.0)),
        ]);
        assert_eq!(obj.to_json(), r#"{"a": 1, "b": 2}"#);
    }

    #[test]
    fn test_trait() {
        assert_eq!(42i32.to_json_value().to_json(), "42");
        assert_eq!("hello".to_json_value().to_json(), "\"hello\"");
    }

    #[test]
    fn test_option() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;
        assert_eq!(some.to_json_value().to_json(), "42");
        assert_eq!(none.to_json_value().to_json(), "null");
    }
}
