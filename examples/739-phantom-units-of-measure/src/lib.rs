#![allow(clippy::all)]
//! # Phantom Units Of Measure

pub fn placeholder() -> &'static str {
    "phantom-units-of-measure implementation"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() {
        assert!(!placeholder().is_empty());
    }
}
