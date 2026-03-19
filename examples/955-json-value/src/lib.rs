// 955: JSON Value Type
// OCaml: type json = Null | Bool of bool | Number of float | Str of string | Array of json list | Object of (string * json) list
// Rust: enum JsonValue with derived traits

// Approach 1: Direct enum translation
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// Approach 2: Type checks and simple display
impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }
    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::Str(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }

    pub fn to_string_simple(&self) -> String {
        match self {
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
            JsonValue::Str(s) => format!("\"{}\"", s),
            JsonValue::Array(_) => "[...]".to_string(),
            JsonValue::Object(_) => "{...}".to_string(),
        }
    }
}

// Approach 3: Builder helpers
impl JsonValue {
    pub fn object(pairs: &[(&str, JsonValue)]) -> Self {
        JsonValue::Object(
            pairs
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }

    pub fn array(items: Vec<JsonValue>) -> Self {
        JsonValue::Array(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checks() {
        assert!(JsonValue::Null.is_null());
        assert!(JsonValue::Bool(true).is_bool());
        assert!(JsonValue::Number(1.0).is_number());
        assert!(JsonValue::Str("x".into()).is_string());
        assert!(JsonValue::Array(vec![]).is_array());
        assert!(JsonValue::Object(vec![]).is_object());
    }

    #[test]
    fn test_to_string_simple() {
        assert_eq!(JsonValue::Null.to_string_simple(), "null");
        assert_eq!(JsonValue::Bool(true).to_string_simple(), "true");
        assert_eq!(JsonValue::Bool(false).to_string_simple(), "false");
        assert_eq!(JsonValue::Number(42.0).to_string_simple(), "42");
        assert_eq!(
            JsonValue::Str("hello".into()).to_string_simple(),
            "\"hello\""
        );
        assert_eq!(JsonValue::Array(vec![]).to_string_simple(), "[...]");
        assert_eq!(JsonValue::Object(vec![]).to_string_simple(), "{...}");
    }

    #[test]
    fn test_equality() {
        assert_eq!(JsonValue::Null, JsonValue::Null);
        assert_eq!(JsonValue::Bool(true), JsonValue::Bool(true));
        assert_ne!(JsonValue::Bool(true), JsonValue::Bool(false));
        assert_eq!(JsonValue::Number(1.0), JsonValue::Number(1.0));
        let arr1 = JsonValue::Array(vec![JsonValue::Null, JsonValue::Bool(true)]);
        let arr2 = JsonValue::Array(vec![JsonValue::Null, JsonValue::Bool(true)]);
        assert_eq!(arr1, arr2);
    }

    #[test]
    fn test_nested_object() {
        let obj = JsonValue::object(&[
            ("name", JsonValue::Str("Alice".into())),
            ("age", JsonValue::Number(30.0)),
            ("active", JsonValue::Bool(true)),
        ]);
        assert!(obj.is_object());
        if let JsonValue::Object(pairs) = &obj {
            assert_eq!(pairs.len(), 3);
            assert_eq!(pairs[0].0, "name");
        }
    }
}
