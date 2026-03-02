//! # Pack Consecutive Duplicates
//! OCaml 99 Problems #9 — Pack consecutive duplicates into sublists.

/// Idiomatic Rust: imperative with explicit groups.
/// Slice-based input, returns owned Vec<Vec<T>>.
pub fn pack<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
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

/// Functional style: fold-based, mirrors the OCaml accumulator pattern.
pub fn pack_fold<T: PartialEq + Clone>(list: &[T]) -> Vec<Vec<T>> {
    list.iter().fold(Vec::new(), |mut acc: Vec<Vec<T>>, item| {
        match acc.last_mut() {
            Some(group) if group[0] == *item => {
                group.push(item.clone());
            }
            _ => {
                acc.push(vec![item.clone()]);
            }
        }
        acc
    })
}

/// Slice-based: returns groups as index ranges (zero-copy where possible).
/// Most efficient — avoids cloning entirely, returns `&[T]` slices.
pub fn pack_slices<T: PartialEq>(list: &[T]) -> Vec<&[T]> {
    if list.is_empty() {
        return vec![];
    }
    let mut result = Vec::new();
    let mut start = 0;
    for i in 1..list.len() {
        if list[i] != list[start] {
            result.push(&list[start..i]);
            start = i;
        }
    }
    result.push(&list[start..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack() {
        let input = vec!["a", "a", "b", "c", "c", "c"];
        let expected = vec![vec!["a", "a"], vec!["b"], vec!["c", "c", "c"]];
        assert_eq!(pack(&input), expected);
        assert_eq!(pack_fold(&input), expected);
    }

    #[test]
    fn test_empty() {
        assert_eq!(pack::<i32>(&[]), Vec::<Vec<i32>>::new());
        assert_eq!(pack_fold::<i32>(&[]), Vec::<Vec<i32>>::new());
        assert_eq!(pack_slices::<i32>(&[]), Vec::<&[i32]>::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(pack(&[1]), vec![vec![1]]);
        assert_eq!(pack_fold(&[1]), vec![vec![1]]);
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(pack(&[1, 2, 3]), vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(pack(&[5, 5, 5]), vec![vec![5, 5, 5]]);
    }

    #[test]
    fn test_slices() {
        let input = [1, 1, 2, 3, 3];
        let result = pack_slices(&input);
        assert_eq!(result, vec![&[1, 1][..], &[2][..], &[3, 3][..]]);
    }
}
