//! # Copy-on-Write Pattern

use std::borrow::Cow;

/// Process string, only allocating if modification needed
pub fn process_name(name: &str) -> Cow<str> {
    if name.chars().all(|c| c.is_ascii_alphabetic()) {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(name.chars().filter(|c| c.is_ascii_alphabetic()).collect())
    }
}

/// Cow for avoiding clones
pub fn maybe_modify(data: Cow<[i32]>, threshold: i32) -> Cow<[i32]> {
    if data.iter().any(|&x| x > threshold) {
        Cow::Owned(data.iter().map(|&x| if x > threshold { threshold } else { x }).collect())
    } else {
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cow_no_alloc() {
        let name = process_name("Alice");
        assert!(matches!(name, Cow::Borrowed(_)));
    }
    #[test]
    fn test_cow_alloc() {
        let name = process_name("Alice123");
        assert!(matches!(name, Cow::Owned(_)));
        assert_eq!(name, "Alice");
    }
}
