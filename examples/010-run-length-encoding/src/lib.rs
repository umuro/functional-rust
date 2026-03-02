//! # Run-Length Encoding
//! OCaml 99 Problems #10 — Encode consecutive duplicates as (count, element) pairs.

/// Idiomatic Rust: pack then map to (count, element).
pub fn encode<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    pack(list)
        .into_iter()
        .map(|group| (group.len(), group[0].clone()))
        .collect()
}

/// Functional style: single-pass fold, no intermediate packing.
pub fn encode_fold<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    list.iter().fold(Vec::new(), |mut acc, item| {
        match acc.last_mut() {
            Some((count, ref val)) if val == item => {
                *count += 1;
            }
            _ => {
                acc.push((1, item.clone()));
            }
        }
        acc
    })
}

/// Direct approach: iterate with counting, no helper function.
pub fn encode_direct<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    if list.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut count = 1;
    for i in 1..list.len() {
        if list[i] == list[i - 1] {
            count += 1;
        } else {
            result.push((count, list[i - 1].clone()));
            count = 1;
        }
    }
    result.push((count, list[list.len() - 1].clone()));
    result
}

/// Helper: pack consecutive duplicates (from example 009).
fn pack<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
    if list.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut current = vec![list[0].clone()];
    for item in &list[1..] {
        if *item == current[0] {
            current.push(item.clone());
        } else {
            result.push(current);
            current = vec![item.clone()];
        }
    }
    result.push(current);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = vec!["a", "a", "b", "c", "c", "c"];
        let expected = vec![(2, "a"), (1, "b"), (3, "c")];
        assert_eq!(encode(&input), expected);
        assert_eq!(encode_fold(&input), expected);
        assert_eq!(encode_direct(&input), expected);
    }

    #[test]
    fn test_empty() {
        assert_eq!(encode::<i32>(&[]), Vec::<(usize, i32)>::new());
        assert_eq!(encode_fold::<i32>(&[]), Vec::<(usize, i32)>::new());
        assert_eq!(encode_direct::<i32>(&[]), Vec::<(usize, i32)>::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(encode(&[1]), vec![(1, 1)]);
        assert_eq!(encode_fold(&[1]), vec![(1, 1)]);
        assert_eq!(encode_direct(&[1]), vec![(1, 1)]);
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(encode(&[1, 2, 3]), vec![(1, 1), (1, 2), (1, 3)]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(encode(&[5, 5, 5, 5]), vec![(4, 5)]);
    }
}
