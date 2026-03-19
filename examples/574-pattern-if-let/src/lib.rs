#![allow(clippy::all)]
//! if-let and while-let
//!
//! Conditional pattern matching.

/// Basic if-let.
pub fn describe_option(opt: Option<i32>) -> String {
    if let Some(n) = opt {
        format!("Got: {}", n)
    } else {
        "Nothing".to_string()
    }
}

/// if-let with else-if-let.
pub fn categorize(opt: Option<i32>) -> &'static str {
    if let Some(n) = opt {
        if n > 0 {
            "positive"
        } else if n < 0 {
            "negative"
        } else {
            "zero"
        }
    } else {
        "none"
    }
}

/// if-let with enum.
#[derive(Debug)]
pub enum Message {
    Text(String),
    Number(i32),
    Empty,
}

pub fn extract_text(msg: &Message) -> Option<&str> {
    if let Message::Text(s) = msg {
        Some(s)
    } else {
        None
    }
}

/// while-let for iteration.
pub fn sum_stack(mut stack: Vec<i32>) -> i32 {
    let mut sum = 0;
    while let Some(n) = stack.pop() {
        sum += n;
    }
    sum
}

/// Combining if-let with guards.
pub fn check_value(opt: Option<i32>) -> &'static str {
    if let Some(n) = opt {
        if n > 100 {
            "large"
        } else {
            "small"
        }
    } else {
        "none"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        assert!(describe_option(Some(5)).contains("Got"));
        assert!(describe_option(None).contains("Nothing"));
    }

    #[test]
    fn test_categorize() {
        assert_eq!(categorize(Some(5)), "positive");
        assert_eq!(categorize(Some(-3)), "negative");
        assert_eq!(categorize(None), "none");
    }

    #[test]
    fn test_extract_text() {
        let msg = Message::Text("hello".into());
        assert_eq!(extract_text(&msg), Some("hello"));
        assert_eq!(extract_text(&Message::Empty), None);
    }

    #[test]
    fn test_sum_stack() {
        assert_eq!(sum_stack(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_stack(vec![]), 0);
    }

    #[test]
    fn test_check_value() {
        assert_eq!(check_value(Some(200)), "large");
        assert_eq!(check_value(Some(50)), "small");
    }
}
