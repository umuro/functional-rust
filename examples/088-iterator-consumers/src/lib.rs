#![allow(clippy::all)]
// 088: Iterator Consumers
// Terminal operations that drive the lazy chain

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum() {
        assert_eq!((1..=5).sum::<i32>(), 15);
    }

    #[test]
    fn test_product() {
        assert_eq!((1..=5).product::<i32>(), 120);
    }

    #[test]
    fn test_count() {
        assert_eq!((0..10).count(), 10);
    }

    #[test]
    fn test_collect() {
        let v: Vec<i32> = (0..3).collect();
        assert_eq!(v, vec![0, 1, 2]);
    }

    #[test]
    fn test_fold() {
        assert_eq!((1..=5).fold(0, |acc, x| acc + x), 15);
    }

    #[test]
    fn test_min_max() {
        assert_eq!([3, 1, 4, 1, 5].iter().min(), Some(&1));
        assert_eq!([3, 1, 4, 1, 5].iter().max(), Some(&5));
    }

    #[test]
    fn test_any_all() {
        assert!([1, 2, 3, 4].iter().any(|&x| x > 3));
        assert!(![1, 2, 3].iter().any(|&x| x > 10));
        assert!([1, 2, 3].iter().all(|&x| x > 0));
        assert!(![1, 2, 3].iter().all(|&x| x > 2));
    }

    #[test]
    fn test_collect_string() {
        let s: String = vec!["a", "b", "c"].into_iter().collect();
        assert_eq!(s, "abc");
    }
}
