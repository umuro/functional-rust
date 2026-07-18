#![allow(clippy::all)]
// Rotate a list left by n positions, handling negative n and n >= len via modular arithmetic
pub fn rotate_left<T: Clone>(v: &[T], n: i64) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }
    let len = v.len() as i64;
    let n = (((n % len) + len) % len) as usize;
    let mut result = v[n..].to_vec();
    result.extend_from_slice(&v[..n]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_by_two() {
        assert_eq!(rotate_left(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn test_rotate_by_zero() {
        assert_eq!(rotate_left(&[1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_by_len_is_identity() {
        assert_eq!(rotate_left(&[1, 2, 3], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_rotate_more_than_len_wraps() {
        assert_eq!(rotate_left(&[1, 2, 3], 7), vec![2, 3, 1]);
    }

    #[test]
    fn test_rotate_negative() {
        assert_eq!(rotate_left(&[1, 2, 3, 4, 5], -2), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_rotate_empty_list() {
        let empty: Vec<i32> = vec![];
        assert_eq!(rotate_left(&empty, 3), empty);
    }
}
