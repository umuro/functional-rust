//! # Perf String Ops

pub fn placeholder() -> &'static str { "perf-string-ops implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
