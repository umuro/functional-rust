/// # Scan Left — Running Accumulation
///
/// `scan` returns all intermediate results of a fold operation.
/// Like fold, but keeps the running state at each step.

/// Custom scan_left: returns vec of all intermediate accumulator values.
pub fn scan_left<T, A, F>(init: A, items: &[T], f: F) -> Vec<A>
where
    A: Clone,
    F: Fn(&A, &T) -> A,
{
    let mut result = vec![init.clone()];
    let mut acc = init;
    for item in items {
        acc = f(&acc, item);
        result.push(acc.clone());
    }
    result
}

/// Running sum
pub fn running_sum(nums: &[i64]) -> Vec<i64> {
    scan_left(0i64, nums, |acc, x| acc + x)
}

/// Running max
pub fn running_max(nums: &[i64]) -> Vec<i64> {
    scan_left(i64::MIN, nums, |acc, x| *acc.max(x))
}

/// Idiomatic Rust: use the built-in `scan` iterator adapter.
/// Note: `Iterator::scan` is slightly different — it takes FnMut with mutable state.
pub fn running_sum_idiomatic(nums: &[i64]) -> Vec<i64> {
    let mut result = vec![0i64];
    result.extend(nums.iter().scan(0i64, |state, &x| {
        *state += x;
        Some(*state)
    }));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4, 5]), vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_running_sum_empty() {
        assert_eq!(running_sum(&[]), vec![0]);
    }

    #[test]
    fn test_running_max() {
        let result = running_max(&[3, 1, 4, 1, 5, 9, 2, 6]);
        assert_eq!(result[1..], [3, 3, 4, 4, 5, 9, 9, 9]);
    }

    #[test]
    fn test_scan_left_generic() {
        let result = scan_left(String::new(), &["hello", " ", "world"], |acc, s| {
            format!("{}{}", acc, s)
        });
        assert_eq!(result, vec!["", "hello", "hello ", "hello world"]);
    }

    #[test]
    fn test_idiomatic() {
        assert_eq!(
            running_sum_idiomatic(&[1, 2, 3, 4, 5]),
            vec![0, 1, 3, 6, 10, 15]
        );
    }
}
