/// # Unfold — Generating Sequences from Seeds
///
/// Unfold is the dual of fold: fold consumes a list into a value,
/// unfold produces a list from a seed value.

/// Generic unfold: applies f to seed repeatedly until f returns None.
/// Returns a Vec of produced values.
pub fn unfold<T, S, F>(seed: S, f: F) -> Vec<T>
where
    F: Fn(S) -> Option<(T, S)>,
    S: Clone,
{
    let mut result = Vec::new();
    let mut state = seed;
    while let Some((value, next)) = f(state.clone()) {
        result.push(value);
        state = next;
    }
    result
}

/// Range using unfold
pub fn range(a: i64, b: i64) -> Vec<i64> {
    unfold(a, |i| if i > b { None } else { Some((i, i + 1)) })
}

/// Countdown using unfold
pub fn countdown(n: i64) -> Vec<i64> {
    unfold(n, |i| if i < 0 { None } else { Some((i, i - 1)) })
}

/// Collatz sequence using unfold
pub fn collatz(n: u64) -> Vec<u64> {
    unfold(n, |x| {
        if x == 0 {
            None
        } else if x == 1 {
            Some((1, 0))
        } else if x % 2 == 0 {
            Some((x, x / 2))
        } else {
            Some((x, 3 * x + 1))
        }
    })
}

/// Iterator-based unfold using `std::iter::successors`
pub fn fibs_iter() -> impl Iterator<Item = u64> {
    std::iter::successors(Some((0u64, 1u64)), |&(a, b)| Some((b, a + b)))
        .map(|(a, _)| a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(range(1, 5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_range_empty() {
        assert_eq!(range(5, 3), vec![]);
    }

    #[test]
    fn test_countdown() {
        assert_eq!(countdown(5), vec![5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_collatz_one() {
        assert_eq!(collatz(1), vec![1]);
    }

    #[test]
    fn test_fibs_iter() {
        let first8: Vec<u64> = fibs_iter().take(8).collect();
        assert_eq!(first8, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }
}
