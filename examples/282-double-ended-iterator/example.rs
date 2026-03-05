//! 282. DoubleEndedIterator and rev()
//!
//! `DoubleEndedIterator` enables traversal from both ends; `rev()` swaps the direction.

struct Counter {
    front: i32,
    back: i32,
}

impl Counter {
    fn new(n: i32) -> Self { Counter { front: 1, back: n } }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let v = self.front;
        self.front += 1;
        Some(v)
    }
}

impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let v = self.back;
        self.back -= 1;
        Some(v)
    }
}

fn main() {
    // Basic rev() on a range
    let reversed: Vec<i32> = (1..=5).rev().collect();
    println!("Reversed range: {:?}", reversed);

    // rev() on a slice
    let words = ["alpha", "beta", "gamma", "delta"];
    let rev_words: Vec<_> = words.iter().rev().collect();
    println!("Reversed words: {:?}", rev_words);

    // Custom DoubleEndedIterator
    let c: Vec<i32> = Counter::new(5).collect();
    println!("Counter: {:?}", c);

    let c_rev: Vec<i32> = Counter::new(5).rev().collect();
    println!("Counter reversed: {:?}", c_rev);

    // Consume from both ends simultaneously
    let mut counter = Counter::new(5);
    print!("Both ends: ");
    loop {
        match (counter.next(), counter.next_back()) {
            (Some(f), Some(b)) => print!("({},{}) ", f, b),
            (Some(f), None)    => print!("({}) ", f),
            (None, Some(b))    => print!("({}) ", b),
            (None, None)       => break,
        }
    }
    println!();

    // rev + other adapters
    let last_3_even: Vec<i32> = (1..=20)
        .filter(|x| x % 2 == 0)
        .rev()
        .take(3)
        .collect();
    println!("Last 3 evens (reversed): {:?}", last_3_even);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_range() {
        let result: Vec<i32> = (1..=5).rev().collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_custom_dei_rev() {
        let result: Vec<i32> = Counter::new(5).rev().collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_next_back() {
        let mut c = Counter::new(5);
        assert_eq!(c.next_back(), Some(5));
        assert_eq!(c.next_back(), Some(4));
        assert_eq!(c.next(), Some(1));
    }

    #[test]
    fn test_rev_collect_string() {
        let result: String = "hello".chars().rev().collect();
        assert_eq!(result, "olleh");
    }
}
