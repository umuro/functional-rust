//! Enum Variant Patterns
//!
//! Matching and destructuring enum variants.

#[derive(Debug, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// Match all variants.
pub fn process_message(msg: &Message) -> String {
    match msg {
        Message::Quit => "Quit".to_string(),
        Message::Move { x, y } => format!("Move to ({}, {})", x, y),
        Message::Write(text) => format!("Write: {}", text),
        Message::ChangeColor(r, g, b) => format!("Color: rgb({}, {}, {})", r, g, b),
    }
}

/// if let for single variant.
pub fn is_quit(msg: &Message) -> bool {
    matches!(msg, Message::Quit)
}

/// Extract from specific variant.
pub fn get_write_text(msg: &Message) -> Option<&str> {
    if let Message::Write(text) = msg {
        Some(text)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!(process_message(&Message::Quit), "Quit");
        assert!(process_message(&Message::Move { x: 1, y: 2 }).contains("1"));
        assert!(process_message(&Message::Write("hi".into())).contains("hi"));
    }

    #[test]
    fn test_is_quit() {
        assert!(is_quit(&Message::Quit));
        assert!(!is_quit(&Message::Write("x".into())));
    }

    #[test]
    fn test_get_write() {
        assert_eq!(get_write_text(&Message::Write("hello".into())), Some("hello"));
        assert_eq!(get_write_text(&Message::Quit), None);
    }
}
