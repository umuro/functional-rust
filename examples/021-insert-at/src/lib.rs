#![allow(clippy::all)]
// Insert an element at position k (OCaml 99 Problems #21), 0-based
pub fn insert_at<T: Clone>(v: &[T], k: usize, elem: T) -> Vec<T> {
    let k = k.min(v.len());
    let mut result = v[..k].to_vec();
    result.push(elem);
    result.extend_from_slice(&v[k..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_middle() {
        assert_eq!(insert_at(&[1, 2, 3, 4, 5], 2, 99), vec![1, 2, 99, 3, 4, 5]);
    }

    #[test]
    fn test_insert_at_front() {
        assert_eq!(insert_at(&[1, 2, 3], 0, 0), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_insert_at_end() {
        assert_eq!(insert_at(&[1, 2, 3], 3, 4), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_insert_beyond_len_appends() {
        assert_eq!(insert_at(&[1, 2, 3], 99, 4), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_insert_into_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(insert_at(&empty, 0, 1), vec![1]);
    }
}
