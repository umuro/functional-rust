//! # Arena Allocator

pub fn placeholder() -> &'static str { "arena-allocator implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
