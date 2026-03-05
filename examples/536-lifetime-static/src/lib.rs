//! 'static Lifetime
//!
//! Program-duration references: literals, statics, and 'static bounds.

/// String literals are &'static str — embedded in binary.
pub static APP_NAME: &str = "MyRustApp";
pub static VERSION: &str = "1.0.0";
pub static MAX_CONNECTIONS: usize = 100;

/// Static slice.
pub static ERROR_MESSAGES: &[(u16, &str)] = &[
    (404, "Not Found"),
    (500, "Internal Server Error"),
    (403, "Forbidden"),
    (200, "OK"),
];

pub fn get_error_msg(code: u16) -> &'static str {
    ERROR_MESSAGES
        .iter()
        .find(|&&(c, _)| c == code)
        .map(|(_, msg)| *msg)
        .unwrap_or("Unknown Error")
}

/// Return a string literal — always 'static.
pub fn get_greeting() -> &'static str {
    "Hello, World!"
}

/// Owned data can satisfy 'static bound.
pub fn make_static_string() -> String {
    // String is 'static because it owns its data (no borrows)
    String::from("I am owned")
}

/// Function accepting 'static data.
pub fn store_static(s: &'static str) -> &'static str {
    s
}

/// Thread spawn requires 'static (data must outlive thread).
pub fn spawn_example() {
    let owned = String::from("owned data");
    // std::thread::spawn requires 'static
    // move closure transfers ownership
    let _handle = std::thread::spawn(move || {
        println!("{}", owned);
    });
}

/// 'static bound on generic: T must not contain non-static borrows.
pub fn needs_static<T: 'static>(value: T) -> T {
    value
}

/// Lazy static pattern (without lazy_static crate).
use std::sync::OnceLock;

static CONFIG: OnceLock<String> = OnceLock::new();

pub fn get_config() -> &'static str {
    CONFIG.get_or_init(|| String::from("default_config"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_str() {
        let s: &'static str = "literal";
        assert_eq!(s, "literal");
    }

    #[test]
    fn test_static_constants() {
        assert_eq!(APP_NAME, "MyRustApp");
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_get_error_msg() {
        assert_eq!(get_error_msg(404), "Not Found");
        assert_eq!(get_error_msg(999), "Unknown Error");
    }

    #[test]
    fn test_get_greeting() {
        let greeting: &'static str = get_greeting();
        assert_eq!(greeting, "Hello, World!");
    }

    #[test]
    fn test_needs_static_owned() {
        // String satisfies 'static because it owns its data
        let s = needs_static(String::from("hello"));
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_needs_static_literal() {
        // &'static str satisfies 'static
        let s = needs_static("literal");
        assert_eq!(s, "literal");
    }

    #[test]
    fn test_get_config() {
        let config = get_config();
        assert_eq!(config, "default_config");
    }

    #[test]
    fn test_store_static() {
        let result = store_static("hello");
        assert_eq!(result, "hello");
    }
}
