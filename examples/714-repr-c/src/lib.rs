//! # Repr C

pub fn placeholder() -> &'static str { "repr-c implementation" }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_placeholder() { assert!(!placeholder().is_empty()); }
}
