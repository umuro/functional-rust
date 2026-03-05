//! 267. Infinite cycling with cycle()
//!
//! `cycle()` repeats a finite iterator infinitely. Requires `Clone` on the inner iterator.

#[cfg(test)]
mod tests {
    #[test]
    fn test_cycle_basic() {
        let result: Vec<i32> = [1, 2, 3].iter().copied().cycle().take(7).collect();
        assert_eq!(result, vec![1, 2, 3, 1, 2, 3, 1]);
    }

    #[test]
    fn test_cycle_zip_round_robin() {
        let items = [1i32, 2, 3, 4];
        let labels = ["a", "b"];
        let paired: Vec<_> = items.iter().zip(labels.iter().cycle()).collect();
        assert_eq!(paired.len(), 4);
        assert_eq!(*paired[2].1, "a");
    }

    #[test]
    fn test_cycle_alternating() {
        let alt: Vec<bool> = [true, false].iter().copied().cycle().take(6).collect();
        assert_eq!(alt, vec![true, false, true, false, true, false]);
    }
}
