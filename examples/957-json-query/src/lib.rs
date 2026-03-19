// 957: JSON Query by Path
// get(["users", "0", "name"], json) → Option<&JsonValue>
// Rust uses lifetime-annotated references; OCaml returns values directly

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

// Approach 1: Path query returning Option<&JsonValue> (borrows from source)
pub fn get<'a>(path: &[&str], json: &'a JsonValue) -> Option<&'a JsonValue> {
    match path {
        [] => Some(json),
        [key, rest @ ..] => match json {
            JsonValue::Object(pairs) => {
                let found = pairs.iter().find(|(k, _)| k == key);
                found.and_then(|(_, v)| get(rest, v))
            }
            JsonValue::Array(items) => {
                let idx: usize = key.parse().ok()?;
                items.get(idx).and_then(|v| get(rest, v))
            }
            _ => None,
        },
    }
}

// Approach 2: Typed extractors (return borrowed inner values)
pub fn get_string<'a>(path: &[&str], json: &'a JsonValue) -> Option<&'a str> {
    match get(path, json) {
        Some(JsonValue::Str(s)) => Some(s.as_str()),
        _ => None,
    }
}

pub fn get_number(path: &[&str], json: &JsonValue) -> Option<f64> {
    match get(path, json) {
        Some(JsonValue::Number(n)) => Some(*n),
        _ => None,
    }
}

pub fn get_bool(path: &[&str], json: &JsonValue) -> Option<bool> {
    match get(path, json) {
        Some(JsonValue::Bool(b)) => Some(*b),
        _ => None,
    }
}

pub fn get_array<'a>(path: &[&str], json: &'a JsonValue) -> Option<&'a Vec<JsonValue>> {
    match get(path, json) {
        Some(JsonValue::Array(items)) => Some(items),
        _ => None,
    }
}

// Approach 3: Query with default (clones for ownership)
pub fn get_or(default: JsonValue, path: &[&str], json: &JsonValue) -> JsonValue {
    match get(path, json) {
        Some(v) => v.clone(),
        None => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_json() -> JsonValue {
        JsonValue::Object(vec![
            (
                "users".to_string(),
                JsonValue::Array(vec![
                    JsonValue::Object(vec![
                        ("name".to_string(), JsonValue::Str("Alice".to_string())),
                        ("age".to_string(), JsonValue::Number(30.0)),
                        ("active".to_string(), JsonValue::Bool(true)),
                    ]),
                    JsonValue::Object(vec![
                        ("name".to_string(), JsonValue::Str("Bob".to_string())),
                        ("age".to_string(), JsonValue::Number(25.0)),
                        ("active".to_string(), JsonValue::Bool(false)),
                    ]),
                ]),
            ),
            ("count".to_string(), JsonValue::Number(2.0)),
            (
                "meta".to_string(),
                JsonValue::Object(vec![
                    ("version".to_string(), JsonValue::Str("1.0".to_string())),
                    ("tag".to_string(), JsonValue::Null),
                ]),
            ),
        ])
    }

    #[test]
    fn test_basic_queries() {
        let json = make_json();
        assert_eq!(get(&["count"], &json), Some(&JsonValue::Number(2.0)));
        assert_eq!(
            get(&["users", "0", "name"], &json),
            Some(&JsonValue::Str("Alice".to_string()))
        );
        assert_eq!(
            get(&["users", "1", "name"], &json),
            Some(&JsonValue::Str("Bob".to_string()))
        );
        assert_eq!(get(&["meta", "tag"], &json), Some(&JsonValue::Null));
    }

    #[test]
    fn test_missing_paths() {
        let json = make_json();
        assert_eq!(get(&["missing"], &json), None);
        assert_eq!(get(&["users", "5", "name"], &json), None);
        assert_eq!(get(&["users", "0", "missing"], &json), None);
    }

    #[test]
    fn test_typed_extractors() {
        let json = make_json();
        assert_eq!(get_string(&["users", "0", "name"], &json), Some("Alice"));
        assert_eq!(get_number(&["count"], &json), Some(2.0));
        assert_eq!(get_bool(&["users", "0", "active"], &json), Some(true));
        assert_eq!(get_bool(&["users", "1", "active"], &json), Some(false));
    }

    #[test]
    fn test_empty_path_returns_root() {
        let json = make_json();
        assert_eq!(get(&[], &json), Some(&json));
    }

    #[test]
    fn test_get_or_default() {
        let json = make_json();
        let result = get_or(JsonValue::Str("default".into()), &["missing"], &json);
        assert_eq!(result, JsonValue::Str("default".to_string()));
        let result2 = get_or(JsonValue::Null, &["count"], &json);
        assert_eq!(result2, JsonValue::Number(2.0));
    }
}
