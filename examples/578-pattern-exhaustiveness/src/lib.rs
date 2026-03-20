#![allow(clippy::all)]
//! # Pattern Exhaustiveness
//!
//! Rust's match expressions must cover all possible cases at compile time.

/// Direction enum for demonstration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

/// Describe a direction - all cases covered, no wildcard needed.
pub fn describe(d: Dir) -> &'static str {
    match d {
        Dir::N => "north",
        Dir::S => "south",
        Dir::E => "east",
        Dir::W => "west",
        // No _ needed: all variants covered → compile-time guarantee!
    }
}

/// Check if direction is horizontal.
pub fn horizontal(d: Dir) -> bool {
    match d {
        Dir::E | Dir::W => true,
        _ => false,
    }
}

/// Alternative using matches! macro.
pub fn horizontal_matches(d: Dir) -> bool {
    matches!(d, Dir::E | Dir::W)
}

/// Library-style enum with #[non_exhaustive] for future extensibility.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok,
    NotFound,
    Unauthorized,
    ServerError,
}

/// Convert status code to text - requires wildcard due to #[non_exhaustive].
pub fn status_text(c: StatusCode) -> &'static str {
    match c {
        StatusCode::Ok => "OK",
        StatusCode::NotFound => "Not Found",
        StatusCode::Unauthorized => "Unauthorized",
        StatusCode::ServerError => "Internal Server Error",
        _ => "Unknown", // required by #[non_exhaustive]
    }
}

/// Classify an integer - exhaustive range matching.
pub fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=i32::MAX => "positive",
    }
}

/// Alternative classification using conditionals.
pub fn classify_if(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}

/// Result-based exhaustiveness example.
pub fn handle_result<T: std::fmt::Debug, E: std::fmt::Debug>(r: Result<T, E>) -> String {
    match r {
        Ok(v) => format!("Success: {:?}", v),
        Err(e) => format!("Error: {:?}", e),
        // Both variants covered - exhaustive
    }
}

/// Option-based exhaustiveness.
pub fn option_to_string<T: std::fmt::Display>(opt: Option<T>) -> String {
    match opt {
        Some(v) => v.to_string(),
        None => "None".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_all_directions() {
        assert_eq!(describe(Dir::N), "north");
        assert_eq!(describe(Dir::S), "south");
        assert_eq!(describe(Dir::E), "east");
        assert_eq!(describe(Dir::W), "west");
    }

    #[test]
    fn test_horizontal() {
        assert!(horizontal(Dir::E));
        assert!(horizontal(Dir::W));
        assert!(!horizontal(Dir::N));
        assert!(!horizontal(Dir::S));
    }

    #[test]
    fn test_horizontal_approaches_equivalent() {
        for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
            assert_eq!(horizontal(d), horizontal_matches(d));
        }
    }

    #[test]
    fn test_status_text() {
        assert_eq!(status_text(StatusCode::Ok), "OK");
        assert_eq!(status_text(StatusCode::NotFound), "Not Found");
        assert_eq!(status_text(StatusCode::Unauthorized), "Unauthorized");
        assert_eq!(
            status_text(StatusCode::ServerError),
            "Internal Server Error"
        );
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify(-100), "negative");
        assert_eq!(classify(-1), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(1), "positive");
        assert_eq!(classify(100), "positive");
    }

    #[test]
    fn test_classify_approaches_equivalent() {
        for n in [-100, -1, 0, 1, 100, i32::MIN, i32::MAX] {
            assert_eq!(classify(n), classify_if(n));
        }
    }

    #[test]
    fn test_handle_result() {
        let ok: Result<i32, &str> = Ok(42);
        let err: Result<i32, &str> = Err("oops");
        assert!(handle_result(ok).contains("42"));
        assert!(handle_result(err).contains("oops"));
    }

    #[test]
    fn test_option_to_string() {
        assert_eq!(option_to_string(Some(42)), "42");
        assert_eq!(option_to_string::<i32>(None), "None");
    }
}
