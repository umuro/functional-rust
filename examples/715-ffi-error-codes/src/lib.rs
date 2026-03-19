#![allow(clippy::all)]
//! # Ffi Error Codes

pub fn placeholder() -> &'static str {
    "ffi-error-codes implementation"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() {
        assert!(!placeholder().is_empty());
    }
}
