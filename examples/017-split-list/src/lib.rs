#![allow(clippy::all)]
// Split a list at position n (OCaml 99 Problems #17), 0-based: (list[..n], list[n..])
pub fn split_list<T: Clone>(list: &[T], n: usize) -> (Vec<T>, Vec<T>) {
    let n = n.min(list.len());
    let (left, right) = list.split_at(n);
    (left.to_vec(), right.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_middle() {
        assert_eq!(split_list(&[1, 2, 3, 4, 5], 2), (vec![1, 2], vec![3, 4, 5]));
    }

    #[test]
    fn test_split_at_zero() {
        assert_eq!(split_list(&[1, 2, 3], 0), (vec![], vec![1, 2, 3]));
    }

    #[test]
    fn test_split_at_len() {
        assert_eq!(split_list(&[1, 2, 3], 3), (vec![1, 2, 3], vec![]));
    }

    #[test]
    fn test_split_n_greater_than_len() {
        assert_eq!(split_list(&[1, 2, 3], 10), (vec![1, 2, 3], vec![]));
    }

    #[test]
    fn test_split_empty_list() {
        let empty: Vec<i32> = vec![];
        assert_eq!(split_list(&empty, 2), (vec![], vec![]));
    }
}
