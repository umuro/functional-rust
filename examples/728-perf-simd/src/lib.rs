//! # Perf Simd

pub fn placeholder() -> &'static str { "perf-simd implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
