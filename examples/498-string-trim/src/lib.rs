//! # String Trim — Whitespace Handling

pub fn trim_all(s: &str) -> &str {
    s.trim()
}

pub fn trim_start(s: &str) -> &str {
    s.trim_start()
}

pub fn trim_end(s: &str) -> &str {
    s.trim_end()
}

pub fn trim_chars(s: &str, chars: &[char]) -> &str {
    s.trim_matches(chars)
}

pub fn trim_prefix(s: &str, prefix: &str) -> &str {
    s.strip_prefix(prefix).unwrap_or(s)
}

pub fn trim_suffix(s: &str, suffix: &str) -> &str {
    s.strip_suffix(suffix).unwrap_or(s)
}

pub fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        assert_eq!(trim_all("  hello  "), "hello");
        assert_eq!(trim_start("  hello  "), "hello  ");
        assert_eq!(trim_end("  hello  "), "  hello");
    }

    #[test]
    fn test_trim_chars() {
        assert_eq!(trim_chars("##hello##", &['#']), "hello");
    }

    #[test]
    fn test_strip() {
        assert_eq!(trim_prefix("hello world", "hello "), "world");
        assert_eq!(trim_suffix("hello world", " world"), "hello");
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize_whitespace("  a   b   c  "), "a b c");
    }
}
