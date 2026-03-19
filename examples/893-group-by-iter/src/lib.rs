#![allow(clippy::all)]
// Example 099: Group By Iterator
// Group consecutive equal (or same-key) elements without external crates.

// === Approach 1: Idiomatic Rust — slice-based, uses windows/peekable pattern ===
// Groups consecutive equal elements; returns Vec of grouped Vecs.
pub fn group_consecutive<T: PartialEq + Clone>(data: &[T]) -> Vec<Vec<T>> {
    let mut groups: Vec<Vec<T>> = Vec::new();
    let mut iter = data.iter();
    let Some(first) = iter.next() else {
        return groups;
    };
    let mut current = vec![first.clone()];

    for item in iter {
        if *item == current[0] {
            current.push(item.clone());
        } else {
            groups.push(current);
            current = vec![item.clone()];
        }
    }
    groups.push(current);
    groups
}

// === Approach 2: Group by key function — functional style, returns (key, group) pairs ===
pub fn group_by_key<T: Clone, K: PartialEq>(data: &[T], key: impl Fn(&T) -> K) -> Vec<(K, Vec<T>)> {
    let mut groups: Vec<(K, Vec<T>)> = Vec::new();
    let mut iter = data.iter();
    let Some(first) = iter.next() else {
        return groups;
    };
    let mut current_key = key(first);
    let mut current_group = vec![first.clone()];

    for item in iter {
        let k = key(item);
        if k == current_key {
            current_group.push(item.clone());
        } else {
            groups.push((current_key, current_group));
            current_key = k;
            current_group = vec![item.clone()];
        }
    }
    groups.push((current_key, current_group));
    groups
}

// === Approach 3: Run-length encoding — encodes as (element, count) pairs ===
pub fn run_length_encode<T: PartialEq + Clone>(data: &[T]) -> Vec<(T, usize)> {
    group_consecutive(data)
        .into_iter()
        .map(|g| {
            let len = g.len();
            // Safe: group_consecutive never produces empty groups
            (g.into_iter().next().unwrap(), len)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- group_consecutive ---

    #[test]
    fn test_group_consecutive_empty() {
        let result = group_consecutive::<i32>(&[]);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_group_consecutive_single() {
        assert_eq!(group_consecutive(&[42]), vec![vec![42]]);
    }

    #[test]
    fn test_group_consecutive_multiple() {
        let input = vec![1, 1, 2, 2, 2, 3, 1, 1];
        let expected = vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1, 1]];
        assert_eq!(group_consecutive(&input), expected);
    }

    #[test]
    fn test_group_consecutive_all_same() {
        assert_eq!(group_consecutive(&[5, 5, 5]), vec![vec![5, 5, 5]]);
    }

    #[test]
    fn test_group_consecutive_all_different() {
        assert_eq!(
            group_consecutive(&[1, 2, 3]),
            vec![vec![1], vec![2], vec![3]]
        );
    }

    #[test]
    fn test_group_consecutive_strings() {
        let input = vec!["a", "a", "b", "c", "c"];
        let expected = vec![vec!["a", "a"], vec!["b"], vec!["c", "c"]];
        assert_eq!(group_consecutive(&input), expected);
    }

    // --- group_by_key ---

    #[test]
    fn test_group_by_key_empty() {
        let result = group_by_key::<i32, bool>(&[], |x| *x > 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_group_by_key_sign() {
        let input = vec![1, 2, -1, -2, 3];
        let result = group_by_key(&input, |x| *x > 0);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], (true, vec![1, 2]));
        assert_eq!(result[1], (false, vec![-1, -2]));
        assert_eq!(result[2], (true, vec![3]));
    }

    #[test]
    fn test_group_by_key_parity() {
        let input = vec![2, 4, 1, 3, 6];
        let result = group_by_key(&input, |x| x % 2);
        assert_eq!(result[0], (0, vec![2, 4]));
        assert_eq!(result[1], (1, vec![1, 3]));
        assert_eq!(result[2], (0, vec![6]));
    }

    // --- run_length_encode ---

    #[test]
    fn test_rle_empty() {
        let result = run_length_encode::<char>(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_rle_typical() {
        let input = vec!['a', 'a', 'a', 'b', 'b', 'c'];
        let expected = vec![('a', 3), ('b', 2), ('c', 1)];
        assert_eq!(run_length_encode(&input), expected);
    }

    #[test]
    fn test_rle_no_repeats() {
        let input = vec![1, 2, 3];
        let expected = vec![(1, 1), (2, 1), (3, 1)];
        assert_eq!(run_length_encode(&input), expected);
    }

    #[test]
    fn test_rle_single_run() {
        assert_eq!(run_length_encode(&[7, 7, 7, 7]), vec![(7, 4)]);
    }
}
