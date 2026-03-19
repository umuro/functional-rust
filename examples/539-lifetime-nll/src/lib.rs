//! Non-Lexical Lifetimes (NLL)
//!
//! Modern borrow checker: borrows end at last use, not end of block.

/// NLL allows mutation after last borrow use.
pub fn nll_basic() -> Vec<i32> {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0]; // borrow ends after this line
    v.push(6); // OK with NLL
    assert_eq!(first, 1);
    v
}

/// NLL enables conditional borrows.
pub fn nll_conditional(data: &mut Vec<i32>, add: bool) {
    let first = data.first().copied();
    if add {
        data.push(42); // OK: first borrow ended
    }
    if let Some(f) = first {
        println!("First was: {}", f);
    }
}

/// NLL with match arms.
pub fn nll_match(opt: &mut Option<String>) -> Option<&str> {
    match opt {
        Some(s) => Some(s.as_str()),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nll_basic() {
        let v = nll_basic();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_nll_conditional() {
        let mut data = vec![1, 2, 3];
        nll_conditional(&mut data, true);
        assert_eq!(data.len(), 4);
    }

    #[test]
    fn test_nll_match() {
        let mut opt = Some(String::from("hello"));
        let result = nll_match(&mut opt);
        assert_eq!(result, Some("hello"));
    }
}
