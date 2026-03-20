#![allow(clippy::all)]
/// Returns the maximum element in a list using the iterator method.
/// Idiomatic Rust: borrows the slice and uses .max() directly.
pub fn list_max(list: &[i32]) -> Option<i32> {
    list.iter().copied().max()
}

/// Returns the maximum element using recursion.
/// Functional style: matches on list structure, closer to OCaml.
pub fn list_max_recursive(list: &[i32]) -> Option<i32> {
    match list {
        [] => None,
        [h] => Some(*h),
        [h, ref rest @ ..] => list_max_recursive(rest).map(|m| (*h).max(m)),
    }
}

/// Returns the first element of a list as Option.
/// Safe alternative to indexing — no panic on empty list.
pub fn safe_head(list: &[i32]) -> Option<i32> {
    list.first().copied()
}

/// Maps a function over an optional value.
/// This is just Option::map, but shown explicitly for pedagogy.
pub fn option_map<T, U>(f: impl Fn(T) -> U, opt: Option<T>) -> Option<U> {
    opt.map(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_max_empty() {
        assert_eq!(list_max(&[]), None);
    }

    #[test]
    fn test_list_max_single() {
        assert_eq!(list_max(&[42]), Some(42));
    }

    #[test]
    fn test_list_max_multiple() {
        assert_eq!(list_max(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
    }

    #[test]
    fn test_list_max_negative() {
        assert_eq!(list_max(&[-5, -2, -10, -1]), Some(-1));
    }

    #[test]
    fn test_list_max_recursive_empty() {
        assert_eq!(list_max_recursive(&[]), None);
    }

    #[test]
    fn test_list_max_recursive_single() {
        assert_eq!(list_max_recursive(&[42]), Some(42));
    }

    #[test]
    fn test_list_max_recursive_multiple() {
        assert_eq!(list_max_recursive(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
    }

    #[test]
    fn test_list_max_recursive_negative() {
        assert_eq!(list_max_recursive(&[-5, -2, -10, -1]), Some(-1));
    }

    #[test]
    fn test_safe_head_empty() {
        assert_eq!(safe_head(&[]), None);
    }

    #[test]
    fn test_safe_head_single() {
        assert_eq!(safe_head(&[99]), Some(99));
    }

    #[test]
    fn test_safe_head_multiple() {
        assert_eq!(safe_head(&[10, 20, 30]), Some(10));
    }

    #[test]
    fn test_option_map_some() {
        let opt = Some(5);
        assert_eq!(option_map(|x| x * 2, opt), Some(10));
    }

    #[test]
    fn test_option_map_none() {
        let opt: Option<i32> = None;
        assert_eq!(option_map(|x| x * 2, opt), None);
    }

    #[test]
    fn test_option_map_with_max() {
        let nums = [3, 1, 4, 1, 5, 9, 2, 6];
        let doubled_max = option_map(|x| x * 2, list_max(&nums));
        assert_eq!(doubled_max, Some(18));
    }

    #[test]
    fn test_option_map_with_empty() {
        let nums: [i32; 0] = [];
        let doubled_max = option_map(|x| x * 2, list_max(&nums));
        assert_eq!(doubled_max, None);
    }
}
