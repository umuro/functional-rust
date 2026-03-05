//! # Phantom Capabilities

pub fn placeholder() -> &'static str { "phantom-capabilities implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
