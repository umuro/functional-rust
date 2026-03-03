/// List Operations: fundamental building blocks in functional programming.
///
/// In OCaml, lists are the bread and butter — singly-linked, immutable, with
/// pattern matching driving recursion. In Rust, Vec is the workhorse, but we
/// can still express recursive/functional patterns using slices and iterators.

// ── Idiomatic Rust: Iterator-based ──────────────────────────────────────────

/// Length using iterator (idiomatic Rust just uses `.len()`, but here we show fold)
pub fn length<T>(list: &[T]) -> usize {
    list.iter().fold(0, |acc, _| acc + 1)
}

/// Sum using iterators
pub fn sum(list: &[i64]) -> i64 {
    list.iter().sum()
}

/// Append two slices into a new Vec
pub fn append<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().chain(b.iter()).cloned().collect()
}

/// Reverse using iterators
pub fn reverse<T: Clone>(list: &[T]) -> Vec<T> {
    list.iter().rev().cloned().collect()
}

/// Map: apply a function to each element
pub fn map<T, U>(list: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    list.iter().map(f).collect()
}

/// Filter: keep elements satisfying a predicate
pub fn filter<T: Clone>(list: &[T], pred: impl Fn(&T) -> bool) -> Vec<T> {
    list.iter().filter(|x| pred(x)).cloned().collect()
}

// ── Functional/Recursive style (closer to OCaml) ───────────────────────────

/// Recursive length using slice pattern matching.
/// Note: Rust slices don't have head::tail destructuring like OCaml,
/// so we use `split_first()` or index-based patterns.
pub fn length_recursive<T>(list: &[T]) -> usize {
    match list.split_first() {
        None => 0,
        Some((_, tail)) => 1 + length_recursive(tail),
    }
}

/// Recursive sum
pub fn sum_recursive(list: &[i64]) -> i64 {
    match list.split_first() {
        None => 0,
        Some((&head, tail)) => head + sum_recursive(tail),
    }
}

/// Recursive append — builds result by consing head onto recursive call.
/// In Rust, we must allocate a new Vec each time (no persistent list sharing).
pub fn append_recursive<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    match a.split_first() {
        None => b.to_vec(),
        Some((head, tail)) => {
            let mut result = vec![head.clone()];
            result.extend(append_recursive(tail, b));
            result
        }
    }
}

// ── Tail-recursive style ───────────────────────────────────────────────────

/// Tail-recursive length with accumulator (mirrors OCaml's aux pattern)
pub fn length_tail_recursive<T>(list: &[T]) -> usize {
    fn aux<T>(acc: usize, list: &[T]) -> usize {
        match list.split_first() {
            None => acc,
            Some((_, tail)) => aux(acc + 1, tail),
        }
    }
    aux(0, list)
}

/// Tail-recursive sum
pub fn sum_tail_recursive(list: &[i64]) -> i64 {
    fn aux(acc: i64, list: &[i64]) -> i64 {
        match list.split_first() {
            None => acc,
            Some((&head, tail)) => aux(acc + head, tail),
        }
    }
    aux(0, list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_empty() {
        assert_eq!(length::<i32>(&[]), 0);
        assert_eq!(length_recursive::<i32>(&[]), 0);
        assert_eq!(length_tail_recursive::<i32>(&[]), 0);
    }

    #[test]
    fn test_length_nonempty() {
        assert_eq!(length(&[1, 2, 3]), 3);
        assert_eq!(length_recursive(&[1, 2, 3]), 3);
        assert_eq!(length_tail_recursive(&[1, 2, 3]), 3);
    }

    #[test]
    fn test_sum_variants() {
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_recursive(&[10, -5, 3]), 8);
        assert_eq!(sum_tail_recursive(&[100]), 100);
    }

    #[test]
    fn test_append() {
        assert_eq!(append(&[1, 2], &[3, 4]), vec![1, 2, 3, 4]);
        assert_eq!(append::<i32>(&[], &[1]), vec![1]);
        assert_eq!(append_recursive(&[1], &[2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(reverse(&[1, 2, 3]), vec![3, 2, 1]);
        assert_eq!(reverse(&[42]), vec![42]);
    }

    #[test]
    fn test_map_and_filter() {
        assert_eq!(map(&[1, 2, 3], |x| x * 2), vec![2, 4, 6]);
        assert_eq!(filter(&[1, 2, 3, 4, 5], |x| x % 2 == 0), vec![2, 4]);
        assert_eq!(map::<i32, i32>(&[], |x| x + 1), Vec::<i32>::new());
    }

    #[test]
    fn test_large_list() {
        let big: Vec<i64> = (1..=1000).collect();
        assert_eq!(sum(&big), 500500);
        assert_eq!(length(&big), 1000);
    }
}

fn main() {
    println!("{:?}", length::<i32>(&[]), 0);
    println!("{:?}", length_recursive::<i32>(&[]), 0);
    println!("{:?}", length_tail_recursive::<i32>(&[]), 0);
}
