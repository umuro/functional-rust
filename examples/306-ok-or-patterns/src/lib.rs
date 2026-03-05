//! # ok_or and ok_or_else
//!
//! `ok_or(err)` converts `Option<T>` to `Result<T, E>`.

use std::collections::HashMap;

/// Lookup with descriptive error
pub fn lookup<'a>(map: &'a HashMap<&str, &str>, key: &str) -> Result<&'a str, String> {
    map.get(key).copied().ok_or_else(|| format!("key '{}' not found", key))
}

/// Get port from config
pub fn get_port(config: &HashMap<&str, &str>) -> Result<u16, String> {
    let s = config.get("port").copied().ok_or("port not set")?;
    s.parse::<u16>().map_err(|e| format!("invalid port: {}", e))
}

/// Convert Option to Result with eager error
pub fn require_some<T>(opt: Option<T>, err: &str) -> Result<T, String> {
    opt.ok_or_else(|| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_or_some() {
        assert_eq!(Some(5i32).ok_or("missing"), Ok(5));
    }

    #[test]
    fn test_ok_or_none() {
        assert_eq!(None::<i32>.ok_or("missing"), Err("missing"));
    }

    #[test]
    fn test_ok_or_else_lazy() {
        let mut called = false;
        let _: Result<i32, &str> = Some(5).ok_or_else(|| { called = true; "err" });
        assert!(!called);
    }

    #[test]
    fn test_lookup_found() {
        let mut map = HashMap::new();
        map.insert("host", "localhost");
        assert_eq!(lookup(&map, "host").unwrap(), "localhost");
    }

    #[test]
    fn test_lookup_missing() {
        let map: HashMap<&str, &str> = HashMap::new();
        assert!(lookup(&map, "missing").is_err());
    }

    #[test]
    fn test_get_port() {
        let mut map = HashMap::new();
        map.insert("port", "8080");
        assert_eq!(get_port(&map).unwrap(), 8080);
    }
}
