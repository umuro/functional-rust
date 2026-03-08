//! 277. Counting with count()
//!
//! `count()` consumes an iterator and returns the total number of elements.

fn main() {
    let nums: Vec<i32> = (1..=10).collect();
    println!("Count: {}", nums.iter().count());

    let even_count = nums.iter().filter(|&&x| x % 2 == 0).count();
    println!("Even count: {}", even_count);

    let s = "hello world";
    let vowels = s.chars().filter(|c| "aeiou".contains(*c)).count();
    println!("Vowels in '{}': {}", s, vowels);

    let text = "the quick brown fox jumps over the lazy dog";
    println!("Word count: {}", text.split_whitespace().count());

    // Efficient count for Range (ExactSizeIterator — O(1))
    let range_count = (0usize..1_000_000).count();
    println!("Range count: {}", range_count);

    let sorted = [1i32, 3, 5, 7, 9, 11, 13];
    let under_10 = sorted.iter().take_while(|&&x| x < 10).count();
    println!("Elements < 10: {}", under_10);
}

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
