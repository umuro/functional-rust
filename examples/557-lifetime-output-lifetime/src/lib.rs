//! Output Lifetime Patterns
//!
//! How output lifetimes relate to inputs.

/// Output tied to single input.
pub fn first_char(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[..c.len_utf8()])
}

/// Output tied to shortest input.
pub fn common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str {
    let len = a.chars().zip(b.chars()).take_while(|(x, y)| x == y).count();
    &a[..a.chars().take(len).map(|c| c.len_utf8()).sum()]
}

/// Output lifetime independent of input.
pub fn static_str(_s: &str) -> &'static str {
    "static"
}

/// Struct returning references to its data.
pub struct Container {
    items: Vec<String>,
}

impl Container {
    pub fn new() -> Self {
        Container { items: Vec::new() }
    }

    pub fn add(&mut self, s: &str) {
        self.items.push(s.to_string());
    }

    pub fn get(&self, idx: usize) -> Option<&str> {
        self.items.get(idx).map(|s| s.as_str())
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_char() {
        assert_eq!(first_char("hello"), Some("h"));
        assert_eq!(first_char(""), None);
    }

    #[test]
    fn test_common_prefix() {
        assert_eq!(common_prefix("hello", "help"), "hel");
        assert_eq!(common_prefix("abc", "xyz"), "");
    }

    #[test]
    fn test_static_str() {
        let s = String::from("temporary");
        let result = static_str(&s);
        assert_eq!(result, "static");
    }

    #[test]
    fn test_container() {
        let mut c = Container::new();
        c.add("hello");
        c.add("world");
        assert_eq!(c.get(0), Some("hello"));
        assert_eq!(c.get(1), Some("world"));
    }
}
