#![allow(clippy::all)]
//! 271. Transform-and-Find with find_map()
//!
//! `find_map(f)` finds the first `Some(...)` result — single pass, lazy.
//! Equivalent to `filter_map(f).next()` but expresses intent more clearly.

/// Parse the first valid integer from a slice of strings.
pub fn first_int(strings: &[&str]) -> Option<i32> {
    strings.iter().find_map(|s| s.parse::<i32>().ok())
}

/// Find the first string longer than `min_len` and return its length.
pub fn first_long_len(strings: &[&str], min_len: usize) -> Option<usize> {
    strings.iter().find_map(|s| {
        if s.len() > min_len {
            Some(s.len())
        } else {
            None
        }
    })
}

/// Parse the first `key=value` entry from a slice of config-style strings.
pub fn first_kv<'a>(entries: &[&'a str]) -> Option<(&'a str, &'a str)> {
    entries.iter().find_map(|s| s.split_once('='))
}

/// Recursive implementation mirroring OCaml's `List.find_map`.
pub fn find_map_rec<T, B, F>(list: &[T], f: F) -> Option<B>
where
    F: Fn(&T) -> Option<B>,
{
    match list {
        [] => None,
        [head, tail @ ..] => f(head).or_else(|| find_map_rec(tail, f)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_int_found() {
        let strings = ["hello", "42", "world", "17"];
        assert_eq!(first_int(&strings), Some(42));
    }

    #[test]
    fn test_first_int_none() {
        let strings = ["hello", "world", "foo"];
        assert_eq!(first_int(&strings), None);
    }

    #[test]
    fn test_first_int_empty() {
        assert_eq!(first_int(&[]), None);
    }

    #[test]
    fn test_first_long_len() {
        let strings = ["hi", "hello", "world", "rust"];
        assert_eq!(first_long_len(&strings, 4), Some(5));
    }

    #[test]
    fn test_first_long_len_none() {
        let strings = ["hi", "yo", "ok"];
        assert_eq!(first_long_len(&strings, 4), None);
    }

    #[test]
    fn test_first_kv_found() {
        let entries = ["BAD", "PATH=/usr/bin", "HOME=/root"];
        assert_eq!(first_kv(&entries), Some(("PATH", "/usr/bin")));
    }

    #[test]
    fn test_first_kv_none() {
        let entries = ["noequals", "alsonone"];
        assert_eq!(first_kv(&entries), None);
    }

    #[test]
    fn test_find_map_rec() {
        let nums = [1i32, 2, 3, 10, 4];
        let result = find_map_rec(&nums, |&n| if n > 5 { Some(n * 2) } else { None });
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_find_map_rec_none() {
        let nums = [1i32, 2, 3];
        let result = find_map_rec(&nums, |&n| if n > 100 { Some(n) } else { None });
        assert_eq!(result, None);
    }

    #[test]
    fn test_find_map_vs_filter_map_next() {
        // find_map is equivalent to filter_map(...).next()
        let strings = ["a", "2", "b", "3"];
        let via_find_map = strings.iter().find_map(|s| s.parse::<i32>().ok());
        let via_filter_map = strings.iter().filter_map(|s| s.parse::<i32>().ok()).next();
        assert_eq!(via_find_map, via_filter_map);
        assert_eq!(via_find_map, Some(2));
    }
}
