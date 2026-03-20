#![allow(clippy::all)]
// Example 107: Lifetimes in Structs
//
// When a struct holds a reference, it needs a lifetime parameter
// to ensure the referenced data outlives the struct.

// Approach 1: Idiomatic Rust — struct borrowing a string slice
// The lifetime 'a ties the struct's validity to the borrowed data.
#[derive(Debug)]
pub struct Excerpt<'a> {
    pub text: &'a str,
    pub page: u32,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str, page: u32) -> Self {
        Excerpt { text, page }
    }

    pub fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.text
    }
}

// Approach 2: Struct with multiple borrowed fields
// All fields share the same lifetime 'a — the struct is valid for
// exactly as long as the shortest-lived of its borrowed data.
#[derive(Debug)]
pub struct Article<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub body: &'a str,
}

impl<'a> Article<'a> {
    pub fn new(title: &'a str, author: &'a str, body: &'a str) -> Self {
        Article {
            title,
            author,
            body,
        }
    }

    pub fn summarize(&self) -> String {
        format!(
            "{} by {} ({} chars)",
            self.title,
            self.author,
            self.body.len()
        )
    }
}

// Approach 3: Struct with a lifetime-bound method returning a reference
// The returned &str borrows from self, so it cannot outlive the struct.
#[derive(Debug)]
pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    /// Returns the remaining unparsed input — a sub-slice of the original.
    pub fn remaining(&self) -> &'a str {
        &self.input[self.pos..]
    }

    /// Advance past the next `n` bytes (ASCII only for simplicity).
    pub fn advance(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.input.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excerpt_borrows_slice() {
        let book = String::from("Call me Ishmael. Some years ago...");
        let exc = Excerpt::new(&book[..16], 1);
        assert_eq!(exc.text, "Call me Ishmael.");
        assert_eq!(exc.page, 1);
    }

    #[test]
    fn test_excerpt_announce_returns_text() {
        let sentence = "Fear is the mind-killer.";
        let exc = Excerpt::new(sentence, 42);
        let returned = exc.announce("test");
        assert_eq!(returned, sentence);
    }

    #[test]
    fn test_article_summarize() {
        let title = "Rust Ownership";
        let author = "Alice";
        let body = "Ownership is the key to Rust's safety guarantees.";
        let article = Article::new(title, author, body);
        let summary = article.summarize();
        assert!(summary.contains("Rust Ownership"));
        assert!(summary.contains("Alice"));
        assert!(summary.contains(&body.len().to_string()));
    }

    #[test]
    fn test_article_fields_accessible() {
        let article = Article {
            title: "Zero Cost",
            author: "Bob",
            body: "Abstractions without overhead.",
        };
        assert_eq!(article.title, "Zero Cost");
        assert_eq!(article.author, "Bob");
    }

    #[test]
    fn test_parser_remaining_advances() {
        let input = "fn main() {}";
        let mut parser = Parser::new(input);
        assert_eq!(parser.remaining(), "fn main() {}");
        parser.advance(3);
        assert_eq!(parser.remaining(), "main() {}");
    }

    #[test]
    fn test_parser_advance_clamps_to_end() {
        let input = "hello";
        let mut parser = Parser::new(input);
        parser.advance(100);
        assert_eq!(parser.remaining(), "");
    }

    #[test]
    fn test_excerpt_debug_format() {
        let text = "To be or not to be";
        let exc = Excerpt::new(text, 7);
        let debug = format!("{:?}", exc);
        assert!(debug.contains("To be or not to be"));
        assert!(debug.contains('7'));
    }
}
