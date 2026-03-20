#![allow(clippy::all)]
//! Ref Patterns
//!
//! Borrowing in patterns with ref and ref mut.

/// Using ref to borrow.
pub fn inspect(opt: &Option<String>) -> usize {
    match opt {
        Some(ref s) => s.len(),
        None => 0,
    }
}

/// Modern ergonomics (automatic ref).
pub fn inspect_modern(opt: &Option<String>) -> usize {
    match opt {
        Some(s) => s.len(), // s is automatically &String
        None => 0,
    }
}

/// Using ref mut.
pub fn append_exclaim(opt: &mut Option<String>) {
    match opt {
        Some(ref mut s) => s.push('!'),
        None => {}
    }
}

/// Ref in struct destructuring.
pub struct Container {
    pub data: String,
}

pub fn peek(c: &Container) -> &str {
    let Container { ref data } = c;
    data
}

/// Ref vs move.
pub fn demonstrate_ref() {
    let opt = Some(String::from("hello"));

    // With ref: borrow, opt still valid
    if let Some(ref s) = opt {
        println!("Borrowed: {}", s);
    }
    // opt still accessible
    assert!(opt.is_some());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect() {
        let opt = Some("hello".to_string());
        assert_eq!(inspect(&opt), 5);
        assert!(opt.is_some()); // still valid
    }

    #[test]
    fn test_inspect_modern() {
        let opt = Some("world".to_string());
        assert_eq!(inspect_modern(&opt), 5);
    }

    #[test]
    fn test_append() {
        let mut opt = Some("hi".to_string());
        append_exclaim(&mut opt);
        assert_eq!(opt, Some("hi!".to_string()));
    }

    #[test]
    fn test_peek() {
        let c = Container {
            data: "test".into(),
        };
        assert_eq!(peek(&c), "test");
    }
}
