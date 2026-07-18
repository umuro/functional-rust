#![allow(clippy::all)]
// Remove the kth element (OCaml 99 Problems #20), 1-based; returns the removed element and the rest
pub fn remove_kth<T: Clone>(list: &[T], k: usize) -> Option<(T, Vec<T>)> {
    if k == 0 || k > list.len() {
        return None;
    }
    let idx = k - 1;
    let elem = list[idx].clone();
    let mut rest = list[..idx].to_vec();
    rest.extend_from_slice(&list[idx + 1..]);
    Some((elem, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_middle() {
        assert_eq!(remove_kth(&[1, 2, 3, 4, 5], 3), Some((3, vec![1, 2, 4, 5])));
    }

    #[test]
    fn test_remove_first() {
        assert_eq!(remove_kth(&[1, 2, 3], 1), Some((1, vec![2, 3])));
    }

    #[test]
    fn test_remove_last() {
        assert_eq!(remove_kth(&[1, 2, 3], 3), Some((3, vec![1, 2])));
    }

    #[test]
    fn test_remove_k_zero_is_none() {
        assert_eq!(remove_kth(&[1, 2, 3], 0), None);
    }

    #[test]
    fn test_remove_k_out_of_bounds_is_none() {
        assert_eq!(remove_kth(&[1, 2, 3], 10), None);
    }

    #[test]
    fn test_remove_from_empty_is_none() {
        let empty: Vec<i32> = vec![];
        assert_eq!(remove_kth(&empty, 1), None);
    }
}
