//! Lifetime Cheatsheet
//!
//! Quick reference for common lifetime patterns.

// 'a: Lifetime parameter
// &'a T: Reference valid for 'a
// &'static T: Reference valid forever
// T: 'a: T outlives 'a

/// Elision: one input → output.
pub fn trim(s: &str) -> &str { s.trim() }

/// Explicit: multiple inputs.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// Struct with lifetime.
pub struct View<'a> { pub data: &'a str }

/// Impl with lifetime.
impl<'a> View<'a> {
    pub fn new(data: &'a str) -> Self { View { data } }
}

/// Static lifetime.
pub fn get_static() -> &'static str { "static" }

/// Bound: T outlives 'a.
pub fn bound<'a, T: 'a>(_t: &'a T) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() { assert_eq!(trim("  x  "), "x"); }

    #[test]
    fn test_longer() { assert_eq!(longer("ab", "abc"), "abc"); }

    #[test]
    fn test_view() {
        let v = View::new("test");
        assert_eq!(v.data, "test");
    }

    #[test]
    fn test_static() { assert_eq!(get_static(), "static"); }
}
