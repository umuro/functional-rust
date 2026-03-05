//! # Perf Cache Friendly

pub fn placeholder() -> &'static str { "perf-cache-friendly implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
