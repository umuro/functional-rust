//! String — Trim, Uppercase, Contains
//!
//! Common string operations: trimming whitespace, case conversion,
//! and substring search — idiomatic Rust vs. recursive OCaml style.

// Solution 1: Idiomatic Rust — method chaining on &str
pub fn trim_and_upper(s: &str) -> String {
    s.trim().to_uppercase()
}

pub fn trim_and_lower(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Returns true if `haystack` contains `needle` (case-sensitive).
/// Rust's `str::contains` accepts any pattern, including `&str`.
pub fn contains_substring(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

// Solution 2: Functional/recursive — mirrors the OCaml manual search
/// Recursively scans `s` for `needle` starting at byte index `i`.
/// Mirrors OCaml's hand-rolled `find` recursion.
pub fn contains_recursive(s: &str, needle: &str) -> bool {
    fn find(s: &str, needle: &str, i: usize) -> bool {
        if i + needle.len() > s.len() {
            false
        } else if s[i..].starts_with(needle) {
            true
        } else {
            find(s, needle, i + 1)
        }
    }
    find(s, needle, 0)
}

// Solution 3: Iterator chain — scan character windows
/// Finds `needle` by sliding a window across `s` using iterators.
pub fn contains_windowed(s: &str, needle: &str) -> bool {
    let n = needle.len();
    if n == 0 {
        return true;
    }
    (0..=s.len().saturating_sub(n)).any(|i| s[i..].starts_with(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- trim_and_upper ---

    #[test]
    fn test_trim_upper_leading_trailing_spaces() {
        assert_eq!(trim_and_upper("  hello  "), "HELLO");
    }

    #[test]
    fn test_trim_upper_no_spaces() {
        assert_eq!(trim_and_upper("world"), "WORLD");
    }

    #[test]
    fn test_trim_upper_already_upper() {
        assert_eq!(trim_and_upper("  RUST  "), "RUST");
    }

    #[test]
    fn test_trim_upper_empty_string() {
        assert_eq!(trim_and_upper(""), "");
    }

    // --- trim_and_lower ---

    #[test]
    fn test_trim_lower_mixed_case() {
        assert_eq!(trim_and_lower("  Hello, World!  "), "hello, world!");
    }

    #[test]
    fn test_trim_lower_only_whitespace() {
        assert_eq!(trim_and_lower("   "), "");
    }

    // --- contains_substring (idiomatic) ---

    #[test]
    fn test_contains_present() {
        assert!(contains_substring("Hello, World!", "World"));
    }

    #[test]
    fn test_contains_absent() {
        assert!(!contains_substring("Hello, World!", "Rust"));
    }

    #[test]
    fn test_contains_empty_needle() {
        assert!(contains_substring("anything", ""));
    }

    #[test]
    fn test_contains_empty_haystack() {
        assert!(!contains_substring("", "x"));
    }

    // --- contains_recursive ---

    #[test]
    fn test_recursive_found() {
        assert!(contains_recursive("  Hello, World!  ", "World"));
    }

    #[test]
    fn test_recursive_not_found() {
        assert!(!contains_recursive("  Hello, World!  ", "Rust"));
    }

    #[test]
    fn test_recursive_at_start() {
        assert!(contains_recursive("Rust is fast", "Rust"));
    }

    #[test]
    fn test_recursive_at_end() {
        assert!(contains_recursive("Hello Rust", "Rust"));
    }

    #[test]
    fn test_recursive_empty_needle() {
        assert!(contains_recursive("anything", ""));
    }

    // --- contains_windowed ---

    #[test]
    fn test_windowed_found() {
        assert!(contains_windowed("Hello, World!", "World"));
    }

    #[test]
    fn test_windowed_not_found() {
        assert!(!contains_windowed("Hello, World!", "Rust"));
    }

    #[test]
    fn test_windowed_empty_haystack() {
        assert!(!contains_windowed("", "x"));
    }

    #[test]
    fn test_windowed_empty_needle() {
        assert!(contains_windowed("hello", ""));
    }

    // --- all three contain impls agree ---

    #[test]
    fn test_all_contains_agree() {
        let cases = [
            ("  Hello, World!  ", "World", true),
            ("  Hello, World!  ", "Rust", false),
            ("abcdef", "cde", true),
            ("abcdef", "xyz", false),
        ];
        for (s, needle, expected) in cases {
            assert_eq!(contains_substring(s, needle), expected, "idiomatic: {s:?}");
            assert_eq!(contains_recursive(s, needle), expected, "recursive: {s:?}");
            assert_eq!(contains_windowed(s, needle), expected, "windowed: {s:?}");
        }
    }
}
