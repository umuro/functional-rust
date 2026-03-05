//! Clone-on-Write (Cow) Pattern
//!
//! Avoid allocation when data doesn't need to be modified.

use std::borrow::Cow;

// === Approach 1: String processing ===

/// Replace spaces with underscores, only allocating if needed
pub fn ensure_no_spaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}

/// Truncate string to limit, only allocating if needed
pub fn truncate_to_limit(s: &str, limit: usize) -> Cow<str> {
    if s.len() <= limit {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s[..limit].to_string())
    }
}

/// Normalize whitespace (collapse multiple spaces, trim)
pub fn normalize_whitespace(input: &str) -> Cow<str> {
    // Check if normalization is needed
    let needs_normalization =
        input.contains("  ") || input.starts_with(' ') || input.ends_with(' ');

    if !needs_normalization {
        Cow::Borrowed(input)
    } else {
        let mut result = String::with_capacity(input.len());
        let mut prev_space = true; // start true to trim leading
        for c in input.chars() {
            if c == ' ' {
                if !prev_space {
                    result.push(c);
                }
                prev_space = true;
            } else {
                result.push(c);
                prev_space = false;
            }
        }
        // Trim trailing
        while result.ends_with(' ') {
            result.pop();
        }
        Cow::Owned(result)
    }
}

// === Approach 2: Converting to uppercase conditionally ===

/// Convert to uppercase only if needed
pub fn to_uppercase_if_needed(s: &str) -> Cow<str> {
    if s.chars().all(|c| !c.is_lowercase()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_uppercase())
    }
}

/// Convert to lowercase only if needed
pub fn to_lowercase_if_needed(s: &str) -> Cow<str> {
    if s.chars().all(|c| !c.is_uppercase()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_lowercase())
    }
}

// === Approach 3: Escape special characters ===

/// Escape HTML special characters, only allocating if needed
pub fn escape_html(s: &str) -> Cow<str> {
    if !s.contains(['&', '<', '>', '"', '\'']) {
        Cow::Borrowed(s)
    } else {
        let mut result = String::with_capacity(s.len() + 10);
        for c in s.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&#39;"),
                _ => result.push(c),
            }
        }
        Cow::Owned(result)
    }
}

/// URL-encode a string, only allocating if needed
pub fn url_encode(s: &str) -> Cow<str> {
    let needs_encoding = s.chars().any(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~'));

    if !needs_encoding {
        Cow::Borrowed(s)
    } else {
        let mut result = String::with_capacity(s.len() * 3);
        for c in s.chars() {
            if matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~') {
                result.push(c);
            } else {
                for byte in c.to_string().bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
        Cow::Owned(result)
    }
}

/// Check if Cow is borrowed (no allocation occurred)
pub fn is_borrowed<T: ?Sized + ToOwned>(cow: &Cow<T>) -> bool {
    matches!(cow, Cow::Borrowed(_))
}

/// Check if Cow is owned (allocation occurred)
pub fn is_owned<T: ?Sized + ToOwned>(cow: &Cow<T>) -> bool {
    matches!(cow, Cow::Owned(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_spaces_borrowed() {
        let result = ensure_no_spaces("hello");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_has_spaces_owned() {
        let result = ensure_no_spaces("hello world");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_truncate_no_change() {
        let result = truncate_to_limit("hello", 10);
        assert!(is_borrowed(&result));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_truncate_needed() {
        let result = truncate_to_limit("hello world", 5);
        assert!(is_owned(&result));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_normalize_whitespace_no_change() {
        let result = normalize_whitespace("hello world");
        assert!(is_borrowed(&result));
    }

    #[test]
    fn test_normalize_whitespace_needed() {
        let result = normalize_whitespace("  hello   world  ");
        assert!(is_owned(&result));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_uppercase_no_change() {
        let result = to_uppercase_if_needed("HELLO");
        assert!(is_borrowed(&result));
    }

    #[test]
    fn test_uppercase_needed() {
        let result = to_uppercase_if_needed("Hello");
        assert!(is_owned(&result));
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_escape_html_no_change() {
        let result = escape_html("hello world");
        assert!(is_borrowed(&result));
    }

    #[test]
    fn test_escape_html_needed() {
        let result = escape_html("<script>");
        assert!(is_owned(&result));
        assert_eq!(result, "&lt;script&gt;");
    }

    #[test]
    fn test_url_encode_no_change() {
        let result = url_encode("hello-world_123");
        assert!(is_borrowed(&result));
    }

    #[test]
    fn test_url_encode_needed() {
        let result = url_encode("hello world");
        assert!(is_owned(&result));
        assert_eq!(result, "hello%20world");
    }
}
