//! Higher-Order Functions
//!
//! Rust's iterator HOFs: map, filter, fold, flat_map, zip, and custom ones.

/// Custom HOF: zip two slices with a combining function.
pub fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    F: Fn(&A, &B) -> C,
{
    a.iter().zip(b.iter()).map(|(x, y)| f(x, y)).collect()
}

/// Custom HOF: scan (like fold but returns all intermediate values).
pub fn scan_left<T: Clone, U: Clone, F>(items: &[T], init: U, f: F) -> Vec<U>
where
    F: Fn(U, &T) -> U,
{
    let mut acc = init;
    let mut result = vec![acc.clone()];
    for item in items {
        acc = f(acc, item);
        result.push(acc.clone());
    }
    result
}

/// Custom HOF: partition by predicate.
pub fn partition_by<T, P>(items: Vec<T>, pred: P) -> (Vec<T>, Vec<T>)
where
    P: Fn(&T) -> bool,
{
    items.into_iter().partition(pred)
}

/// Custom HOF: take while predicate holds.
pub fn take_while_custom<T, P>(items: &[T], pred: P) -> Vec<T>
where
    T: Clone,
    P: Fn(&T) -> bool,
{
    items.iter().take_while(|x| pred(x)).cloned().collect()
}

/// Custom HOF: group consecutive elements.
pub fn group_by<T: Clone + PartialEq>(items: &[T]) -> Vec<Vec<T>> {
    if items.is_empty() {
        return vec![];
    }
    let mut groups = vec![vec![items[0].clone()]];
    for item in items.iter().skip(1) {
        if groups.last().unwrap().last() == Some(item) {
            groups.last_mut().unwrap().push(item.clone());
        } else {
            groups.push(vec![item.clone()]);
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_with_add() {
        let a = [1, 2, 3];
        let b = [10, 20, 30];
        let result = zip_with(&a, &b, |x, y| x + y);
        assert_eq!(result, vec![11, 22, 33]);
    }

    #[test]
    fn test_zip_with_concat() {
        let a = ["a", "b"];
        let b = ["1", "2"];
        let result: Vec<String> = zip_with(&a, &b, |x, y| format!("{}{}", x, y));
        assert_eq!(result, vec!["a1", "b2"]);
    }

    #[test]
    fn test_scan_left_sum() {
        let nums = [1, 2, 3, 4];
        let result = scan_left(&nums, 0, |acc, &x| acc + x);
        assert_eq!(result, vec![0, 1, 3, 6, 10]);
    }

    #[test]
    fn test_partition_by() {
        let (evens, odds) = partition_by(vec![1, 2, 3, 4, 5, 6], |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_take_while_custom() {
        let nums = [1, 2, 3, 10, 4, 5];
        let result = take_while_custom(&nums, |&x| x < 5);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_group_by() {
        let items = [1, 1, 2, 2, 2, 3, 1, 1];
        let groups = group_by(&items);
        assert_eq!(groups, vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1, 1]]);
    }

    #[test]
    fn test_standard_hofs() {
        let nums = vec![1, 2, 3, 4, 5];

        // map
        let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

        // filter
        let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).cloned().collect();
        assert_eq!(evens, vec![2, 4]);

        // fold
        let sum: i32 = nums.iter().fold(0, |acc, &x| acc + x);
        assert_eq!(sum, 15);
    }
}
