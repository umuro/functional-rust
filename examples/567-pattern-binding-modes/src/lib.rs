#![allow(clippy::all)]
//! Pattern Binding Modes
//!
//! ref, ref mut, and move in patterns.

/// Move binding (default for owned).
pub fn move_binding(opt: Option<String>) -> usize {
    match opt {
        Some(s) => s.len(), // s is moved
        None => 0,
    }
}

/// Ref binding (borrow).
pub fn ref_binding(opt: &Option<String>) -> usize {
    match opt {
        Some(ref s) => s.len(), // s is &String
        None => 0,
    }
}

/// Modern: match ergonomics.
pub fn ergonomic_binding(opt: &Option<String>) -> usize {
    match opt {
        Some(s) => s.len(), // s is automatically &String
        None => 0,
    }
}

/// Ref mut binding.
pub fn ref_mut_binding(opt: &mut Option<String>) {
    if let Some(ref mut s) = opt {
        s.push_str("!");
    }
}

/// Binding with @.
pub fn at_binding(n: i32) -> String {
    match n {
        x @ 1..=5 => format!("small: {}", x),
        x @ 6..=10 => format!("medium: {}", x),
        x => format!("other: {}", x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        assert_eq!(move_binding(Some("hello".into())), 5);
    }

    #[test]
    fn test_ref() {
        let opt = Some("world".to_string());
        assert_eq!(ref_binding(&opt), 5);
        assert!(opt.is_some()); // still valid
    }

    #[test]
    fn test_ergonomic() {
        let opt = Some("test".to_string());
        assert_eq!(ergonomic_binding(&opt), 4);
    }

    #[test]
    fn test_ref_mut() {
        let mut opt = Some("hi".to_string());
        ref_mut_binding(&mut opt);
        assert_eq!(opt, Some("hi!".to_string()));
    }

    #[test]
    fn test_at() {
        assert!(at_binding(3).contains("small"));
        assert!(at_binding(7).contains("medium"));
    }
}
