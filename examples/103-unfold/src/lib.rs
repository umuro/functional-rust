//! # Unfold — Generating Sequences from Seeds
//!
//! `unfold` is the dual of `fold`: instead of reducing a list to a value,
//! it builds a list from a seed value.

// ---------------------------------------------------------------------------
// Approach A: Collect into Vec (mirrors OCaml's list building)
// ---------------------------------------------------------------------------

pub fn unfold<S, T>(seed: S, f: impl Fn(S) -> Option<(T, S)>) -> Vec<T> {
    let mut result = Vec::new();
    let mut state = seed;
    loop {
        match f(state) {
            None => break,
            Some((value, next)) => {
                result.push(value);
                state = next;
            }
        }
    }
    result
}

pub fn range(a: i32, b: i32) -> Vec<i32> {
    unfold(a, |i| if i > b { None } else { Some((i, i + 1)) })
}

pub fn countdown(n: i32) -> Vec<i32> {
    unfold(n, |i| if i < 0 { None } else { Some((i, i - 1)) })
}

pub fn collatz(n: u64) -> Vec<u64> {
    unfold(n, |x| {
        if x == 0 {
            None
        } else if x == 1 {
            Some((1, 0))
        } else {
            Some((x, if x % 2 == 0 { x / 2 } else { 3 * x + 1 }))
        }
    })
}

// ---------------------------------------------------------------------------
// Approach B: Lazy unfold — returns an iterator
// ---------------------------------------------------------------------------

pub fn unfold_iter<S, T>(seed: S, f: impl Fn(&S) -> Option<(T, S)>) -> impl Iterator<Item = T> {
    let mut state = Some(seed);
    std::iter::from_fn(move || {
        let s = state.take()?;
        let (value, next) = f(&s)?;
        state = Some(next);
        Some(value)
    })
}

// ---------------------------------------------------------------------------
// Approach C: Using std::iter::successors
// ---------------------------------------------------------------------------

pub fn collatz_iter(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(n), |&x| {
        if x <= 1 {
            None
        } else if x % 2 == 0 {
            Some(x / 2)
        } else {
            Some(3 * x + 1)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(range(1, 5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_countdown() {
        assert_eq!(countdown(3), vec![3, 2, 1, 0]);
    }

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn test_empty_range() {
        assert_eq!(range(5, 3), vec![]);
    }

    #[test]
    fn test_lazy_unfold() {
        let first5: Vec<i32> = unfold_iter(0, |&i| Some((i, i + 1))).take(5).collect();
        assert_eq!(first5, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collatz_iter() {
        let seq: Vec<u64> = collatz_iter(6).collect();
        assert_eq!(seq, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }
}
