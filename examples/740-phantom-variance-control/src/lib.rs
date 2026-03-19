//! # Phantom Variance Control

pub fn placeholder() -> &'static str {
    "phantom-variance-control implementation"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() {
        assert!(!placeholder().is_empty());
    }
}
