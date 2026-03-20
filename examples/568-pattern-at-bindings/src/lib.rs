#![allow(clippy::all)]
//! @ Bindings
//!
//! Binding a name while also matching a pattern.

/// Bind while matching range.
pub fn describe_age(age: u32) -> String {
    match age {
        a @ 0..=12 => format!("child ({})", a),
        a @ 13..=19 => format!("teen ({})", a),
        a @ 20..=64 => format!("adult ({})", a),
        a => format!("senior ({})", a),
    }
}

/// Bind while matching enum.
#[derive(Debug)]
pub enum Event {
    Click(i32, i32),
    KeyPress(char),
}

pub fn process_event(e: &Event) -> String {
    match e {
        e @ Event::Click(_, _) => format!("click: {:?}", e),
        e @ Event::KeyPress(_) => format!("key: {:?}", e),
    }
}

/// Bind while destructuring.
pub fn first_two(v: &[i32]) -> Option<(i32, i32)> {
    match v {
        [first @ .., second, _] if v.len() >= 2 => Some((*first.first()?, *second)),
        _ => None,
    }
}

/// Bind with guards.
pub fn check_value(n: i32) -> &'static str {
    match n {
        x @ 0..=10 if x % 2 == 0 => "small even",
        x @ 0..=10 => "small odd",
        x if x > 100 => "large",
        _ => "medium",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_age() {
        assert!(describe_age(5).contains("child"));
        assert!(describe_age(15).contains("teen"));
        assert!(describe_age(30).contains("adult"));
    }

    #[test]
    fn test_process_event() {
        let e = Event::Click(10, 20);
        assert!(process_event(&e).contains("click"));
    }

    #[test]
    fn test_check_value() {
        assert_eq!(check_value(4), "small even");
        assert_eq!(check_value(5), "small odd");
        assert_eq!(check_value(200), "large");
    }
}
