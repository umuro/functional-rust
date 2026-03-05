//! # Advanced Enum Patterns
//!
//! Complex enum usage with recursive variants, associated data, and nested matching.

/// JSON-like value type demonstrating various enum patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    /// Check if value is null.
    pub fn is_null(&self) -> bool {
        matches!(self, Json::Null)
    }

    /// Check if value is truthy (non-null, non-false, non-empty).
    pub fn is_truthy(&self) -> bool {
        match self {
            Json::Null => false,
            Json::Bool(b) => *b,
            Json::Num(n) => *n != 0.0,
            Json::Str(s) => !s.is_empty(),
            Json::Array(a) => !a.is_empty(),
            Json::Object(o) => !o.is_empty(),
        }
    }

    /// Calculate nesting depth.
    pub fn depth(&self) -> usize {
        match self {
            Json::Array(xs) => 1 + xs.iter().map(|x| x.depth()).max().unwrap_or(0),
            Json::Object(kv) => 1 + kv.iter().map(|(_, v)| v.depth()).max().unwrap_or(0),
            _ => 0,
        }
    }

    /// Get a value by key from an object.
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Object(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Get array element by index.
    pub fn index(&self, idx: usize) -> Option<&Json> {
        match self {
            Json::Array(xs) => xs.get(idx),
            _ => None,
        }
    }

    /// Count total nodes in the JSON tree.
    pub fn count_nodes(&self) -> usize {
        match self {
            Json::Array(xs) => 1 + xs.iter().map(|x| x.count_nodes()).sum::<usize>(),
            Json::Object(kv) => 1 + kv.iter().map(|(_, v)| v.count_nodes()).sum::<usize>(),
            _ => 1,
        }
    }

    /// Get type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            Json::Null => "null",
            Json::Bool(_) => "boolean",
            Json::Num(_) => "number",
            Json::Str(_) => "string",
            Json::Array(_) => "array",
            Json::Object(_) => "object",
        }
    }

    /// Try to extract as number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to extract as string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Try to extract as bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Json::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Json::Null => write!(f, "null"),
            Json::Bool(b) => write!(f, "{}", b),
            Json::Num(n) => write!(f, "{}", n),
            Json::Str(s) => write!(f, "{:?}", s),
            Json::Array(xs) => {
                write!(f, "[")?;
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", x)?;
                }
                write!(f, "]")
            }
            Json::Object(kv) => {
                write!(f, "{{")?;
                for (i, (k, v)) in kv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

/// Builder helpers.
impl Json {
    pub fn array(items: Vec<Json>) -> Self {
        Json::Array(items)
    }

    pub fn object(pairs: Vec<(&str, Json)>) -> Self {
        Json::Object(pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
    }

    pub fn string(s: &str) -> Self {
        Json::Str(s.to_string())
    }

    pub fn number(n: f64) -> Self {
        Json::Num(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_json() -> Json {
        Json::object(vec![
            ("name", Json::string("Alice")),
            ("age", Json::number(30.0)),
            ("scores", Json::array(vec![Json::number(95.0), Json::number(87.0)])),
            ("active", Json::Bool(true)),
            ("notes", Json::Null),
        ])
    }

    #[test]
    fn test_is_null() {
        assert!(Json::Null.is_null());
        assert!(!Json::Bool(true).is_null());
    }

    #[test]
    fn test_is_truthy() {
        assert!(!Json::Null.is_truthy());
        assert!(Json::Bool(true).is_truthy());
        assert!(!Json::Bool(false).is_truthy());
        assert!(Json::number(1.0).is_truthy());
        assert!(!Json::number(0.0).is_truthy());
        assert!(Json::string("hi").is_truthy());
        assert!(!Json::string("").is_truthy());
    }

    #[test]
    fn test_depth_flat() {
        assert_eq!(Json::Null.depth(), 0);
        assert_eq!(Json::number(1.0).depth(), 0);
    }

    #[test]
    fn test_depth_nested() {
        let j = Json::array(vec![Json::array(vec![Json::Null])]);
        assert_eq!(j.depth(), 2);
    }

    #[test]
    fn test_depth_object() {
        let j = sample_json();
        assert_eq!(j.depth(), 2); // object -> array -> number
    }

    #[test]
    fn test_get() {
        let j = sample_json();
        assert!(j.get("name").is_some());
        assert_eq!(j.get("name").and_then(|v| v.as_str()), Some("Alice"));
        assert!(j.get("nonexistent").is_none());
    }

    #[test]
    fn test_index() {
        let j = Json::array(vec![Json::number(1.0), Json::number(2.0)]);
        assert_eq!(j.index(0).and_then(|v| v.as_number()), Some(1.0));
        assert!(j.index(10).is_none());
    }

    #[test]
    fn test_count_nodes() {
        let j = sample_json();
        assert_eq!(j.count_nodes(), 8); // object + 5 values + array + 2 scores
    }

    #[test]
    fn test_type_name() {
        assert_eq!(Json::Null.type_name(), "null");
        assert_eq!(Json::Bool(true).type_name(), "boolean");
        assert_eq!(Json::number(1.0).type_name(), "number");
        assert_eq!(Json::string("hi").type_name(), "string");
        assert_eq!(Json::array(vec![]).type_name(), "array");
        assert_eq!(Json::object(vec![]).type_name(), "object");
    }

    #[test]
    fn test_as_extractors() {
        assert_eq!(Json::number(3.14).as_number(), Some(3.14));
        assert_eq!(Json::string("hi").as_str(), Some("hi"));
        assert_eq!(Json::Bool(true).as_bool(), Some(true));
        assert_eq!(Json::Null.as_number(), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Json::Null), "null");
        assert_eq!(format!("{}", Json::Bool(true)), "true");
        assert_eq!(format!("{}", Json::number(42.0)), "42");
    }
}
