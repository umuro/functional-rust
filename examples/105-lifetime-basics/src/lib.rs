#![allow(clippy::all)]
// 105: Lifetime Basics
// Lifetime annotations tell the compiler how long references live

// Approach 1: Lifetime in function signature
// 'a means: the returned reference lives as long as the inputs
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn first_element<'a>(v: &'a [i32]) -> Option<&'a i32> {
    v.first()
}

// Approach 2: Lifetime in struct
struct Important<'a> {
    content: &'a str,
}

impl<'a> Important<'a> {
    fn new(content: &'a str) -> Self {
        Important { content }
    }

    fn content(&self) -> &str {
        self.content
    }
}

// Approach 3: Multiple lifetimes
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// This would NOT compile — dangling reference:
// fn dangling() -> &str {
//     let s = String::from("hello");
//     &s // ERROR: s dropped here, reference would dangle
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("hello", "hi"), "hello");
        assert_eq!(longest("a", "bb"), "bb");
    }

    #[test]
    fn test_first_element() {
        assert_eq!(first_element(&[1, 2, 3]), Some(&1));
        assert_eq!(first_element(&[]), None);
    }

    #[test]
    fn test_important() {
        let msg = Important::new("test");
        assert_eq!(msg.content(), "test");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }
}
