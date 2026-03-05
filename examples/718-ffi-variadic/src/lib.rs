//! # Ffi Variadic

pub fn placeholder() -> &'static str { "ffi-variadic implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
