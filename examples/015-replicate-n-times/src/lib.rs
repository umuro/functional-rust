/// Replicate Elements N Times (99 Problems #15)
///
/// Replicate every element of a list n times.
/// replicate [a; b; c] 3 → [a; a; a; b; b; b; c; c; c]

// ── Idiomatic Rust: flat_map + repeat ───────────────────────────────────────

pub fn replicate<T: Clone>(list: &[T], n: usize) -> Vec<T> {
    list.iter()
        .flat_map(|x| std::iter::repeat(x.clone()).take(n))
        .collect()
}

// ── Pre-allocated version ───────────────────────────────────────────────────

pub fn replicate_prealloc<T: Clone>(list: &[T], n: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(list.len() * n);
    for item in list {
        for _ in 0..n {
            result.push(item.clone());
        }
    }
    result
}

// ── Recursive style ─────────────────────────────────────────────────────────

pub fn replicate_recursive<T: Clone>(list: &[T], n: usize) -> Vec<T> {
    fn repeat_elem<T: Clone>(x: &T, n: usize) -> Vec<T> {
        if n == 0 { vec![] }
        else {
            let mut rest = repeat_elem(x, n - 1);
            rest.insert(0, x.clone());
            rest
        }
    }

    match list.split_first() {
        None => vec![],
        Some((head, tail)) => {
            let mut result = repeat_elem(head, n);
            result.extend(replicate_recursive(tail, n));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(replicate::<i32>(&[], 5), vec![]);
    }

    #[test]
    fn test_zero_times() {
        assert_eq!(replicate(&[1, 2, 3], 0), vec![]);
    }

    #[test]
    fn test_one_time() {
        assert_eq!(replicate(&[1, 2, 3], 1), vec![1, 2, 3]);
    }

    #[test]
    fn test_three_times() {
        assert_eq!(
            replicate(&['a', 'b', 'c'], 3),
            vec!['a','a','a','b','b','b','c','c','c']
        );
        assert_eq!(
            replicate_prealloc(&['a', 'b', 'c'], 3),
            vec!['a','a','a','b','b','b','c','c','c']
        );
        assert_eq!(
            replicate_recursive(&['a', 'b', 'c'], 3),
            vec!['a','a','a','b','b','b','c','c','c']
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(replicate(&[42], 5), vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_large() {
        let input: Vec<i32> = (0..10).collect();
        let result = replicate(&input, 100);
        assert_eq!(result.len(), 1000);
        // First 100 should all be 0
        assert!(result[..100].iter().all(|&x| x == 0));
        // Last 100 should all be 9
        assert!(result[900..].iter().all(|&x| x == 9));
    }
}
