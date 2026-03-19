#![allow(clippy::all)]
//! Higher-Ranked Trait Bounds (for<'a>)
//!
//! Universal quantification over lifetimes for flexible callbacks.

/// Without HRTB: lifetime fixed at call site.
pub fn apply_fixed<'a, F>(f: F, s: &'a str) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(s)
}

/// With HRTB: F works for ANY lifetime.
pub fn apply_hrtb<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s).to_string()
}

/// HRTB in trait definitions.
pub trait Processor {
    fn process<'a>(&self, input: &'a str) -> &'a str;
}

/// Common HRTB pattern: Fn(&T) -> &T.
pub fn transform_all<F>(items: &[String], f: F) -> Vec<String>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    items.iter().map(|s| f(s).to_string()).collect()
}

/// Identity processor (for<'a> Fn(&'a str) -> &'a str).
pub fn identity(s: &str) -> &str {
    s
}

/// Trim processor.
pub fn trim(s: &str) -> &str {
    s.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_fixed() {
        let s = String::from("  hello  ");
        let result = apply_fixed(|x| x.trim(), &s);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_apply_hrtb() {
        let s = "  world  ";
        let result = apply_hrtb(|x| x.trim(), s);
        assert_eq!(result, "world");
    }

    #[test]
    fn test_transform_all() {
        let items = vec![String::from("  a  "), String::from("  b  ")];
        let result = transform_all(&items, trim);
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_identity() {
        let s = "hello";
        assert_eq!(identity(s), "hello");
    }
}
