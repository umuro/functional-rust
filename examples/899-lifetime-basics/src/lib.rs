// Example 899: Lifetime Basics
//
// Lifetimes ensure references don't outlive the data they point to.
// OCaml's GC handles this automatically; Rust proves it at compile time.

// Approach 1: Idiomatic — lifetime annotation on a function
// Tells the compiler: the returned reference lives no longer than both inputs.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

// Approach 2: Returning a reference tied to a single input
// 'a says: the returned &str borrows from `s`, not from `prefix`.
pub fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

// Approach 3: Struct with a lifetime
// The struct cannot outlive the string slice it holds.
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    // Lifetime elision applies: returned &str borrows from `self`.
    pub fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

// Approach 4: Functional/recursive — annotated helper
// Finds the longest string in a slice, returning a reference into it.
pub fn longest_in<'a>(strs: &[&'a str]) -> Option<&'a str> {
    strs.iter().copied().max_by_key(|s| s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_first_wins_on_tie() {
        let a = "hello";
        let b = "world";
        assert_eq!(longest(a, b), "hello");
    }

    #[test]
    fn test_longest_picks_longer() {
        let a = "short";
        let b = "much longer string";
        assert_eq!(longest(a, b), "much longer string");
    }

    #[test]
    fn test_trim_prefix_removes_prefix() {
        assert_eq!(trim_prefix("Hello, Alice!", "Hello, "), "Alice!");
    }

    #[test]
    fn test_trim_prefix_no_match_returns_original() {
        assert_eq!(trim_prefix("Hello, Alice!", "Bye, "), "Hello, Alice!");
    }

    #[test]
    fn test_excerpt_first_word() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let excerpt = Excerpt::new(&novel);
        assert_eq!(excerpt.first_word(), "Call");
    }

    #[test]
    fn test_excerpt_single_word() {
        let word = String::from("Rust");
        let excerpt = Excerpt::new(&word);
        assert_eq!(excerpt.first_word(), "Rust");
    }

    #[test]
    fn test_longest_in_finds_max() {
        let strs = vec!["cat", "elephant", "ox"];
        assert_eq!(longest_in(&strs), Some("elephant"));
    }

    #[test]
    fn test_longest_in_empty_returns_none() {
        let strs: Vec<&str> = vec![];
        assert_eq!(longest_in(&strs), None);
    }
}
