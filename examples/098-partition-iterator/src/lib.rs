// 098: Partition Iterator

#[cfg(test)]
mod tests {
    #[test]
    fn test_partition() {
        let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=6).partition(|x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_partition_empty() {
        let (a, b): (Vec<i32>, Vec<i32>) = std::iter::empty().partition(|_: &i32| true);
        assert!(a.is_empty());
        assert!(b.is_empty());
    }

    #[test]
    fn test_partition_all_match() {
        let (yes, no): (Vec<i32>, Vec<i32>) = (1..=3).partition(|_| true);
        assert_eq!(yes, vec![1, 2, 3]);
        assert!(no.is_empty());
    }
}
