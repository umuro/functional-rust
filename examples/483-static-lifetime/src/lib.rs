//! # Static Lifetime — 'static Bounds
//!
//! Understanding the 'static lifetime bound.

// String literals are 'static
pub fn static_str() -> &'static str {
    "I live forever"
}

// Owned types satisfy 'static
pub fn spawn_with_data<T: Send + 'static>(data: T) {
    std::thread::spawn(move || {
        let _ = data;
    });
}

// 'static doesn't mean immortal - it means CAN live forever
pub struct Owned {
    data: String,
}

impl Owned {
    pub fn new(s: &str) -> Self {
        Self { data: s.to_string() }
    }
}

// Owned implements 'static because it doesn't borrow
fn assert_static<T: 'static>() {}

pub fn check_static_bounds() {
    assert_static::<String>();
    assert_static::<i32>();
    assert_static::<Owned>();
    // assert_static::<&str>(); // Only works for &'static str
}

// Static promotion
pub fn promoted() -> &'static [i32; 3] {
    &[1, 2, 3] // Promoted to static storage
}

// Leaking to create 'static references
pub fn leak_to_static(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_str() {
        let s = static_str();
        assert_eq!(s, "I live forever");
    }

    #[test]
    fn test_promoted() {
        let arr = promoted();
        assert_eq!(arr, &[1, 2, 3]);
    }

    #[test]
    fn test_leak() {
        let s = String::from("leaked");
        let leaked: &'static str = leak_to_static(s);
        assert_eq!(leaked, "leaked");
    }

    #[test]
    fn test_spawn() {
        spawn_with_data(String::from("owned"));
        spawn_with_data(42);
    }
}
