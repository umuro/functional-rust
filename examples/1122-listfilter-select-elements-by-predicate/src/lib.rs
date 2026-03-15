/// Filter elements from a slice that satisfy the predicate.
///
/// Solution 1: Idiomatic Rust — iterator `.filter()` chain.
/// Mirrors OCaml's `List.filter` directly.
pub fn filter_idiomatic<T, F>(list: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    list.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Split a slice into matching and non-matching elements in one pass.
///
/// Solution 2: Partition — more efficient than filtering twice when you need
/// both halves, as in the OCaml example that computes both evens and odds.
pub fn partition_by<T, F>(list: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    // iter() yields &T; partition collects into (Vec<&T>, Vec<&T>)
    let (yes, no): (Vec<&T>, Vec<&T>) = list.iter().partition(|x| predicate(x));
    (
        yes.into_iter().cloned().collect(),
        no.into_iter().cloned().collect(),
    )
}

/// Filter elements recursively — mirrors OCaml's explicit pattern matching.
///
/// Solution 3: Recursive, closest to OCaml's structural recursion on lists.
pub fn filter_recursive<T, F>(list: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    fn go<T, F>(list: &[T], pred: &F) -> Vec<T>
    where
        T: Clone,
        F: Fn(&T) -> bool,
    {
        match list {
            [] => vec![],
            [head, tail @ ..] => {
                let mut rest = go(tail, pred);
                if pred(head) {
                    // Prepend: mirrors OCaml's `x :: filter pred rest`
                    rest.insert(0, head.clone());
                }
                rest
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
        assert_eq!(filter_idiomatic(empty, |x| x % 2 == 0), Vec::<i32>::new());
    }

    #[test]
    fn test_filter_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(filter_idiomatic(&numbers, |x| x % 2 == 0), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(filter_idiomatic(&numbers, |x| x % 2 != 0), vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_none_match() {
        let numbers = [1, 3, 5];
        assert_eq!(
            filter_idiomatic(&numbers, |x| x % 2 == 0),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_filter_all_match() {
        let numbers = [2, 4, 6];
        assert_eq!(filter_idiomatic(&numbers, |x| x % 2 == 0), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_by_threshold() {
        let numbers = [1, 5, 10, 15, 20];
        assert_eq!(filter_idiomatic(&numbers, |x| *x > 10), vec![15, 20]);
    }

    #[test]
    fn test_partition_evens_and_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let (evens, odds) = partition_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
        assert_eq!(odds, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_recursive_matches_idiomatic() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            filter_recursive(&numbers, |x| x % 2 == 0),
            filter_idiomatic(&numbers, |x| x % 2 == 0)
        );
    }
}
