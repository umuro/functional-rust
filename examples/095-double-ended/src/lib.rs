#![allow(clippy::all)]
// 095: Double-Ended Iterator

fn is_palindrome(v: &[i32]) -> bool {
    v.iter().eq(v.iter().rev())
}

fn take_from_both(v: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut iter = v.iter();
    let front: Vec<i32> = (0..2).filter_map(|_| iter.next().copied()).collect();
    let back: Vec<i32> = (0..2).filter_map(|_| iter.next_back().copied()).collect();
    (front, back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(&[1, 2, 3, 2, 1]));
        assert!(!is_palindrome(&[1, 2, 3]));
    }

    #[test]
    fn test_take_both() {
        let (f, b) = take_from_both(&[1, 2, 3, 4, 5]);
        assert_eq!(f, vec![1, 2]);
        assert_eq!(b, vec![5, 4]);
    }

    #[test]
    fn test_next_back() {
        let v = vec![1, 2, 3];
        let mut iter = v.iter();
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);
    }
}
