//! # String Split — Tokenization

pub fn split_whitespace(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn split_char(s: &str, c: char) -> Vec<&str> {
    s.split(c).collect()
}

pub fn split_str(s: &str, delim: &str) -> Vec<&str> {
    s.split(delim).collect()
}

pub fn split_lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

pub fn split_n(s: &str, c: char, n: usize) -> Vec<&str> {
    s.splitn(n, c).collect()
}

pub fn split_once_str(s: &str, delim: &str) -> Option<(&str, &str)> {
    s.split_once(delim)
}

pub fn split_by<F: Fn(char) -> bool>(s: &str, f: F) -> Vec<&str> {
    s.split(f).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_whitespace() {
        assert_eq!(split_whitespace("  a  b  c  "), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_char() {
        assert_eq!(split_char("a,b,c", ','), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_lines() {
        assert_eq!(split_lines("a\nb\nc"), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_n() {
        assert_eq!(split_n("a:b:c:d", ':', 2), vec!["a", "b:c:d"]);
    }

    #[test]
    fn test_split_once() {
        assert_eq!(split_once_str("key=value", "="), Some(("key", "value")));
    }
}
