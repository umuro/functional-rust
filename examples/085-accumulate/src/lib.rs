/// Accumulate — Custom Map Implementation
///
/// Ownership: The function takes ownership of input vec and returns new vec.
/// The closure borrows or moves each element depending on the function.

/// Recursive version (not tail-recursive — will stack overflow on large inputs)
pub fn accumulate<T, U, F>(lst: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    fn inner<T, U>(lst: &[T], f: &dyn Fn(&T) -> U) -> Vec<U> {
        match lst {
            [] => vec![],
            [head, tail @ ..] => {
                let mut result = vec![f(head)];
                result.extend(inner(tail, f));
                result
            }
        }
    }
    inner(lst, &f)
}

/// Tail-recursive version using accumulator
pub fn accumulate_tr<T, U, F>(lst: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    let mut acc = Vec::with_capacity(lst.len());
    for item in lst {
        acc.push(f(item));
    }
    acc
}

/// Iterator-based version (most idiomatic Rust)
pub fn accumulate_iter<T, U>(lst: impl IntoIterator<Item = T>, f: impl Fn(T) -> U) -> Vec<U> {
    lst.into_iter().map(f).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares() {
        assert_eq!(accumulate(&[1, 2, 3, 4, 5], |x| x * x), vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(accumulate::<i32, i32, _>(&[], |x| x * x), Vec::<i32>::new());
    }

    #[test]
    fn test_strings() {
        let words = vec!["hello".to_string(), "world".to_string()];
        assert_eq!(
            accumulate(&words, |s| s.to_uppercase()),
            vec!["HELLO", "WORLD"]
        );
    }

    #[test]
    fn test_tail_recursive() {
        assert_eq!(accumulate_tr(&[1, 2, 3], |x| x + 10), vec![11, 12, 13]);
    }

    #[test]
    fn test_iterator_version() {
        assert_eq!(accumulate_iter(vec![1, 2, 3], |x| x * 2), vec![2, 4, 6]);
    }

    #[test]
    fn test_type_change() {
        assert_eq!(accumulate(&[1, 2, 3], |x| x.to_string()), vec!["1", "2", "3"]);
    }
}
