#![allow(clippy::all)]
//! # Hylomorphism
//! Unfold then fold - ana composed with cata.

pub fn hylo<S, A, B>(
    seed: S,
    next: impl Fn(S) -> Option<(A, S)>,
    nil: B,
    cons: impl Fn(A, B) -> B,
) -> B {
    let list: Vec<A> = {
        let mut result = Vec::new();
        let mut s = seed;
        while let Some((a, s2)) = next(s) {
            result.push(a);
            s = s2;
        }
        result
    };
    list.into_iter().rev().fold(nil, |acc, x| cons(x, acc))
}

pub fn factorial(n: u64) -> u64 {
    hylo(
        n,
        |k| if k == 0 { None } else { Some((k, k - 1)) },
        1,
        |x, acc| x * acc,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
    }
}
