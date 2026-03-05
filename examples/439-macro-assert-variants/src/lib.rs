//! Assert Variant Macros
//!
//! Testing enum variants.

/// Assert that value matches pattern.
#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:pat) => {
        assert!(
            matches!($value, $pattern),
            "assertion failed: `{:?}` does not match `{}`",
            $value,
            stringify!($pattern)
        );
    };
}

/// Extract variant or panic.
#[macro_export]
macro_rules! unwrap_variant {
    ($value:expr, $pattern:pat => $extracted:expr) => {
        match $value {
            $pattern => $extracted,
            _ => panic!("Expected {}", stringify!($pattern)),
        }
    };
}

#[derive(Debug)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
pub enum Message {
    Text(String),
    Number(i32),
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_matches_ok() {
        let r: std::result::Result<i32, &str> = Ok(42);
        assert_matches!(r, Ok(_));
    }

    #[test]
    fn test_assert_matches_some() {
        let o = Some(5);
        assert_matches!(o, Some(_));
    }

    #[test]
    fn test_unwrap_variant() {
        let m = Message::Number(42);
        let n = unwrap_variant!(m, Message::Number(x) => x);
        assert_eq!(n, 42);
    }

    #[test]
    fn test_assert_matches_text() {
        let m = Message::Text("hello".into());
        assert_matches!(m, Message::Text(_));
    }

    #[test]
    fn test_assert_matches_empty() {
        let m = Message::Empty;
        assert_matches!(m, Message::Empty);
    }
}
