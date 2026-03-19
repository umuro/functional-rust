#![allow(clippy::all)]
//! 272. One-level flattening with flatten()
//!
//! `flatten()` removes exactly one level of iterator nesting.

#[cfg(test)]
mod tests {
    #[test]
    fn test_flatten_vec_vec() {
        let nested = vec![vec![1i32, 2], vec![3, 4]];
        let flat: Vec<i32> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_flatten_option() {
        assert_eq!(Some(Some(42i32)).flatten(), Some(42));
        assert_eq!(Some(None::<i32>).flatten(), None);
        assert_eq!(None::<Option<i32>>.flatten(), None);
    }

    #[test]
    fn test_flatten_options_in_iter() {
        let v: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
        let result: Vec<i32> = v.into_iter().flatten().collect();
        assert_eq!(result, vec![1, 3]);
    }
}
