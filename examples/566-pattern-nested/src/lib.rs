//! Nested Patterns
//!
//! Matching deeply nested structures.

#[derive(Debug)]
pub struct Outer {
    pub inner: Inner,
}
#[derive(Debug)]
pub struct Inner {
    pub value: i32,
}

/// Match nested struct.
pub fn get_value(o: &Outer) -> i32 {
    match o {
        Outer {
            inner: Inner { value },
        } => *value,
    }
}

/// Nested Option.
pub fn unwrap_nested(opt: Option<Option<i32>>) -> i32 {
    match opt {
        Some(Some(v)) => v,
        Some(None) => -1,
        None => -2,
    }
}

/// Nested Result.
pub fn process_nested(res: Result<Result<i32, &str>, &str>) -> i32 {
    match res {
        Ok(Ok(v)) => v,
        Ok(Err(_)) => -1,
        Err(_) => -2,
    }
}

/// Deeply nested.
pub fn deep_match(data: Option<(i32, Option<(i32, i32)>)>) -> i32 {
    match data {
        Some((a, Some((b, c)))) => a + b + c,
        Some((a, None)) => a,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value() {
        let o = Outer {
            inner: Inner { value: 42 },
        };
        assert_eq!(get_value(&o), 42);
    }

    #[test]
    fn test_unwrap_nested() {
        assert_eq!(unwrap_nested(Some(Some(5))), 5);
        assert_eq!(unwrap_nested(Some(None)), -1);
        assert_eq!(unwrap_nested(None), -2);
    }

    #[test]
    fn test_process_nested() {
        assert_eq!(process_nested(Ok(Ok(10))), 10);
        assert_eq!(process_nested(Ok(Err("e"))), -1);
        assert_eq!(process_nested(Err("e")), -2);
    }

    #[test]
    fn test_deep_match() {
        assert_eq!(deep_match(Some((1, Some((2, 3))))), 6);
        assert_eq!(deep_match(Some((5, None))), 5);
        assert_eq!(deep_match(None), 0);
    }
}
