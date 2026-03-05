//! # Memory Layout

pub fn placeholder() -> &'static str { "memory-layout implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
