/// Duplicate Elements (99 Problems #14)
///
/// Duplicate every element of a list.
/// [a; b; c] → [a; a; b; b; c; c]

// ── Idiomatic Rust: flat_map ────────────────────────────────────────────────

pub fn duplicate<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().flat_map(|x| vec![x.clone(), x.clone()]).collect()
}

// ── Using iterators more efficiently ────────────────────────────────────────

pub fn duplicate_iter<T: Clone>(list: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(list.len() * 2);
    for item in list {
        result.push(item.clone());
        result.push(item.clone());
    }
    result
}

// ── Recursive style ─────────────────────────────────────────────────────────

pub fn duplicate_recursive<T: Clone>(list: &[T]) -> Vec<T> {
    match list.split_first() {
        None => vec![],
        Some((head, tail)) => {
            let mut result = vec![head.clone(), head.clone()];
            result.extend(duplicate_recursive(tail));
            result
        }
    }
}

// ── Fold-based ──────────────────────────────────────────────────────────────

pub fn duplicate_fold<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().fold(Vec::with_capacity(list.len() * 2), |mut acc, x| {
        acc.push(x.clone());
        acc.push(x.clone());
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(duplicate::<i32>(&[]), vec![]);
        assert_eq!(duplicate_recursive::<i32>(&[]), vec![]);
    }

    #[test]
    fn test_single() {
        assert_eq!(duplicate(&[1]), vec![1, 1]);
    }

    #[test]
    fn test_multiple() {
        assert_eq!(duplicate(&[1, 2, 3]), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(duplicate_iter(&[1, 2, 3]), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(duplicate_recursive(&[1, 2, 3]), vec![1, 1, 2, 2, 3, 3]);
        assert_eq!(duplicate_fold(&[1, 2, 3]), vec![1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            duplicate(&["a", "b"]),
            vec!["a", "a", "b", "b"]
        );
    }

    #[test]
    fn test_large() {
        let input: Vec<i32> = (0..100).collect();
        let result = duplicate(&input);
        assert_eq!(result.len(), 200);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[198], 99);
        assert_eq!(result[199], 99);
    }
}
