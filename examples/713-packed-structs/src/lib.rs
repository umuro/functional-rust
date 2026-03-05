//! # Packed Structs

pub fn placeholder() -> &'static str { "packed-structs implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
