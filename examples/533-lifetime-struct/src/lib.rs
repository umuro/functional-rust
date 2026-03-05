//! Lifetimes in Structs
//!
//! Struct fields that are references require lifetime annotations.

/// A highlight into a larger text — borrows from the source string.
#[derive(Debug, Clone)]
pub struct Highlight<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> Highlight<'a> {
    pub fn new(source: &'a str, start: usize, end: usize) -> Option<Self> {
        if end <= source.len() && start <= end {
            Some(Highlight {
                text: &source[start..end],
                start,
                end,
            })
        } else {
            None
        }
    }

    pub fn text(&self) -> &str {
        self.text
    }
}

/// Iterator that borrows from source.
pub struct Words<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Words<'a> {
    pub fn new(source: &'a str) -> Self {
        Words { source, position: 0 }
    }
}

impl<'a> Iterator for Words<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = &self.source[self.position..];
        let trimmed = remaining.trim_start();
        if trimmed.is_empty() {
            return None;
        }
        self.position = self.source.len() - trimmed.len();

        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        self.position += end;
        Some(&trimmed[..end])
    }
}

/// Config that borrows from environment.
#[derive(Debug)]
pub struct Config<'a> {
    pub name: &'a str,
    pub values: Vec<&'a str>,
}

impl<'a> Config<'a> {
    pub fn new(name: &'a str) -> Self {
        Config {
            name,
            values: Vec::new(),
        }
    }

    pub fn add_value(&mut self, value: &'a str) {
        self.values.push(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_creation() {
        let text = "Hello, World!";
        let highlight = Highlight::new(text, 0, 5).unwrap();
        assert_eq!(highlight.text(), "Hello");
        assert_eq!(highlight.start, 0);
        assert_eq!(highlight.end, 5);
    }

    #[test]
    fn test_highlight_invalid() {
        let text = "short";
        assert!(Highlight::new(text, 0, 100).is_none());
        assert!(Highlight::new(text, 5, 3).is_none());
    }

    #[test]
    fn test_words_iterator() {
        let text = "hello world rust";
        let words: Vec<&str> = Words::new(text).collect();
        assert_eq!(words, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_words_empty() {
        let words: Vec<&str> = Words::new("").collect();
        assert!(words.is_empty());
    }

    #[test]
    fn test_config() {
        let name = "my_config";
        let mut config = Config::new(name);
        config.add_value("value1");
        config.add_value("value2");

        assert_eq!(config.name, "my_config");
        assert_eq!(config.values.len(), 2);
    }

    #[test]
    fn test_struct_lifetime_scope() {
        let highlight;
        {
            let text = String::from("temporary text");
            highlight = Highlight::new(&text, 0, 9);
            assert_eq!(highlight.as_ref().unwrap().text(), "temporary");
        }
        // highlight is now invalid because text is dropped
    }
}
