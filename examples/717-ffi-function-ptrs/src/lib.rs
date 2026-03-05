//! # Ffi Function Ptrs

pub fn placeholder() -> &'static str { "ffi-function-ptrs implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
