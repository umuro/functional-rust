//! 286. Creating iterators with from_fn()
//!
//! `std::iter::from_fn(f)` creates an iterator from a closure returning `Option<T>`.

fn main() {
    // Simple counter
    let mut n = 0i32;
    let counter = std::iter::from_fn(move || {
        n += 1;
        if n <= 5 { Some(n) } else { None }
    });
    let v: Vec<i32> = counter.collect();
    println!("Counter: {:?}", v);

    // Fibonacci via from_fn
    let fib = {
        let (mut a, mut b) = (0u64, 1u64);
        std::iter::from_fn(move || {
            let val = a;
            let next = a + b;
            a = b;
            b = next;
            Some(val)
        })
    };
    let first_10: Vec<u64> = fib.take(10).collect();
    println!("Fibonacci: {:?}", first_10);

    // Parse tokens from a string
    let input = "42 17 99 3 55";
    let mut words = input.split_whitespace();
    let numbers: Vec<u32> = std::iter::from_fn(|| {
        words.next().and_then(|w| w.parse().ok())
    }).collect();
    println!("Parsed: {:?}", numbers);

    // Simulate reading from a buffer
    let buffer = vec![1u8, 2, 3, 4, 5];
    let mut idx = 0;
    let reader = std::iter::from_fn(|| {
        if idx < buffer.len() {
            let v = buffer[idx];
            idx += 1;
            Some(v)
        } else {
            None
        }
    });
    let bytes: Vec<u8> = reader.collect();
    println!("Buffer: {:?}", bytes);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_fn_counter() {
        let mut n = 0i32;
        let result: Vec<i32> = std::iter::from_fn(move || {
            n += 1;
            if n <= 3 { Some(n) } else { None }
        }).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_from_fn_fibonacci_first_5() {
        let (mut a, mut b) = (0u64, 1u64);
        let result: Vec<u64> = std::iter::from_fn(|| {
            let v = a; let n = a + b; a = b; b = n; Some(v)
        }).take(5).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_from_fn_empty() {
        let result: Vec<i32> = std::iter::from_fn(|| None).collect();
        assert!(result.is_empty());
    }
}
