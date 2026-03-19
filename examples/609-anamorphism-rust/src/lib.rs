//! # Anamorphism (Unfold)
//! Generalized unfold to build recursive structures.

pub fn ana<S, A>(seed: S, next: impl Fn(S) -> Option<(A, S)>) -> Vec<A> {
    let mut result = Vec::new();
    let mut s = seed;
    while let Some((a, s2)) = next(s) {
        result.push(a);
        s = s2;
    }
    result
}

pub fn range(start: i32, end: i32) -> Vec<i32> {
    ana(start, |n| if n < end { Some((n, n + 1)) } else { None })
}

pub fn iterate<A: Clone>(init: A, f: impl Fn(&A) -> A, n: usize) -> Vec<A> {
    ana((init, n), |(a, count)| {
        if count > 0 {
            Some((a.clone(), (f(&a), count - 1)))
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_range() {
        assert_eq!(range(0, 5), vec![0, 1, 2, 3, 4]);
    }
    #[test]
    fn test_iterate() {
        assert_eq!(iterate(1, |x| x * 2, 4), vec![1, 2, 4, 8]);
    }
}
