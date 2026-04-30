//! Example 221: Apomorphism — Ana that Can Short-Circuit
//!
//! `apo` is like `ana` (anamorphism / unfold), but at each step the
//! coalgebra can return either:
//! - `Left(fix)` — inject a pre-built subtree and stop recursing
//! - `Right(seed)` — produce the next seed and keep unfolding

/// The list functor: one layer of a linked list.
#[derive(Debug, Clone)]
pub enum ListF<A> {
    NilF,
    ConsF(i64, A),
}

impl<A> ListF<A> {
    pub fn map<B>(self, f: impl Fn(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(x, f(a)),
        }
    }
}

/// The fixed point of `ListF`: a recursive linked list of `i64`.
#[derive(Debug, Clone)]
pub struct FixList(pub Box<ListF<FixList>>);

/// Construct an empty list.
pub fn nil() -> FixList {
    FixList(Box::new(ListF::NilF))
}

/// Prepend an element to a list.
pub fn cons(x: i64, xs: FixList) -> FixList {
    FixList(Box::new(ListF::ConsF(x, xs)))
}

/// Flatten a `FixList` into a `Vec<i64>`.
pub fn to_vec(fl: &FixList) -> Vec<i64> {
    let mut result = Vec::new();
    let mut cur = fl;
    loop {
        match cur.0.as_ref() {
            ListF::NilF => break,
            ListF::ConsF(x, rest) => {
                result.push(*x);
                cur = rest;
            }
        }
    }
    result
}

/// Build a `FixList` from a `Vec<i64>`.
pub fn from_vec(v: &[i64]) -> FixList {
    v.iter().rev().fold(nil(), |acc, &x| cons(x, acc))
}

/// `Left` = pre-built subtree (short-circuit); `Right` = next seed (continue).
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Apomorphism: unfold a seed into a `FixList`.
///
/// The coalgebra returns `ListF<Either<FixList, S>>`. When it yields
/// `Left(fix)` the subtree is embedded as-is; when it yields `Right(s)`
/// unfolding continues with the new seed `s`.
pub fn apo<S>(coalg: &dyn Fn(S) -> ListF<Either<FixList, S>>, seed: S) -> FixList {
    FixList(Box::new(coalg(seed).map(|either| match either {
        Either::Left(fix) => fix,
        Either::Right(s) => apo(coalg, s),
    })))
}

/// Insert `x` into a sorted `FixList`, preserving sort order.
///
/// Uses `apo` to short-circuit once the insertion point is found,
/// avoiding unnecessary traversal of the tail.
pub fn insert(x: i64, lst: FixList) -> FixList {
    apo(
        &|fl: FixList| match fl.0.as_ref() {
            ListF::NilF => ListF::ConsF(x, Either::Left(nil())),
            ListF::ConsF(y, _) if x <= *y => ListF::ConsF(x, Either::Left(fl.clone())),
            ListF::ConsF(y, rest) => ListF::ConsF(*y, Either::Right(rest.clone())),
        },
        lst,
    )
}

/// Return the first `n` elements of a `FixList`.
///
/// Short-circuits as soon as `n` reaches zero rather than continuing to
/// traverse the remainder of the list.
pub fn take(n: usize, lst: FixList) -> FixList {
    apo(
        &|(n, fl): (usize, FixList)| match (n, fl.0.as_ref()) {
            (0, _) | (_, ListF::NilF) => ListF::NilF,
            (n, ListF::ConsF(x, rest)) => ListF::ConsF(*x, Either::Right((n - 1, rest.clone()))),
        },
        (n, lst),
    )
}

/// Replace the first occurrence of `target` with `replacement` in a `FixList`.
///
/// Short-circuits after the replacement so the remaining tail is kept
/// intact without re-traversal.
pub fn replace_first(target: i64, replacement: i64, lst: FixList) -> FixList {
    apo(
        &|fl: FixList| match fl.0.as_ref() {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, rest) if *x == target => {
                ListF::ConsF(replacement, Either::Left(rest.clone()))
            }
            ListF::ConsF(x, rest) => ListF::ConsF(*x, Either::Right(rest.clone())),
        },
        lst,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted() -> FixList {
        from_vec(&[1, 3, 5])
    }
    fn five() -> FixList {
        from_vec(&[1, 2, 3, 4, 5])
    }

    #[test]
    fn test_insert_middle() {
        assert_eq!(to_vec(&insert(2, sorted())), vec![1, 2, 3, 5]);
    }
    #[test]
    fn test_insert_front() {
        assert_eq!(to_vec(&insert(0, sorted())), vec![0, 1, 3, 5]);
    }
    #[test]
    fn test_insert_back() {
        assert_eq!(to_vec(&insert(6, sorted())), vec![1, 3, 5, 6]);
    }
    #[test]
    fn test_insert_into_empty() {
        assert_eq!(to_vec(&insert(7, nil())), vec![7]);
    }
    #[test]
    fn test_insert_duplicate() {
        assert_eq!(to_vec(&insert(3, sorted())), vec![1, 3, 3, 5]);
    }
    #[test]
    fn test_take_some() {
        assert_eq!(to_vec(&take(3, five())), vec![1, 2, 3]);
    }
    #[test]
    fn test_take_zero() {
        assert_eq!(to_vec(&take(0, five())), vec![]);
    }
    #[test]
    fn test_take_more_than_length() {
        assert_eq!(to_vec(&take(10, five())), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_take_exact_length() {
        assert_eq!(to_vec(&take(5, five())), vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_take_from_empty() {
        assert_eq!(to_vec(&take(3, nil())), vec![]);
    }
    #[test]
    fn test_replace_first_found() {
        let l = from_vec(&[1, 2, 3, 2]);
        assert_eq!(to_vec(&replace_first(2, 99, l)), vec![1, 99, 3, 2]);
    }
    #[test]
    fn test_replace_first_not_found() {
        let l = from_vec(&[1, 2, 3]);
        assert_eq!(to_vec(&replace_first(9, 0, l)), vec![1, 2, 3]);
    }
    #[test]
    fn test_replace_first_at_head() {
        let l = from_vec(&[5, 1, 2]);
        assert_eq!(to_vec(&replace_first(5, 0, l)), vec![0, 1, 2]);
    }
    #[test]
    fn test_replace_first_in_empty() {
        assert_eq!(to_vec(&replace_first(1, 99, nil())), vec![]);
    }
    #[test]
    fn test_from_vec_roundtrip() {
        let v = vec![10, 20, 30];
        assert_eq!(to_vec(&from_vec(&v)), v);
    }
    #[test]
    fn test_nil_is_empty() {
        assert_eq!(to_vec(&nil()), vec![]);
    }
}
