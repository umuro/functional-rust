#![allow(clippy::all)]
//! Lifetime Annotations: 'a Basics
//!
//! Explicit lifetime parameters expressing reference validity constraints.

/// Return the longer of two string slices.
/// 'a means: output valid as long as BOTH inputs are valid.
pub fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

/// First word of a string: output tied to input's lifetime.
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// Two different lifetimes: x and y may have different scopes.
pub fn pick_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x // output tied to 'a only
}

/// Struct holding a reference — must annotate lifetime.
#[derive(Debug)]
pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Excerpt { part: text }
    }

    pub fn part(&self) -> &str {
        self.part
    }
}

/// Function returning a reference to one of its inputs.
pub fn min_by_len<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() <= b.len() {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer() {
        let s1 = "hello";
        let s2 = "hi";
        assert_eq!(longer(s1, s2), "hello");
    }

    #[test]
    fn test_longer_equal() {
        let s1 = "abc";
        let s2 = "xyz";
        assert_eq!(longer(s1, s2), "abc"); // first wins on tie
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_pick_first() {
        let x = "first";
        let y = "second";
        assert_eq!(pick_first(x, y), "first");
    }

    #[test]
    fn test_excerpt() {
        let text = String::from("Call me Ishmael.");
        let excerpt = Excerpt::new(&text[..7]);
        assert_eq!(excerpt.part(), "Call me");
    }

    #[test]
    fn test_min_by_len() {
        assert_eq!(min_by_len("hi", "hello"), "hi");
        assert_eq!(min_by_len("abc", "ab"), "ab");
    }

    #[test]
    fn test_lifetime_scope() {
        let result;
        {
            let s1 = String::from("long string");
            let s2 = String::from("short");
            result = longer(&s1, &s2);
            assert_eq!(result, "long string");
        }
        // result is no longer valid here because s1, s2 are dropped
    }
}
