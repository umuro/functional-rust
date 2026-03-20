#![allow(clippy::all)]
// Example 104: Mutable References (&mut T)
//
// Only ONE &mut T at a time. No &T while &mut T exists.
// This prevents data races at compile time.

// Approach 1: Mutable reference to a struct
pub struct Counter {
    pub count: i32,
}

pub fn increment(c: &mut Counter) {
    c.count += 1;
}

pub fn get_count(c: &Counter) -> i32 {
    c.count
}

// Approach 2: Mutable reference for accumulation
// Takes a mutable reference to the total — the caller owns the value,
// and we write into it without taking ownership.
pub fn sum_into(data: &[i32], total: &mut i32) {
    for &x in data {
        *total += x;
    }
}

// Approach 3: In-place mutation via &mut [T]
// Reverses a slice in place — no allocation, mutates through the reference.
pub fn reverse_in_place(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n / 2 {
        arr.swap(i, n - 1 - i);
    }
}

// Approach 4: Exclusive access — demonstrates borrow checker rules.
// We can pass &mut to a helper and regain access after it returns.
pub fn double_all(values: &mut [i32]) {
    for v in values.iter_mut() {
        *v *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let mut c = Counter { count: 0 };
        increment(&mut c);
        increment(&mut c);
        increment(&mut c);
        assert_eq!(get_count(&c), 3);
    }

    #[test]
    fn test_counter_starts_at_zero() {
        let c = Counter { count: 0 };
        assert_eq!(get_count(&c), 0);
    }

    #[test]
    fn test_sum_into_accumulates() {
        let mut total = 0;
        sum_into(&[1, 2, 3, 4, 5], &mut total);
        assert_eq!(total, 15);
    }

    #[test]
    fn test_sum_into_empty_slice() {
        let mut total = 42;
        sum_into(&[], &mut total);
        assert_eq!(total, 42);
    }

    #[test]
    fn test_sum_into_accumulates_across_calls() {
        let mut total = 0;
        sum_into(&[1, 2, 3], &mut total);
        sum_into(&[4, 5], &mut total);
        assert_eq!(total, 15);
    }

    #[test]
    fn test_reverse_in_place_even() {
        let mut arr = [1, 2, 3, 4];
        reverse_in_place(&mut arr);
        assert_eq!(arr, [4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_in_place_odd() {
        let mut arr = [1, 2, 3];
        reverse_in_place(&mut arr);
        assert_eq!(arr, [3, 2, 1]);
    }

    #[test]
    fn test_reverse_in_place_single() {
        let mut arr = [42];
        reverse_in_place(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_reverse_in_place_empty() {
        let mut arr: [i32; 0] = [];
        reverse_in_place(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_double_all() {
        let mut values = vec![1, 2, 3, 4];
        double_all(&mut values);
        assert_eq!(values, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_double_all_empty() {
        let mut values: Vec<i32> = vec![];
        double_all(&mut values);
        assert_eq!(values, vec![]);
    }
}
