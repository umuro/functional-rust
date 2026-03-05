//! # String Case — Case Conversion

pub fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

pub fn to_lowercase(s: &str) -> String {
    s.to_lowercase()
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn is_all_uppercase(s: &str) -> bool {
    s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

pub fn is_all_lowercase(s: &str) -> bool {
    s.chars().all(|c| !c.is_alphabetic() || c.is_lowercase())
}

pub fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_lowercase("HELLO"), "hello");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello world"), "Hello World");
    }

    #[test]
    fn test_checks() {
        assert!(is_all_uppercase("HELLO"));
        assert!(is_all_lowercase("hello"));
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap_case("Hello"), "hELLO");
    }
}
