#![allow(clippy::all)]
//! List.map — Transform Every Element
//!
//! Apply a function to each element of a list.

/// Idiomatic Rust: use iterators and map.
pub fn map_idiomatic<F, T, U>(list: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    list.iter().map(f).collect()
}

/// Recursive implementation: similar to OCaml's List.map.
pub fn map_recursive<F, T, U>(list: &[T], f: &F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_recursive(tail, f));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty: Vec<i32> = vec![];
        let result = map_idiomatic(&empty, |x| x * 2);
        assert_eq!(result, vec![]);
        let result2 = map_recursive(&empty, &|x| x * 2);
        assert_eq!(result2, vec![]);
    }

    #[test]
    fn test_single() {
        let list = vec![1];
        let result = map_idiomatic(&list, |x| x * 2);
        assert_eq!(result, vec![2]);
        let result2 = map_recursive(&list, &|x| x * 2);
        assert_eq!(result2, vec![2]);
    }

    #[test]
    fn test_multiple() {
        let list = vec![1, 2, 3, 4, 5];
        let result = map_idiomatic(&list, |x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
        let result2 = map_recursive(&list, &|x| x * 2);
        assert_eq!(result2, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_strings() {
        let list = vec!["a", "bb", "ccc"];
        let result = map_idiomatic(&list, |s| s.len());
        assert_eq!(result, vec![1, 2, 3]);
        let result2 = map_recursive(&list, &|s| s.len());
        assert_eq!(result2, vec![1, 2, 3]);
    }
}
