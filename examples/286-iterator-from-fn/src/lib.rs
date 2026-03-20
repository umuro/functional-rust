#![allow(clippy::all)]
//! # Creating Iterators with from_fn()
//!
//! `std::iter::from_fn(f)` creates an iterator from a closure returning `Option<T>`.

/// Create a simple counting iterator using from_fn
pub fn counter(max: i32) -> impl Iterator<Item = i32> {
    let mut n = 0;
    std::iter::from_fn(move || {
        n += 1;
        if n <= max {
            Some(n)
        } else {
            None
        }
    })
}

/// Create a Fibonacci iterator using from_fn
pub fn fibonacci() -> impl Iterator<Item = u64> {
    let (mut a, mut b) = (0u64, 1u64);
    std::iter::from_fn(move || {
        let val = a;
        let next = a.checked_add(b)?; // Return None on overflow
        a = b;
        b = next;
        Some(val)
    })
}

/// Parse numbers from whitespace-separated string
pub fn parse_numbers(input: &str) -> impl Iterator<Item = u32> + '_ {
    let mut words = input.split_whitespace();
    std::iter::from_fn(move || {
        loop {
            match words.next() {
                None => return None,
                Some(w) => {
                    if let Ok(n) = w.parse() {
                        return Some(n);
                    }
                    // skip invalid, continue to next word
                }
            }
        }
    })
}

/// Alternative: Create a range with custom step
pub fn stepped_range(start: i32, end: i32, step: i32) -> impl Iterator<Item = i32> {
    let mut current = start;
    std::iter::from_fn(move || {
        if (step > 0 && current < end) || (step < 0 && current > end) {
            let val = current;
            current += step;
            Some(val)
        } else {
            None
        }
    })
}

/// Create iterator from buffer simulation
pub fn buffer_reader(buffer: Vec<u8>) -> impl Iterator<Item = u8> {
    let mut idx = 0;
    std::iter::from_fn(move || {
        if idx < buffer.len() {
            let v = buffer[idx];
            idx += 1;
            Some(v)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let result: Vec<i32> = counter(5).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counter_zero() {
        let result: Vec<i32> = counter(0).collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_fibonacci_first_10() {
        let result: Vec<u64> = fibonacci().take(10).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_parse_numbers() {
        let result: Vec<u32> = parse_numbers("42 17 99 3 55").collect();
        assert_eq!(result, vec![42, 17, 99, 3, 55]);
    }

    #[test]
    fn test_parse_numbers_with_invalid() {
        let result: Vec<u32> = parse_numbers("1 foo 3 bar 5").collect();
        assert_eq!(result, vec![1, 3, 5]); // skips invalid
    }

    #[test]
    fn test_stepped_range() {
        let result: Vec<i32> = stepped_range(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_stepped_range_negative() {
        let result: Vec<i32> = stepped_range(10, 0, -3).collect();
        assert_eq!(result, vec![10, 7, 4, 1]);
    }

    #[test]
    fn test_buffer_reader() {
        let result: Vec<u8> = buffer_reader(vec![1, 2, 3, 4, 5]).collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
