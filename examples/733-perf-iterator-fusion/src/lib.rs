//! # Perf Iterator Fusion

pub fn placeholder() -> &'static str { "perf-iterator-fusion implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
