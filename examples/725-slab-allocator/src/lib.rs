//! # Slab Allocator

pub fn placeholder() -> &'static str { "slab-allocator implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
