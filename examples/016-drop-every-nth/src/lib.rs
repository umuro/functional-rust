#![allow(clippy::all)]
// Drop every nth element (OCaml 99 Problems #16), 1-indexed: drop positions n, 2n, 3n, ...
pub fn drop_every_nth<T: Clone>(list: &[T], n: usize) -> Vec<T> {
    list.iter()
        .enumerate()
        .filter(|(i, _)| (i + 1) % n != 0)
        .map(|(_, x)| x.clone())
        .collect()
}

// Recursive accumulator version, mirroring the OCaml idiom
pub fn drop_every_nth_rec<T: Clone>(list: &[T], n: usize) -> Vec<T> {
    fn go<T: Clone>(list: &[T], n: usize, count: usize, acc: &mut Vec<T>) {
        if let Some((head, tail)) = list.split_first() {
            if count == n {
                go(tail, n, 1, acc);
            } else {
                acc.push(head.clone());
                go(tail, n, count + 1, acc);
            }
        }
    }
    let mut acc = Vec::new();
    go(list, n, 1, &mut acc);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_every_third() {
        assert_eq!(drop_every_nth(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 3), vec![1, 2, 4, 5, 7, 8]);
    }

    #[test]
    fn test_drop_every_second() {
        assert_eq!(drop_every_nth(&[1, 2, 3, 4, 5, 6], 2), vec![1, 3, 5]);
    }

    #[test]
    fn test_n_one_drops_all() {
        let expected: Vec<i32> = vec![];
        assert_eq!(drop_every_nth(&[1, 2, 3], 1), expected);
    }

    #[test]
    fn test_n_larger_than_list_keeps_all() {
        assert_eq!(drop_every_nth(&[1, 2, 3], 10), vec![1, 2, 3]);
    }

    #[test]
    fn test_empty_list() {
        let empty: Vec<i32> = vec![];
        assert_eq!(drop_every_nth(&empty, 3), empty);
    }

    #[test]
    fn test_recursive_matches_iterative() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(drop_every_nth_rec(&list, 3), drop_every_nth(&list, 3));
    }
}
