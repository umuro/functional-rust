/// Idiomatic Rust: filter using iterator `.filter()` with a predicate closure.
/// Returns a new Vec containing only elements for which the predicate returns true.
pub fn filter<T, F>(items: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| predicate(x)).collect()
}

/// Idiomatic Rust: filter owned values, consuming the iterator.
pub fn filter_owned<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.into_iter().filter(|x| predicate(x)).collect()
}

/// Recursive Rust: closer to the OCaml `List.filter` style.
pub fn filter_recursive<'a, T, F>(items: &'a [T], predicate: &F) -> Vec<&'a T>
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => vec![],
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, predicate);
            if predicate(head) {
                let mut result = vec![head];
                result.append(&mut rest);
                result
            } else {
                rest
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter(empty, |x| *x % 2 == 0), Vec::<&i32>::new());
    }

    #[test]
    fn test_filter_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens: Vec<&i32> = filter(&numbers, |x| *x % 2 == 0);
        assert_eq!(evens, vec![&2, &4, &6, &8]);
    }

    #[test]
    fn test_filter_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let odds: Vec<&i32> = filter(&numbers, |x| *x % 2 != 0);
        assert_eq!(odds, vec![&1, &3, &5, &7]);
    }

    #[test]
    fn test_filter_all_pass() {
        let numbers = [2, 4, 6, 8];
        let evens: Vec<&i32> = filter(&numbers, |x| *x % 2 == 0);
        assert_eq!(evens, vec![&2, &4, &6, &8]);
    }

    #[test]
    fn test_filter_none_pass() {
        let numbers = [1, 3, 5, 7];
        let evens: Vec<&i32> = filter(&numbers, |x| *x % 2 == 0);
        assert_eq!(evens, Vec::<&i32>::new());
    }

    #[test]
    fn test_filter_owned_evens() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_owned(numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_recursive_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_recursive(&numbers, &|x: &i32| *x % 2 == 0);
        assert_eq!(evens, vec![&2, &4, &6, &8]);
    }

    #[test]
    fn test_filter_recursive_empty() {
        let empty: &[i32] = &[];
        assert_eq!(
            filter_recursive(empty, &|x: &i32| *x % 2 == 0),
            Vec::<&i32>::new()
        );
    }

    #[test]
    fn test_filter_strings() {
        let words = ["apple", "banana", "apricot", "cherry"];
        let a_words: Vec<&&str> = filter(&words, |w| w.starts_with('a'));
        assert_eq!(a_words, vec![&"apple", &"apricot"]);
    }

    #[test]
    fn test_filter_single_element_passes() {
        let single = [42];
        let result: Vec<&i32> = filter(&single, |x| *x > 10);
        assert_eq!(result, vec![&42]);
    }

    #[test]
    fn test_filter_single_element_fails() {
        let single = [5];
        let result: Vec<&i32> = filter(&single, |x| *x > 10);
        assert_eq!(result, Vec::<&i32>::new());
    }
}
