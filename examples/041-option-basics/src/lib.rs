#![allow(clippy::all)]
// Option<T> basics: construction, matching, and the common accessor methods.
pub fn safe_min(v: &[i32]) -> Option<i32> {
    v.iter().min().copied()
}

pub fn describe(opt: Option<i32>) -> String {
    match opt {
        Some(x) => format!("Some({})", x),
        None => "None".to_string(),
    }
}

pub fn value_or_default(opt: Option<i32>, default: i32) -> i32 {
    opt.unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_min_nonempty() {
        assert_eq!(safe_min(&[3, 1, 4, 1, 5]), Some(1));
    }

    #[test]
    fn test_safe_min_empty() {
        assert_eq!(safe_min(&[]), None);
    }

    #[test]
    fn test_describe() {
        assert_eq!(describe(Some(42)), "Some(42)");
        assert_eq!(describe(None), "None");
    }

    #[test]
    fn test_value_or_default() {
        assert_eq!(value_or_default(Some(5), 0), 5);
        assert_eq!(value_or_default(None, 0), 0);
    }
}
