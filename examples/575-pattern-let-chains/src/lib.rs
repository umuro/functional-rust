//! # Let Chains (&&)
//!
//! Chain multiple pattern checks with `&&` — combine pattern matching
//! and boolean conditions without nesting.
//!
//! Requires Rust 1.88+ for stable let chains.

/// Process a string, returning doubled value if it's a positive even number.
///
/// Uses let chains to combine parsing, positivity check, and evenness check
/// in a single flat condition.
pub fn process(s: &str) -> Option<i32> {
    if let Ok(n) = s.parse::<i32>() && n > 0 && n % 2 == 0 {
        Some(n * 2)
    } else {
        None
    }
}

/// Alternative approach using traditional nested if-let (pre-1.88 style).
pub fn process_nested(s: &str) -> Option<i32> {
    if let Ok(n) = s.parse::<i32>() {
        if n > 0 {
            if n % 2 == 0 {
                return Some(n * 2);
            }
        }
    }
    None
}

/// Alternative approach using Option combinators.
pub fn process_combinators(s: &str) -> Option<i32> {
    s.parse::<i32>()
        .ok()
        .filter(|&n| n > 0)
        .filter(|&n| n % 2 == 0)
        .map(|n| n * 2)
}

/// Configuration with optional host and port.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Config {
    pub fn new(host: Option<String>, port: Option<u16>) -> Self {
        Self { host, port }
    }
}

/// Create an address string from config using let chains.
///
/// Validates that both host and port exist and are valid.
pub fn make_addr(cfg: &Config) -> Option<String> {
    if let Some(ref host) = cfg.host
        && let Some(port) = cfg.port
        && !host.is_empty()
        && port > 0
    {
        Some(format!("{}:{}", host, port))
    } else {
        None
    }
}

/// Alternative using Option::zip and filter.
pub fn make_addr_combinators(cfg: &Config) -> Option<String> {
    cfg.host
        .as_ref()
        .zip(cfg.port)
        .filter(|(host, port)| !host.is_empty() && *port > 0)
        .map(|(host, port)| format!("{}:{}", host, port))
}

/// Find the first positive even number in a slice of string representations.
pub fn first_positive_even(data: &[&str]) -> Option<i32> {
    for &s in data {
        if let Ok(n) = s.parse::<i32>() && n > 0 && n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

/// Alternative using iterators.
pub fn first_positive_even_iter(data: &[&str]) -> Option<i32> {
    data.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .find(|&n| n > 0 && n % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_valid_positive_even() {
        assert_eq!(process("4"), Some(8));
        assert_eq!(process("8"), Some(16));
        assert_eq!(process("100"), Some(200));
    }

    #[test]
    fn test_process_negative() {
        assert_eq!(process("-2"), None);
        assert_eq!(process("-4"), None);
    }

    #[test]
    fn test_process_odd() {
        assert_eq!(process("3"), None);
        assert_eq!(process("7"), None);
    }

    #[test]
    fn test_process_invalid_string() {
        assert_eq!(process("abc"), None);
        assert_eq!(process(""), None);
        assert_eq!(process("4.5"), None);
    }

    #[test]
    fn test_process_approaches_equivalent() {
        let test_cases = ["4", "-2", "3", "abc", "8", "0", "10"];
        for s in test_cases {
            assert_eq!(
                process(s),
                process_nested(s),
                "Mismatch for input: {}",
                s
            );
            assert_eq!(
                process(s),
                process_combinators(s),
                "Mismatch for input: {}",
                s
            );
        }
    }

    #[test]
    fn test_make_addr_valid() {
        let cfg = Config::new(Some("localhost".into()), Some(8080));
        assert_eq!(make_addr(&cfg), Some("localhost:8080".to_string()));
    }

    #[test]
    fn test_make_addr_empty_host() {
        let cfg = Config::new(Some("".into()), Some(8080));
        assert_eq!(make_addr(&cfg), None);
    }

    #[test]
    fn test_make_addr_missing_fields() {
        let cfg1 = Config::new(None, Some(80));
        let cfg2 = Config::new(Some("host".into()), None);
        assert_eq!(make_addr(&cfg1), None);
        assert_eq!(make_addr(&cfg2), None);
    }

    #[test]
    fn test_make_addr_zero_port() {
        let cfg = Config::new(Some("localhost".into()), Some(0));
        assert_eq!(make_addr(&cfg), None);
    }

    #[test]
    fn test_make_addr_approaches_equivalent() {
        let configs = [
            Config::new(Some("localhost".into()), Some(8080)),
            Config::new(Some("".into()), Some(8080)),
            Config::new(None, Some(80)),
            Config::new(Some("host".into()), None),
        ];
        for cfg in &configs {
            assert_eq!(make_addr(cfg), make_addr_combinators(cfg));
        }
    }

    #[test]
    fn test_first_positive_even() {
        assert_eq!(first_positive_even(&["x", "3", "-4", "6", "8"]), Some(6));
        assert_eq!(first_positive_even(&["1", "3", "5"]), None);
        assert_eq!(first_positive_even(&["-2", "-4"]), None);
        assert_eq!(first_positive_even(&["2"]), Some(2));
    }

    #[test]
    fn test_first_positive_even_approaches_equivalent() {
        let test_cases: &[&[&str]] = &[
            &["x", "3", "-4", "6", "8"],
            &["1", "3", "5"],
            &["-2", "-4"],
            &["2"],
            &[],
        ];
        for data in test_cases {
            assert_eq!(first_positive_even(data), first_positive_even_iter(data));
        }
    }
}
