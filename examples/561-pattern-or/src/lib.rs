//! Or Patterns
//!
//! Matching multiple alternatives with |.

/// Match multiple values.
pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

/// Or pattern in match.
pub fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 | 2 | 3 => "small",
        4 | 5 | 6 => "medium",
        7 | 8 | 9 => "large",
        _ => "huge",
    }
}

/// Or in enum matching.
#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

pub fn is_primary(c: &Color) -> bool {
    matches!(c, Color::Red | Color::Green | Color::Blue)
}

/// Or with bindings (same name in each arm).
pub fn extract_value(opt: Option<Result<i32, i32>>) -> Option<i32> {
    match opt {
        Some(Ok(v) | Err(v)) => Some(v),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowel() {
        assert!(is_vowel('a'));
        assert!(!is_vowel('b'));
    }

    #[test]
    fn test_describe() {
        assert_eq!(describe_number(0), "zero");
        assert_eq!(describe_number(2), "small");
        assert_eq!(describe_number(5), "medium");
    }

    #[test]
    fn test_primary() {
        assert!(is_primary(&Color::Red));
        assert!(!is_primary(&Color::Yellow));
    }

    #[test]
    fn test_extract() {
        assert_eq!(extract_value(Some(Ok(5))), Some(5));
        assert_eq!(extract_value(Some(Err(3))), Some(3));
    }
}
