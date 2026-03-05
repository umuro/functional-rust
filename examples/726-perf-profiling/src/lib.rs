//! # Perf Profiling

pub fn placeholder() -> &'static str { "perf-profiling implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
