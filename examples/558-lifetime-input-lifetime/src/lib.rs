#![allow(clippy::all)]
//! Input Lifetime Patterns
//!
//! How input lifetimes constrain function signatures.

/// Single input lifetime propagates to output.
pub fn echo<'a>(s: &'a str) -> &'a str {
    s
}

/// Multiple inputs with independent lifetimes.
pub fn first<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

/// Input lifetime bounds struct.
pub struct Processor<'input> {
    data: &'input str,
}

impl<'input> Processor<'input> {
    pub fn new(data: &'input str) -> Self {
        Processor { data }
    }

    pub fn process(&self) -> &'input str {
        self.data.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        assert_eq!(echo("hello"), "hello");
    }

    #[test]
    fn test_first() {
        assert_eq!(first("a", "b"), "a");
    }

    #[test]
    fn test_processor() {
        let p = Processor::new("  test  ");
        assert_eq!(p.process(), "test");
    }
}
