//! # Ffi Opaque Types

pub fn placeholder() -> &'static str { "ffi-opaque-types implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
