//! Pattern Guards
//!
//! Additional conditions with if in match arms.

/// Guard with condition.
pub fn categorize(n: i32) -> &'static str {
    match n {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x < 10 => "small positive",
        _ => "large positive",
    }
}

/// Guard with multiple conditions.
pub fn check_range(n: i32, min: i32, max: i32) -> bool {
    match n {
        x if x >= min && x <= max => true,
        _ => false,
    }
}

/// Guard with destructuring.
pub fn process_point(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origin",
        (x, y) if x == y => "diagonal",
        (x, _) if x > 0 => "positive x",
        (_, y) if y > 0 => "positive y",
        _ => "other",
    }
}

/// Guard with Option.
pub fn check_option(opt: Option<i32>) -> &'static str {
    match opt {
        Some(x) if x > 100 => "large",
        Some(x) if x > 0 => "positive",
        Some(0) => "zero",
        Some(_) => "negative",
        None => "none",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize() {
        assert_eq!(categorize(-5), "negative");
        assert_eq!(categorize(0), "zero");
        assert_eq!(categorize(5), "small positive");
        assert_eq!(categorize(100), "large positive");
    }

    #[test]
    fn test_range() {
        assert!(check_range(5, 1, 10));
        assert!(!check_range(15, 1, 10));
    }

    #[test]
    fn test_point() {
        assert_eq!(process_point((0, 0)), "origin");
        assert_eq!(process_point((5, 5)), "diagonal");
    }

    #[test]
    fn test_option() {
        assert_eq!(check_option(Some(200)), "large");
        assert_eq!(check_option(None), "none");
    }
}
