// 498. Safe Unicode truncation
fn truncate_bytes(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    // floor_char_boundary available in Rust 1.72+
    // For compatibility, implement manually:
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

fn truncate_chars(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        Some((byte_pos, _)) => &s[..byte_pos],
        None => s, // shorter than max_chars
    }
}

fn truncate_with_ellipsis(s: &str, max_chars: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_chars {
        return s.to_string();
    }
    let truncated = truncate_chars(s, max_chars.saturating_sub(1));
    format!("{}…", truncated)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_truncate_bytes() {
        assert_eq!(truncate_bytes("hello", 3), "hel");
        assert_eq!(truncate_bytes("café", 3), "caf");
    }
    #[test]
    fn test_truncate_chars() {
        assert_eq!(truncate_chars("café", 3), "caf");
        assert_eq!(truncate_chars("hello", 10), "hello");
    }
    #[test]
    fn test_ellipsis() {
        assert_eq!(truncate_with_ellipsis("hello world", 8), "hello w…");
        assert_eq!(truncate_with_ellipsis("hi", 10), "hi");
    }
    #[test]
    fn test_emoji() {
        let s = "🌍🌎🌏";
        assert_eq!(truncate_chars(s, 2).chars().count(), 2);
    }
}
