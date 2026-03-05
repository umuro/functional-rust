//! # Scott Encoding
//!
//! Alternative to Church encoding using pattern matching style.

/// Scott-encoded natural numbers.
pub enum ScottNat<T> {
    Zero(Box<dyn Fn() -> T>),
    Succ(Box<dyn Fn(ScottNat<T>) -> T>),
}

/// Scott-encoded boolean.
pub fn scott_true<T>(on_true: T, _on_false: T) -> T {
    on_true
}

pub fn scott_false<T>(_on_true: T, on_false: T) -> T {
    on_false
}

/// Scott-encoded Option.
pub enum ScottOption<A, T> {
    None(Box<dyn Fn() -> T>),
    Some(Box<dyn Fn(A) -> T>),
}

/// Simple Scott-like pattern matching simulation.
pub fn scott_match_bool<T>(b: bool, on_true: impl Fn() -> T, on_false: impl Fn() -> T) -> T {
    if b { on_true() } else { on_false() }
}

/// Scott-like Option matching.
pub fn scott_match_option<A, T>(
    opt: Option<A>,
    on_none: impl Fn() -> T,
    on_some: impl Fn(A) -> T,
) -> T {
    match opt {
        None => on_none(),
        Some(a) => on_some(a),
    }
}

/// Scott-like Result matching.
pub fn scott_match_result<A, E, T>(
    res: Result<A, E>,
    on_ok: impl Fn(A) -> T,
    on_err: impl Fn(E) -> T,
) -> T {
    match res {
        Ok(a) => on_ok(a),
        Err(e) => on_err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scott_bool() {
        assert_eq!(scott_true(1, 2), 1);
        assert_eq!(scott_false(1, 2), 2);
    }

    #[test]
    fn test_scott_match_bool() {
        assert_eq!(scott_match_bool(true, || "yes", || "no"), "yes");
        assert_eq!(scott_match_bool(false, || "yes", || "no"), "no");
    }

    #[test]
    fn test_scott_match_option() {
        assert_eq!(scott_match_option(Some(42), || 0, |x| x * 2), 84);
        assert_eq!(scott_match_option(None::<i32>, || 0, |x| x * 2), 0);
    }

    #[test]
    fn test_scott_match_result() {
        let ok: Result<i32, &str> = Ok(10);
        let err: Result<i32, &str> = Err("oops");
        assert_eq!(scott_match_result(ok, |x| x, |_| -1), 10);
        assert_eq!(scott_match_result(err, |x| x, |_| -1), -1);
    }
}
