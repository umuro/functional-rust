//! # Ffi Threading

pub fn placeholder() -> &'static str { "ffi-threading implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
