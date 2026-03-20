#![allow(clippy::all)]
//! # Paramorphism
//! Fold with access to original substructure.

pub fn para<A: Clone, B>(xs: &[A], nil: B, cons: impl Fn(&A, &[A], B) -> B) -> B {
    let mut acc = nil;
    for i in (0..xs.len()).rev() {
        acc = cons(&xs[i], &xs[i + 1..], acc);
    }
    acc
}

pub fn tails<A: Clone>(xs: &[A]) -> Vec<Vec<A>> {
    para(xs, vec![vec![]], |_, rest, mut acc| {
        acc.insert(0, rest.to_vec());
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tails() {
        let result = tails(&[1, 2, 3]);
        assert_eq!(result.len(), 4); // [1,2,3], [2,3], [3], []
    }
}
