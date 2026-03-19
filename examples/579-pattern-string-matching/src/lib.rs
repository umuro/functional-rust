#![allow(clippy::all)]
//! # String Pattern Matching
//!
//! Match against string literals, with OR patterns and guards.

/// Classify a command string.
pub fn classify_cmd(s: &str) -> &'static str {
    match s {
        "quit" | "exit" | "q" => "quit",
        "help" | "?" | "h" => "help",
        s if s.starts_with('/') => "command",
        "" => "empty",
        _ => "unknown",
    }
}

/// Alternative using if-else chain.
pub fn classify_cmd_if(s: &str) -> &'static str {
    if s == "quit" || s == "exit" || s == "q" {
        "quit"
    } else if s == "help" || s == "?" || s == "h" {
        "help"
    } else if s.starts_with('/') {
        "command"
    } else if s.is_empty() {
        "empty"
    } else {
        "unknown"
    }
}

/// Classify a day as weekday or weekend.
pub fn day_type(d: &str) -> &'static str {
    match d {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        "Saturday" | "Sunday" => "weekend",
        _ => "unknown",
    }
}

/// Alternative using array contains.
pub fn day_type_array(d: &str) -> &'static str {
    const WEEKDAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    const WEEKEND: [&str; 2] = ["Saturday", "Sunday"];

    if WEEKDAYS.contains(&d) {
        "weekday"
    } else if WEEKEND.contains(&d) {
        "weekend"
    } else {
        "unknown"
    }
}

/// Classify HTTP method.
pub fn http_method(m: &str) -> &'static str {
    match m {
        "GET" => "read",
        "POST" | "PUT" | "PATCH" => "write",
        "DELETE" => "delete",
        _ => "unknown",
    }
}

/// Personalized greeting based on name patterns.
pub fn greet(name: &str) -> String {
    match name {
        "Alice" => "Hello, Admin Alice!".into(),
        "" => "Hello, stranger!".into(),
        n if n.starts_with("Dr.") => format!("Good day, {}!", n),
        n => format!("Hi, {}!", n),
    }
}

/// Alternative using if-else.
pub fn greet_if(name: &str) -> String {
    if name == "Alice" {
        "Hello, Admin Alice!".into()
    } else if name.is_empty() {
        "Hello, stranger!".into()
    } else if name.starts_with("Dr.") {
        format!("Good day, {}!", name)
    } else {
        format!("Hi, {}!", name)
    }
}

/// Parse a boolean string.
pub fn parse_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "yes" | "1" | "on" => Some(true),
        "false" | "no" | "0" | "off" => Some(false),
        _ => None,
    }
}

/// Match with case-insensitive comparison using guard.
pub fn is_affirmative(s: &str) -> bool {
    match s {
        s if s.eq_ignore_ascii_case("yes") => true,
        s if s.eq_ignore_ascii_case("true") => true,
        s if s.eq_ignore_ascii_case("ok") => true,
        _ => false,
    }
}

/// Alternative using matches! and or_else.
pub fn is_affirmative_simple(s: &str) -> bool {
    matches!(s.to_lowercase().as_str(), "yes" | "true" | "ok")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_cmd_quit() {
        assert_eq!(classify_cmd("quit"), "quit");
        assert_eq!(classify_cmd("exit"), "quit");
        assert_eq!(classify_cmd("q"), "quit");
    }

    #[test]
    fn test_classify_cmd_help() {
        assert_eq!(classify_cmd("help"), "help");
        assert_eq!(classify_cmd("?"), "help");
        assert_eq!(classify_cmd("h"), "help");
    }

    #[test]
    fn test_classify_cmd_slash() {
        assert_eq!(classify_cmd("/run"), "command");
        assert_eq!(classify_cmd("/help"), "command");
    }

    #[test]
    fn test_classify_cmd_empty_unknown() {
        assert_eq!(classify_cmd(""), "empty");
        assert_eq!(classify_cmd("foo"), "unknown");
    }

    #[test]
    fn test_classify_approaches_equivalent() {
        let cases = ["quit", "exit", "help", "?", "/run", "", "foo"];
        for s in cases {
            assert_eq!(classify_cmd(s), classify_cmd_if(s));
        }
    }

    #[test]
    fn test_day_type_weekday() {
        for d in ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"] {
            assert_eq!(day_type(d), "weekday");
        }
    }

    #[test]
    fn test_day_type_weekend() {
        assert_eq!(day_type("Saturday"), "weekend");
        assert_eq!(day_type("Sunday"), "weekend");
    }

    #[test]
    fn test_day_type_unknown() {
        assert_eq!(day_type("Holiday"), "unknown");
        assert_eq!(day_type(""), "unknown");
    }

    #[test]
    fn test_day_type_approaches_equivalent() {
        let days = ["Monday", "Saturday", "Holiday", ""];
        for d in days {
            assert_eq!(day_type(d), day_type_array(d));
        }
    }

    #[test]
    fn test_http_method() {
        assert_eq!(http_method("GET"), "read");
        assert_eq!(http_method("POST"), "write");
        assert_eq!(http_method("PUT"), "write");
        assert_eq!(http_method("PATCH"), "write");
        assert_eq!(http_method("DELETE"), "delete");
        assert_eq!(http_method("WUT"), "unknown");
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Admin Alice!");
        assert_eq!(greet(""), "Hello, stranger!");
        assert_eq!(greet("Dr.Smith"), "Good day, Dr.Smith!");
        assert_eq!(greet("Bob"), "Hi, Bob!");
    }

    #[test]
    fn test_greet_approaches_equivalent() {
        let names = ["Alice", "", "Dr.Smith", "Bob"];
        for n in names {
            assert_eq!(greet(n), greet_if(n));
        }
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("true"), Some(true));
        assert_eq!(parse_bool("TRUE"), Some(true));
        assert_eq!(parse_bool("yes"), Some(true));
        assert_eq!(parse_bool("1"), Some(true));
        assert_eq!(parse_bool("false"), Some(false));
        assert_eq!(parse_bool("no"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }

    #[test]
    fn test_is_affirmative() {
        assert!(is_affirmative("yes"));
        assert!(is_affirmative("YES"));
        assert!(is_affirmative("Yes"));
        assert!(is_affirmative("true"));
        assert!(is_affirmative("ok"));
        assert!(!is_affirmative("no"));
        assert!(!is_affirmative("maybe"));
    }

    #[test]
    fn test_is_affirmative_approaches_equivalent() {
        let cases = ["yes", "YES", "true", "ok", "no", "maybe"];
        for s in cases {
            assert_eq!(is_affirmative(s), is_affirmative_simple(s));
        }
    }
}
