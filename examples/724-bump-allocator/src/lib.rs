//! # Bump Allocator

pub fn placeholder() -> &'static str { "bump-allocator implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
