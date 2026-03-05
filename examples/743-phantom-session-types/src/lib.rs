//! # Phantom Session Types

pub fn placeholder() -> &'static str { "phantom-session-types implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
