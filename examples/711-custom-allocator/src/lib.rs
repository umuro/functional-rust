//! # Custom Allocator

pub fn placeholder() -> &'static str { "custom-allocator implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
