#![allow(clippy::all)]
//! Multiple Lifetime Parameters
//!
//! Independent lifetimes for inputs with different validity scopes.

/// Output tied to x only — y can have shorter lifetime.
pub fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

/// Struct with two independent borrowed fields.
#[derive(Debug)]
pub struct Pair<'a, 'b> {
    pub first: &'a str,
    pub second: &'b str,
}

impl<'a, 'b> Pair<'a, 'b> {
    pub fn new(first: &'a str, second: &'b str) -> Self {
        Pair { first, second }
    }

    /// Returns from first — tied to 'a.
    pub fn get_first(&self) -> &'a str {
        self.first
    }

    /// Returns from second — tied to 'b.
    pub fn get_second(&self) -> &'b str {
        self.second
    }
}

/// Context with reader and writer — independent lifetimes.
pub struct Context<'r, 'w> {
    reader: &'r str,
    writer: &'w mut String,
}

impl<'r, 'w> Context<'r, 'w> {
    pub fn new(reader: &'r str, writer: &'w mut String) -> Self {
        Context { reader, writer }
    }

    pub fn read(&self) -> &'r str {
        self.reader
    }

    pub fn write(&mut self, s: &str) {
        self.writer.push_str(s);
    }
}

/// Three different lifetimes.
pub fn select<'a, 'b, 'c>(a: &'a str, _b: &'b str, _c: &'c str, choice: usize) -> &'a str {
    // Can only return 'a since that's what we promised
    match choice {
        _ => a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_of() {
        let x = "first";
        {
            let y = String::from("second");
            let result = first_of(x, &y);
            assert_eq!(result, "first");
        }
        // x is still valid, y is dropped
    }

    #[test]
    fn test_pair_independent() {
        let first = String::from("hello");
        let second = String::from("world");
        let pair = Pair::new(&first, &second);
        assert_eq!(pair.get_first(), "hello");
        assert_eq!(pair.get_second(), "world");
        // Both references tied to their respective lifetimes
    }

    #[test]
    fn test_pair_same_lifetime() {
        let s1 = "hello";
        let s2 = "world";
        let pair = Pair::new(s1, s2);
        assert_eq!(pair.get_first(), "hello");
        assert_eq!(pair.get_second(), "world");
    }

    #[test]
    fn test_context() {
        let input = "read this";
        let mut output = String::new();
        let mut ctx = Context::new(input, &mut output);

        assert_eq!(ctx.read(), "read this");
        ctx.write("wrote this");
        assert_eq!(output, "wrote this");
    }

    #[test]
    fn test_select() {
        let a = "a";
        let b = "b";
        let c = "c";
        assert_eq!(select(a, b, c, 0), "a");
    }
}
