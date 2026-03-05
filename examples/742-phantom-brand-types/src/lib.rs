//! # Phantom Brand Types

pub fn placeholder() -> &'static str { "phantom-brand-types implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
