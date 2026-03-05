// Example 092: Scan / Accumulate
// Like fold but emits every intermediate value — running sums, cumulative statistics, balance histories.

// === Approach 1: Basic scan using std iterator adapter ===

/// Running sum: each element is the cumulative sum up to that index.
pub fn running_sum(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}

/// Running product: cumulative product at each step.
pub fn running_product(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(1, |acc, &x| {
            *acc *= x;
            Some(*acc)
        })
        .collect()
}

// === Approach 2: Running max / min ===

/// Running max: the highest value seen so far at each position.
pub fn running_max(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(i32::MIN, |max, &x| {
            *max = (*max).max(x);
            Some(*max)
        })
        .collect()
}

/// Running min: the lowest value seen so far at each position.
pub fn running_min(data: &[i32]) -> Vec<i32> {
    data.iter()
        .scan(i32::MAX, |min, &x| {
            *min = (*min).min(x);
            Some(*min)
        })
        .collect()
}

// === Approach 3: Practical application — balance history ===

/// Given a starting balance and a sequence of signed transactions,
/// return the balance after each transaction.
pub fn balance_history(initial: i32, transactions: &[i32]) -> Vec<i32> {
    transactions
        .iter()
        .scan(initial, |bal, &tx| {
            *bal += tx;
            Some(*bal)
        })
        .collect()
}

/// Running average (as f64) at each position.
pub fn running_average(data: &[f64]) -> Vec<f64> {
    data.iter()
        .scan((0.0_f64, 0_usize), |state, &x| {
            state.0 += x;
            state.1 += 1;
            Some(state.0 / state.1 as f64)
        })
        .collect()
}

// === Approach 4: Recursive / functional style mirroring OCaml ===

/// Generic scan: applies `f` to running state and each element,
/// collecting each intermediate state (including `init`).
pub fn scan<S, T, F>(init: S, data: &[T], f: F) -> Vec<S>
where
    S: Clone,
    F: Fn(S, &T) -> S,
{
    let mut result = Vec::with_capacity(data.len() + 1);
    result.push(init.clone());
    let mut state = init;
    for item in data {
        state = f(state, item);
        result.push(state.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum_empty() {
        assert_eq!(running_sum(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_running_sum_single() {
        assert_eq!(running_sum(&[5]), vec![5]);
    }

    #[test]
    fn test_running_sum_multiple() {
        assert_eq!(running_sum(&[1, 2, 3, 4, 5]), vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_running_sum_negative() {
        assert_eq!(running_sum(&[10, -3, 2, -1]), vec![10, 7, 9, 8]);
    }

    #[test]
    fn test_running_product() {
        assert_eq!(running_product(&[1, 2, 3, 4]), vec![1, 2, 6, 24]);
    }

    #[test]
    fn test_running_max_empty() {
        assert_eq!(running_max(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_running_max() {
        assert_eq!(
            running_max(&[3, 1, 4, 1, 5, 9, 2, 6]),
            vec![3, 3, 4, 4, 5, 9, 9, 9]
        );
    }

    #[test]
    fn test_running_min() {
        assert_eq!(running_min(&[5, 3, 8, 1, 7]), vec![5, 3, 3, 1, 1]);
    }

    #[test]
    fn test_balance_history() {
        // start at 100, transactions: +50, -30, +20, -10
        assert_eq!(
            balance_history(100, &[50, -30, 20, -10]),
            vec![150, 120, 140, 130]
        );
    }

    #[test]
    fn test_balance_history_empty() {
        assert_eq!(balance_history(500, &[]), Vec::<i32>::new());
    }

    #[test]
    fn test_running_average() {
        let result = running_average(&[1.0, 2.0, 3.0, 4.0]);
        let expected = vec![1.0, 1.5, 2.0, 2.5];
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_generic_scan_includes_init() {
        // scan with addition: should include initial value 0
        let result = scan(0, &[1, 2, 3], |acc, &x| acc + x);
        assert_eq!(result, vec![0, 1, 3, 6]);
    }

    #[test]
    fn test_generic_scan_empty() {
        let result = scan(42, &[] as &[i32], |acc, &x| acc + x);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_generic_scan_string_concat() {
        let words = ["hello", "world", "rust"];
        let result = scan(String::new(), &words, |mut acc, w| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(w);
            acc
        });
        assert_eq!(result, vec!["", "hello", "hello world", "hello world rust"]);
    }
}
