//! # Perf Lazy Eval

pub fn placeholder() -> &'static str { "perf-lazy-eval implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
