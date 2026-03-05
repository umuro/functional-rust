//! Self-Referential Structs
//!
//! Patterns for structs that reference their own data.

use std::pin::Pin;

/// Safe approach: store index instead of reference.
pub struct Buffer {
    data: String,
    start: usize,
    end: usize,
}

impl Buffer {
    pub fn new(data: &str, start: usize, end: usize) -> Self {
        Buffer {
            data: data.to_string(),
            start,
            end,
        }
    }

    pub fn view(&self) -> &str {
        &self.data[self.start..self.end]
    }
}

/// Using separate owner and view.
pub struct Owner {
    data: String,
}

impl Owner {
    pub fn new(data: &str) -> Self {
        Owner { data: data.to_string() }
    }

    pub fn get(&self) -> &str {
        &self.data
    }
}

/// Pinned self-referential (advanced).
pub struct Pinned {
    data: String,
    // In real code, this would be a pointer set after pinning
}

impl Pinned {
    pub fn new(data: String) -> Pin<Box<Self>> {
        Box::pin(Pinned { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_view() {
        let buf = Buffer::new("hello world", 0, 5);
        assert_eq!(buf.view(), "hello");
    }

    #[test]
    fn test_owner() {
        let owner = Owner::new("test");
        assert_eq!(owner.get(), "test");
    }

    #[test]
    fn test_pinned() {
        let pinned = Pinned::new("data".into());
        // pinned is now immovable
        assert!(pinned.data.len() > 0);
    }
}
