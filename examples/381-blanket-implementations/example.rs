// Blanket implementations in Rust
use std::fmt;

trait Summary {
    fn summarize(&self) -> String;
}

// Blanket impl: anything that is Display also gets Summary
impl<T: fmt::Display> Summary for T {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)
    }
}

// Another example: blanket impl for conversion
trait DoubleString {
    fn double_string(&self) -> String;
}

impl<T: fmt::Display> DoubleString for T {
    fn double_string(&self) -> String {
        let s = self.to_string();
        format!("{}{}", s, s)
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    // i32 gets Summary via blanket impl (Display is implemented for i32)
    print_summary(&42);
    print_summary(&3.14f64);
    print_summary(&"hello world");

    println!("{}", 7u32.double_string());
    println!("{}", "abc".double_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blanket_summary() {
        assert_eq!(42i32.summarize(), "Summary: 42");
        assert_eq!("hi".summarize(), "Summary: hi");
    }

    #[test]
    fn test_blanket_double() {
        assert_eq!(5u8.double_string(), "55");
        assert_eq!("ab".double_string(), "abab");
    }
}
