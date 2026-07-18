#![allow(clippy::all)]
// Extract a slice [i, k] from a list (OCaml 99 Problems #18), 1-based inclusive
pub fn slice_list<T: Clone>(list: &[T], i: usize, k: usize) -> Vec<T> {
    if i == 0 || i > list.len() || k < i {
        return Vec::new();
    }
    let end = k.min(list.len());
    list[(i - 1)..end].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_middle() {
        let v: Vec<i32> = (1..=10).collect();
        assert_eq!(slice_list(&v, 3, 7), vec![3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_slice_single_element() {
        let v = [10, 20, 30];
        assert_eq!(slice_list(&v, 2, 2), vec![20]);
    }

    #[test]
    fn test_slice_whole_list() {
        let v = [1, 2, 3];
        assert_eq!(slice_list(&v, 1, 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_slice_k_beyond_end_clamps() {
        let v = [1, 2, 3];
        assert_eq!(slice_list(&v, 2, 100), vec![2, 3]);
    }

    #[test]
    fn test_slice_i_zero_is_empty() {
        let v = [1, 2, 3];
        let empty: Vec<i32> = vec![];
        assert_eq!(slice_list(&v, 0, 2), empty);
    }

    #[test]
    fn test_slice_i_greater_than_k_is_empty() {
        let v = [1, 2, 3];
        let empty: Vec<i32> = vec![];
        assert_eq!(slice_list(&v, 3, 1), empty);
    }
}
