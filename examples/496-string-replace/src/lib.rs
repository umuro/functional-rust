//! # String Replace — Substitution Patterns

pub fn replace_all(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

pub fn replace_first(s: &str, from: &str, to: &str) -> String {
    s.replacen(from, to, 1)
}

pub fn replace_last(s: &str, from: &str, to: &str) -> String {
    if let Some(pos) = s.rfind(from) {
        format!("{}{}{}", &s[..pos], to, &s[pos + from.len()..])
    } else {
        s.to_string()
    }
}

pub fn replace_chars<F: Fn(char) -> char>(s: &str, f: F) -> String {
    s.chars().map(f).collect()
}

pub fn remove_chars(s: &str, to_remove: &[char]) -> String {
    s.chars().filter(|c| !to_remove.contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_all() {
        assert_eq!(replace_all("aaa", "a", "b"), "bbb");
    }

    #[test]
    fn test_replace_first() {
        assert_eq!(replace_first("aaa", "a", "b"), "baa");
    }

    #[test]
    fn test_replace_last() {
        assert_eq!(replace_last("aaa", "a", "b"), "aab");
    }

    #[test]
    fn test_replace_chars() {
        assert_eq!(replace_chars("abc", |c| (c as u8 + 1) as char), "bcd");
    }

    #[test]
    fn test_remove_chars() {
        assert_eq!(remove_chars("hello world", &[' ', 'o']), "hellwrld");
    }
}
