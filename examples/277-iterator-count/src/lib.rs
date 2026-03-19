//! 277. Counting with count()
//!
//! `count()` consumes an iterator and returns the total number of elements.

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_basic() {
        assert_eq!((1..=10).count(), 10);
    }

    #[test]
    fn test_count_filter() {
        let evens = (1..=10).filter(|x| x % 2 == 0).count();
        assert_eq!(evens, 5);
    }

    #[test]
    fn test_count_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.iter().count(), 0);
    }

    #[test]
    fn test_count_string_chars() {
        let vowels = "hello".chars().filter(|c| "aeiou".contains(*c)).count();
        assert_eq!(vowels, 2);
    }
}
