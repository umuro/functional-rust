// 069: Unfold — generate a sequence from a seed

// Approach 1: Manual unfold function
fn unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T> {
    let mut result = Vec::new();
    let mut state = seed;
    while let Some((value, next)) = f(state) {
        result.push(value);
        state = next;
    }
    result
}

fn range(a: i32, b: i32) -> Vec<i32> {
    unfold(a, |i| if i >= b { None } else { Some((i, i + 1)) })
}

fn fibs_up_to(limit: u64) -> Vec<u64> {
    unfold((0u64, 1u64), |(a, b)| {
        if a > limit {
            None
        } else {
            Some((a, (b, a + b)))
        }
    })
}

// Approach 2: Using std::iter::successors
fn collatz(n: u64) -> Vec<u64> {
    std::iter::successors(Some(n), |&x| {
        if x <= 1 {
            None
        } else if x % 2 == 0 {
            Some(x / 2)
        } else {
            Some(3 * x + 1)
        }
    })
    .collect()
}

// Approach 3: Using from_fn with state
fn range_from_fn(a: i32, b: i32) -> Vec<i32> {
    let mut i = a;
    std::iter::from_fn(move || {
        if i >= b {
            None
        } else {
            let v = i;
            i += 1;
            Some(v)
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(range(1, 6), vec![1, 2, 3, 4, 5]);
        assert_eq!(range(5, 5), Vec::<i32>::new());
    }

    #[test]
    fn test_fibs() {
        assert_eq!(fibs_up_to(20), vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_range_from_fn() {
        assert_eq!(range_from_fn(1, 4), vec![1, 2, 3]);
    }
}
