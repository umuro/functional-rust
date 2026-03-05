//! Named Return Lifetimes
//!
//! Explicit lifetime names for return references.

/// Explicitly named output lifetime.
pub fn first<'out>(items: &'out [i32]) -> Option<&'out i32> {
    items.first()
}

/// Two inputs, output tied to first.
pub fn prefer_first<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

/// Struct with named output lifetime.
pub struct Parser<'input> {
    data: &'input str,
}

impl<'input> Parser<'input> {
    pub fn new(data: &'input str) -> Self {
        Parser { data }
    }

    pub fn parse(&self) -> &'input str {
        self.data.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let items = [1, 2, 3];
        assert_eq!(first(&items), Some(&1));
    }

    #[test]
    fn test_prefer_first() {
        assert_eq!(prefer_first("a", "b"), "a");
    }

    #[test]
    fn test_parser() {
        let input = "  hello  ";
        let parser = Parser::new(input);
        assert_eq!(parser.parse(), "hello");
    }
}
