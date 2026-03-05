//! 281. Implementing Iterator trait from scratch
//!
//! Only `next()` is required — the rest of the iterator API comes for free.

/// A counter that yields squares up to a maximum
struct Squares {
    current: u32,
    max: u32,
}

impl Squares {
    fn new(max: u32) -> Self {
        Squares { current: 0, max }
    }
}

impl Iterator for Squares {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }
        let val = self.current * self.current;
        self.current += 1;
        Some(val)
    }
}

/// Fibonacci sequence iterator
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val) // infinite — always Some
    }
}

fn main() {
    // Squares iterator — uses all iterator adapters for free
    let squares: Vec<u32> = Squares::new(6).collect();
    println!("Squares: {:?}", squares);

    let sum_of_squares: u32 = Squares::new(6).sum();
    println!("Sum of squares: {}", sum_of_squares);

    let big_squares: Vec<u32> = Squares::new(10)
        .filter(|&x| x > 10)
        .collect();
    println!("Squares > 10: {:?}", big_squares);

    // Fibonacci — infinite iterator, use take()
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("First 10 Fibonacci: {:?}", fibs);

    let fib_sum: u64 = Fibonacci::new().take(10).sum();
    println!("Sum of first 10 Fibonacci: {}", fib_sum);

    // Zip two custom iterators
    let zipped: Vec<(u32, u64)> = Squares::new(5)
        .zip(Fibonacci::new())
        .collect();
    println!("Squares zip Fibonacci: {:?}", zipped);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares_collect() {
        let result: Vec<u32> = Squares::new(5).collect();
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_squares_sum() {
        let sum: u32 = Squares::new(4).sum();
        assert_eq!(sum, 0 + 1 + 4 + 9);
    }

    #[test]
    fn test_fibonacci_first_10() {
        let result: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_filter() {
        let even_fibs: Vec<u64> = Fibonacci::new()
            .take(10)
            .filter(|x| x % 2 == 0)
            .collect();
        assert_eq!(even_fibs, vec![0, 2, 8, 34]);
    }
}
