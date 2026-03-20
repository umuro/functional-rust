#![allow(clippy::all)]
// Example 090: Infinite Iterators
// iterate, repeat, cycle, unfold

// === Approach 1: repeat and cycle ===
fn repeat_demo() -> Vec<i32> {
    std::iter::repeat(42).take(5).collect()
}

fn cycle_demo() -> Vec<i32> {
    [1, 2, 3].iter().copied().cycle().take(7).collect()
}

// repeat_with for computed values
fn repeat_with_demo() -> Vec<f64> {
    let mut n = 1.0;
    std::iter::repeat_with(move || {
        let v = n;
        n *= 2.0;
        v
    })
    .take(5)
    .collect()
}

// === Approach 2: successors (like OCaml iterate) ===
fn iterate<T: Clone>(init: T, f: impl Fn(&T) -> T) -> impl Iterator<Item = T> {
    std::iter::successors(Some(init), move |prev| Some(f(prev)))
}

fn doubles_from(n: u64) -> impl Iterator<Item = u64> {
    iterate(n, |x| x * 2)
}

fn add_step(step: i32) -> impl Iterator<Item = i32> {
    iterate(0, move |x| x + step)
}

// === Approach 3: from_fn (like OCaml unfold) ===
fn unfold<T, S, F>(init: S, f: F) -> impl Iterator<Item = T>
where
    F: Fn(&S) -> Option<(T, S)>,
{
    let mut state = Some(init);
    std::iter::from_fn(move || {
        let s = state.take()?;
        let (value, next) = f(&s)?;
        state = Some(next);
        Some(value)
    })
}

fn fibonacci_unfold() -> impl Iterator<Item = u64> {
    unfold((0u64, 1u64), |&(a, b)| Some((a, (b, a + b))))
}

fn digits(mut n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut result: Vec<u64> =
        unfold(n, |&n| if n == 0 { None } else { Some((n % 10, n / 10)) }).collect();
    result.reverse();
    result
}

// Combine infinite iterators
fn interleave<T>(
    a: impl Iterator<Item = T>,
    b: impl Iterator<Item = T>,
) -> impl Iterator<Item = T> {
    a.zip(b)
        .flat_map(|(x, y)| std::iter::once(x).chain(std::iter::once(y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        assert_eq!(repeat_demo(), vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_cycle() {
        assert_eq!(cycle_demo(), vec![1, 2, 3, 1, 2, 3, 1]);
    }

    #[test]
    fn test_doubles() {
        let v: Vec<u64> = doubles_from(1).take(5).collect();
        assert_eq!(v, vec![1, 2, 4, 8, 16]);
    }

    #[test]
    fn test_iterate_add() {
        let v: Vec<i32> = add_step(3).take(5).collect();
        assert_eq!(v, vec![0, 3, 6, 9, 12]);
    }

    #[test]
    fn test_fibonacci_unfold() {
        let v: Vec<u64> = fibonacci_unfold().take(8).collect();
        assert_eq!(v, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits(12345), vec![1, 2, 3, 4, 5]);
        assert_eq!(digits(0), vec![0]);
        assert_eq!(digits(9), vec![9]);
    }

    #[test]
    fn test_interleave() {
        let v: Vec<i32> = interleave(0..5, 10..15).collect();
        assert_eq!(v, vec![0, 10, 1, 11, 2, 12, 3, 13, 4, 14]);
    }

    #[test]
    fn test_successors_collatz() {
        let collatz: Vec<u64> = std::iter::successors(Some(6u64), |&n| {
            if n == 1 {
                None
            } else if n % 2 == 0 {
                Some(n / 2)
            } else {
                Some(3 * n + 1)
            }
        })
        .collect();
        assert_eq!(collatz, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }
}
