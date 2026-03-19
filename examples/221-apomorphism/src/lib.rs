#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
// Example 221: Apomorphism — Ana that Can Short-Circuit

#[derive(Debug, Clone)]
enum ListF<A> {
    NilF,
    ConsF(i64, A),
}

impl<A> ListF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(x, f(a)),
        }
    }
}

#[derive(Debug, Clone)]
struct FixList(Box<ListF<FixList>>);

fn nil() -> FixList {
    FixList(Box::new(ListF::NilF))
}
fn cons(x: i64, xs: FixList) -> FixList {
    FixList(Box::new(ListF::ConsF(x, xs)))
}

fn to_vec(fl: &FixList) -> Vec<i64> {
    let mut result = vec![];
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

// Either: Left = pre-built Fix (stop), Right = seed (continue)
enum Either<L, R> {
    Left(L),
    Right(R),
}

// apo: coalgebra returns F<Either<Fix, Seed>>
fn apo<S>(coalg: &dyn Fn(S) -> ListF<Either<FixList, S>>, seed: S) -> FixList {
    FixList(Box::new(coalg(seed).map(|either| match either {
        Either::Left(fix) => fix,          // short-circuit
        Either::Right(s) => apo(coalg, s), // continue
    })))
}

// Approach 1: Insert into sorted list
fn insert(x: i64, lst: FixList) -> FixList {
    apo(
        &|fl: FixList| match fl.0.as_ref() {
            ListF::NilF => ListF::ConsF(x, Either::Left(nil())),
            ListF::ConsF(y, rest) => {
                if x <= *y {
                    ListF::ConsF(x, Either::Left(fl.clone()))
                } else {
                    ListF::ConsF(*y, Either::Right(rest.clone()))
                }
            }
        },
        lst,
    )
}

// Approach 2: Take n elements
fn take(n: usize, lst: FixList) -> FixList {
    apo(
        &|s: (usize, FixList)| {
            let (n, fl) = s;
            if n == 0 {
                return ListF::NilF;
            }
            match fl.0.as_ref() {
                ListF::NilF => ListF::NilF,
                ListF::ConsF(x, rest) => ListF::ConsF(*x, Either::Right((n - 1, rest.clone()))),
            }
        },
        (n, lst),
    )
}

// Approach 3: Replace first occurrence
fn replace_first(target: i64, replacement: i64, lst: FixList) -> FixList {
    apo(
        &|fl: FixList| match fl.0.as_ref() {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, rest) => {
                if *x == target {
                    ListF::ConsF(replacement, Either::Left(rest.clone()))
                } else {
                    ListF::ConsF(*x, Either::Right(rest.clone()))
                }
            }
        },
        lst,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_middle() {
        let l = cons(1, cons(3, nil()));
        assert_eq!(to_vec(&insert(2, l)), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_empty() {
        assert_eq!(to_vec(&take(5, nil())), vec![]);
    }

    #[test]
    fn test_replace_not_found() {
        let l = cons(1, cons(2, nil()));
        assert_eq!(to_vec(&replace_first(99, 0, l)), vec![1, 2]);
    }
}
