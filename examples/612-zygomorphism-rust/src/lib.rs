#![allow(clippy::all)]
//! # Zygomorphism
//! Two folds running in parallel.

pub fn zygo<A, B, C>(
    xs: &[A],
    nil_b: B,
    cons_b: impl Fn(&A, &B) -> B,
    nil_c: C,
    cons_c: impl Fn(&A, &B, C) -> C,
) -> C
where
    B: Clone,
{
    let mut bs: Vec<B> = vec![nil_b.clone()];
    for x in xs.iter().rev() {
        bs.push(cons_b(x, bs.last().unwrap()));
    }
    bs.reverse();
    let mut c = nil_c;
    for (i, x) in xs.iter().enumerate().rev() {
        c = cons_c(x, &bs[i + 1], c);
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zygo() {
        let sum = zygo(&[1, 2, 3], 0, |x, b| x + b, 0, |_, _, c| c + 1);
        assert_eq!(sum, 3);
    }
}
