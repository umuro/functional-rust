// Solution 1: Idiomatic Rust — iterator `.filter()` with a closure predicate
pub fn filter_by<T, F>(items: &[T], predicate: F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| predicate(x)).copied().collect()
}

// Solution 2: Functional/recursive — mirrors OCaml's List.filter recursion
// Takes predicate by reference to avoid wrapping it in another &F on each recursive call
pub fn filter_recursive<T, F>(list: &[T], predicate: &F) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, predicate);
            if predicate(head) {
                rest.insert(0, *head);
            }
            rest
        }
    }
}

// Solution 3: Using partition to split into two groups simultaneously
pub fn partition_by<T, F>(items: &[T], predicate: F) -> (Vec<T>, Vec<T>)
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().partition(|x| predicate(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let result = filter_by::<i32, _>(&[], |x| x % 2 == 0);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_filter_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let odds = filter_by(&numbers, |x| x % 2 != 0);
        assert_eq!(odds, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_none_match() {
        let numbers = [1, 3, 5, 7];
        let evens = filter_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![]);
    }

    #[test]
    fn test_filter_all_match() {
        let numbers = [2, 4, 6, 8];
        let evens = filter_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_recursive_evens() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_recursive(&numbers, &|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_partition_splits_correctly() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let (evens, odds) = partition_by(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
        assert_eq!(odds, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_by_threshold() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        let large = filter_by(&numbers, |&x| x > 5);
        assert_eq!(large, vec![6, 7, 8]);
    }
}
