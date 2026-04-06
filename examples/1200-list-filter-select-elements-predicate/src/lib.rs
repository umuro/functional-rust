#![allow(dead_code)]

/// Idiomatic Rust: filter a slice using a predicate, returning borrowed refs.
/// Takes &[T] to borrow without allocation; predicate is a closure or fn.
pub fn filter<T, F>(list: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).collect()
}

/// Filter and clone elements (owned output), mirroring OCaml's List.filter.
pub fn filter_cloned<T: Clone, F>(list: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Functional/recursive — mirrors the OCaml recursive pattern explicitly.
/// Uses an inner fn with `&dyn Fn` to avoid infinite type instantiation.
pub fn filter_recursive<T: Clone, F>(list: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    fn go<T: Clone>(list: &[T], predicate: &dyn Fn(&T) -> bool) -> Vec<T> {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, predicate);
                if predicate(head) {
                    let mut result = vec![head.clone()];
                    result.append(&mut rest);
                    result
                } else {
                    rest
                }
            }
        }
    }
    go(list, &predicate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter(empty, |x| x % 2 == 0), Vec::<&i32>::new());
    }

    #[test]
    fn test_filter_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens: Vec<&i32> = filter(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, [&2, &4, &6, &8]);
    }

    #[test]
    fn test_filter_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let odds: Vec<&i32> = filter(&numbers, |x| x % 2 != 0);
        assert_eq!(odds, [&1, &3, &5, &7]);
    }

    #[test]
    fn test_filter_none_match() {
        let numbers = [1, 3, 5, 7];
        assert_eq!(filter(&numbers, |x| x % 2 == 0), Vec::<&i32>::new());
    }

    #[test]
    fn test_filter_all_match() {
        let numbers = [2, 4, 6];
        let result = filter(&numbers, |x| x % 2 == 0);
        assert_eq!(result, [&2, &4, &6]);
    }

    #[test]
    fn test_filter_cloned_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_cloned(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, [2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_recursive_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, [2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_recursive_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter_recursive(empty, |x| x % 2 == 0), Vec::<i32>::new());
    }

    #[test]
    fn test_filter_strings() {
        let words = ["apple", "banana", "cherry", "date"];
        let long_words: Vec<&&str> = filter(&words, |w| w.len() > 5);
        assert_eq!(long_words, [&"banana", &"cherry"]);
    }

    #[test]
    fn test_filter_single_element_match() {
        let numbers = [42];
        assert_eq!(filter(&numbers, |x| *x == 42), [&42]);
    }

    #[test]
    fn test_filter_single_element_no_match() {
        let numbers = [42];
        assert_eq!(filter(&numbers, |x| *x == 0), Vec::<&i32>::new());
    }
}
