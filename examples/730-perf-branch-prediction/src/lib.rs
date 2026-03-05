//! # Perf Branch Prediction

pub fn placeholder() -> &'static str { "perf-branch-prediction implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
