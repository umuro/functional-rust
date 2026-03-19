//! # Histomorphism
//! Fold with access to all previous results.

pub fn histo<A, B: Clone>(xs: &[A], nil: B, cons: impl Fn(&A, &[B]) -> B) -> B {
    let mut history = vec![nil];
    for x in xs.iter().rev() {
        let next = cons(x, &history);
        history.push(next);
    }
    history.pop().unwrap()
}

pub fn fib_histo(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let xs: Vec<()> = vec![(); n];
    histo(&xs, 0u64, |_, hist| {
        if hist.len() < 2 {
            1
        } else {
            hist[hist.len() - 1] + hist[hist.len() - 2]
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fib() {
        assert_eq!(fib_histo(10), 55);
    }
}
