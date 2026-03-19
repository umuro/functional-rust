// 484. Cow<str> for flexible strings
use std::borrow::Cow;

fn ensure_no_spaces(s: &str) -> Cow<str> {
    if !s.contains(' ') {
        Cow::Borrowed(s) // no allocation!
    } else {
        Cow::Owned(s.replace(' ', "_")) // allocates only when needed
    }
}

fn to_uppercase_if_needed(s: Cow<str>) -> Cow<str> {
    if s.chars().any(|c| c.is_lowercase()) {
        Cow::Owned(s.to_uppercase())
    } else {
        s // pass through unchanged
    }
}

fn process(input: Cow<str>) -> String {
    // Can call String methods on Cow<str> via deref
    format!("processed: {}", input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_alloc() {
        assert!(matches!(ensure_no_spaces("nospace"), Cow::Borrowed(_)));
    }
    #[test]
    fn test_allocs() {
        assert!(matches!(ensure_no_spaces("has space"), Cow::Owned(_)));
    }
    #[test]
    fn test_content() {
        assert_eq!(&*ensure_no_spaces("a b"), "a_b");
    }
}
