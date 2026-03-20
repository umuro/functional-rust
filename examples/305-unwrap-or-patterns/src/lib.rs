#![allow(clippy::all)]
//! # unwrap_or, unwrap_or_else, unwrap_or_default
//!
//! Safe alternatives to `unwrap()` when a default value is available.

/// Get value with eager default
pub fn get_or(opt: Option<i32>, default: i32) -> i32 {
    opt.unwrap_or(default)
}

/// Get value with lazy default
pub fn get_or_compute<F: FnOnce() -> i32>(opt: Option<i32>, f: F) -> i32 {
    opt.unwrap_or_else(f)
}

/// Get value using type's Default
pub fn get_or_default<T: Default>(opt: Option<T>) -> T {
    opt.unwrap_or_default()
}

/// Parse with default on failure
pub fn parse_or_default(s: &str, default: i32) -> i32 {
    s.parse::<i32>().unwrap_or(default)
}

/// Config-style loading with default
pub fn load_port(env_val: Option<String>) -> u16 {
    env_val.and_then(|s| s.parse().ok()).unwrap_or(8080)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_or_some() {
        assert_eq!(get_or(Some(5), 0), 5);
    }

    #[test]
    fn test_unwrap_or_none() {
        assert_eq!(get_or(None, 0), 0);
    }

    #[test]
    fn test_unwrap_or_else_not_called() {
        let mut called = false;
        let _ = get_or_compute(Some(5), || {
            called = true;
            0
        });
        assert!(!called);
    }

    #[test]
    fn test_unwrap_or_else_called() {
        let mut called = false;
        let val = get_or_compute(None, || {
            called = true;
            42
        });
        assert!(called);
        assert_eq!(val, 42);
    }

    #[test]
    fn test_unwrap_or_default_vec() {
        let v: Vec<i32> = get_or_default(None);
        assert!(v.is_empty());
    }

    #[test]
    fn test_unwrap_or_default_string() {
        let s: String = get_or_default(None);
        assert_eq!(s, "");
    }

    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42", 0), 42);
        assert_eq!(parse_or_default("bad", 0), 0);
    }

    #[test]
    fn test_load_port() {
        assert_eq!(load_port(Some("3000".to_string())), 3000);
        assert_eq!(load_port(None), 8080);
    }
}
