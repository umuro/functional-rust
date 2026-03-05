//! # Exhaustive Result/Option Method Survey
//!
//! Complete reference of `Result<T,E>` and `Option<T>` methods.

/// Demonstrate Result methods
pub fn result_methods() {
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("bad");

    // Query methods
    let _ = ok.is_ok();
    let _ = ok.is_err();
    let _ = ok.ok();
    let _ = err.err();

    // Transform methods
    let _ = ok.map(|x| x * 2);
    let _ = err.map_err(|e| format!("error: {}", e));
    let _ = ok.map_or(0, |x| x + 1);
    let _ = ok.map_or_else(|_| 0, |x| x);

    // Combinators
    let _ = ok.and(Ok::<i32, &str>(10));
    let _ = err.or(Ok(42));
    let _ = ok.and_then(|x| Ok::<i32, &str>(x * 2));
    let _ = err.or_else(|_| Ok(99));

    // Unwrap variants
    let _ = ok.unwrap_or(0);
    let _ = ok.unwrap_or_else(|_| 0);
    let _ = ok.unwrap_or_default();
}

/// Demonstrate Option methods
pub fn option_methods() {
    let some: Option<i32> = Some(5);
    let none: Option<i32> = None;

    // Query methods
    let _ = some.is_some();
    let _ = none.is_none();

    // Transform methods
    let _ = some.map(|x| x * 2);
    let _ = some.filter(|&x| x > 3);
    let _ = some.map_or(0, |x| x + 1);

    // Combinators
    let _ = some.and(Some(10));
    let _ = none.or(Some(42));
    let _ = some.and_then(|x| Some(x * 2));
    let _ = none.or_else(|| Some(99));

    // Unwrap variants
    let _ = none.unwrap_or(0);
    let _ = none.unwrap_or_else(|| 99);
    let _ = none.unwrap_or_default();

    // Conversion
    let _ = none.ok_or("missing");
    let _ = Some(Some(42)).flatten();
    let _ = some.zip(Some("hello"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_result_map_chain() {
        let r: Result<i32, &str> = Ok(5);
        assert_eq!(r.map(|x| x * 2).map(|x| x + 1), Ok(11));
    }

    #[test]
    fn test_option_and_then_chain() {
        let r = Some(5i32)
            .and_then(|x| if x > 0 { Some(x * 2) } else { None })
            .filter(|&x| x < 20);
        assert_eq!(r, Some(10));
    }

    #[test]
    fn test_result_or_else() {
        let r: Result<i32, &str> = Err("bad");
        assert_eq!(r.or_else(|_| Ok(42)), Ok(42));
    }

    #[test]
    fn test_option_zip() {
        assert_eq!(Some(1).zip(Some("a")), Some((1, "a")));
        assert_eq!(Some(1).zip(None::<&str>), None);
    }

    #[test]
    fn test_option_flatten() {
        assert_eq!(Some(Some(42)).flatten(), Some(42));
        assert_eq!(Some(None::<i32>).flatten(), None);
    }
}
