//! # Fixed-Point Types
//! Recursive types using fixed-point combinators.

pub enum Fix<F> { In(Box<F>) }

pub enum ListF<A, R> { Nil, Cons(A, R) }

pub type FixList<A> = Fix<ListF<A, Fix<ListF<A, ()>>>>;

pub fn fold_list<A: Clone, B>(list: &[A], init: B, f: impl Fn(B, &A) -> B) -> B {
    list.iter().fold(init, |acc, x| f(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fold() {
        let sum = fold_list(&[1, 2, 3], 0, |a, b| a + b);
        assert_eq!(sum, 6);
    }
}
