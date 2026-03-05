//! Example 134: Higher-Kinded Types Simulation
//!
//! Rust lacks native HKTs (`F<_>` type-level functions), but we can simulate
//! them via *defunctionalization*: introduce a marker type (`OptionHKT`,
//! `VecHKT`) that carries the container identity, and use a GAT
//! (`type Applied<T>`) to reconstruct the concrete type at each call site.
//!
//! This lets us write genuinely generic `Functor` and `Monad` algorithms
//! that work over any container — without duplicating code.

// ---------------------------------------------------------------------------
// Approach 1: HKT trait via Generic Associated Types (GATs, stable since 1.65)
// ---------------------------------------------------------------------------

/// A marker trait whose sole purpose is to say "I am a type constructor".
/// `Applied<T>` is the concrete container type for element type `T`.
pub trait HKT {
    type Applied<T>;
}

/// Marker for `Option<_>`.
pub struct OptionHKT;
impl HKT for OptionHKT {
    type Applied<T> = Option<T>;
}

/// Marker for `Vec<_>`.
pub struct VecHKT;
impl HKT for VecHKT {
    type Applied<T> = Vec<T>;
}

/// Marker for `Result<_, E>` with a fixed error type `E`.
pub struct ResultHKT<E>(std::marker::PhantomData<E>);
impl<E> HKT for ResultHKT<E> {
    type Applied<T> = Result<T, E>;
}

// ---------------------------------------------------------------------------
// Approach 2: Functor — generic `map` over any HKT container
// ---------------------------------------------------------------------------

pub trait Functor: HKT {
    fn fmap<A, B>(fa: Self::Applied<A>, f: impl Fn(A) -> B) -> Self::Applied<B>;
}

impl Functor for OptionHKT {
    fn fmap<A, B>(fa: Option<A>, f: impl Fn(A) -> B) -> Option<B> {
        fa.map(f)
    }
}

impl Functor for VecHKT {
    fn fmap<A, B>(fa: Vec<A>, f: impl Fn(A) -> B) -> Vec<B> {
        fa.into_iter().map(f).collect()
    }
}

impl<E> Functor for ResultHKT<E> {
    fn fmap<A, B>(fa: Result<A, E>, f: impl Fn(A) -> B) -> Result<B, E> {
        fa.map(f)
    }
}

/// Generic algorithm: double every `i32` inside *any* Functor container.
/// In OCaml you could write `double_all : 'a list -> 'a list` generically;
/// here we achieve the same by parameterising over the HKT marker `F`.
pub fn double_all<F>(fa: F::Applied<i32>) -> F::Applied<i32>
where
    F: Functor,
    F::Applied<i32>: Sized,
{
    F::fmap(fa, |x| x * 2)
}

// ---------------------------------------------------------------------------
// Approach 3: Monad — `pure` + `bind` on top of Functor
// ---------------------------------------------------------------------------

pub trait Monad: Functor {
    fn pure<A>(a: A) -> Self::Applied<A>;
    fn bind<A, B>(ma: Self::Applied<A>, f: impl Fn(A) -> Self::Applied<B>) -> Self::Applied<B>;
}

impl Monad for OptionHKT {
    fn pure<A>(a: A) -> Option<A> {
        Some(a)
    }

    fn bind<A, B>(ma: Option<A>, f: impl Fn(A) -> Option<B>) -> Option<B> {
        ma.and_then(f)
    }
}

impl Monad for VecHKT {
    fn pure<A>(a: A) -> Vec<A> {
        vec![a]
    }

    fn bind<A, B>(ma: Vec<A>, f: impl Fn(A) -> Vec<B>) -> Vec<B> {
        ma.into_iter().flat_map(f).collect()
    }
}

impl<E> Monad for ResultHKT<E> {
    fn pure<A>(a: A) -> Result<A, E> {
        Ok(a)
    }

    fn bind<A, B>(ma: Result<A, E>, f: impl Fn(A) -> Result<B, E>) -> Result<B, E> {
        ma.and_then(f)
    }
}

/// Generic pipeline: apply two successive transformations inside any Monad.
/// Shows that monadic `bind` sequences computations generically — the same
/// code works for `Option`, `Vec`, and `Result` without duplication.
pub fn pipeline<M>(ma: M::Applied<i32>, f: impl Fn(i32) -> M::Applied<i32>) -> M::Applied<i32>
where
    M: Monad,
    M::Applied<i32>: Sized,
{
    M::bind(ma, f)
}

// ---------------------------------------------------------------------------
// Approach 4: Foldable — reduce any HKT container to a single value
// ---------------------------------------------------------------------------

pub trait Foldable: HKT {
    fn fold_left<A, B>(fa: Self::Applied<A>, init: B, f: impl Fn(B, A) -> B) -> B;
}

impl Foldable for OptionHKT {
    fn fold_left<A, B>(fa: Option<A>, init: B, f: impl Fn(B, A) -> B) -> B {
        match fa {
            None => init,
            Some(a) => f(init, a),
        }
    }
}

impl Foldable for VecHKT {
    fn fold_left<A, B>(fa: Vec<A>, init: B, f: impl Fn(B, A) -> B) -> B {
        fa.into_iter().fold(init, f)
    }
}

/// Generic sum over any Foldable container of `i32`s.
pub fn sum_all<F: Foldable>(fa: F::Applied<i32>) -> i32
where
    F::Applied<i32>: Sized,
{
    F::fold_left(fa, 0, |acc, x| acc + x)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Functor tests ---

    #[test]
    fn test_functor_option_some() {
        let result = OptionHKT::fmap(Some(3), |x| x * 10);
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_functor_option_none() {
        let result = OptionHKT::fmap(None::<i32>, |x| x * 10);
        assert_eq!(result, None);
    }

    #[test]
    fn test_functor_vec() {
        let result = VecHKT::fmap(vec![1, 2, 3], |x| x + 1);
        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn test_functor_vec_empty() {
        let result = VecHKT::fmap(Vec::<i32>::new(), |x| x + 1);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_functor_result_ok() {
        let result = ResultHKT::<String>::fmap(Ok(5), |x| x * 2);
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_functor_result_err() {
        let result = ResultHKT::<String>::fmap(Err("oops".to_owned()), |x: i32| x * 2);
        assert_eq!(result, Err("oops".to_owned()));
    }

    // --- double_all generic algorithm ---

    #[test]
    fn test_double_all_option() {
        assert_eq!(double_all::<OptionHKT>(Some(7)), Some(14));
        assert_eq!(double_all::<OptionHKT>(None), None);
    }

    #[test]
    fn test_double_all_vec() {
        assert_eq!(double_all::<VecHKT>(vec![1, 2, 3]), vec![2, 4, 6]);
    }

    // --- Monad tests ---

    #[test]
    fn test_monad_option_pure() {
        assert_eq!(OptionHKT::pure(42), Some(42));
    }

    #[test]
    fn test_monad_option_bind_some() {
        let result = OptionHKT::bind(Some(10), |x| if x > 5 { Some(x * 2) } else { None });
        assert_eq!(result, Some(20));
    }

    #[test]
    fn test_monad_option_bind_none_propagates() {
        let result = OptionHKT::bind(None::<i32>, |x| Some(x * 2));
        assert_eq!(result, None);
    }

    #[test]
    fn test_monad_option_bind_short_circuits() {
        let result = OptionHKT::bind(Some(1), |x| if x > 5 { Some(x) } else { None });
        assert_eq!(result, None);
    }

    #[test]
    fn test_monad_vec_bind_flat_map() {
        // VecHKT::bind == flat_map
        let result = VecHKT::bind(vec![1, 2, 3], |x| vec![x, x * 10]);
        assert_eq!(result, vec![1, 10, 2, 20, 3, 30]);
    }

    #[test]
    fn test_monad_vec_bind_empty() {
        let result = VecHKT::bind(Vec::<i32>::new(), |x| vec![x, x]);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_monad_result_bind_ok() {
        let result = ResultHKT::<String>::bind(Ok(4), |x| Ok(x + 1));
        assert_eq!(result, Ok(5));
    }

    #[test]
    fn test_monad_result_bind_err_propagates() {
        let result = ResultHKT::<String>::bind(Err("fail".to_owned()), |x: i32| Ok(x + 1));
        assert_eq!(result, Err("fail".to_owned()));
    }

    // --- Foldable tests ---

    #[test]
    fn test_foldable_option_some() {
        let result = OptionHKT::fold_left(Some(5), 0, |acc, x| acc + x);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_foldable_option_none() {
        let result = OptionHKT::fold_left(None::<i32>, 99, |acc, x| acc + x);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_foldable_vec() {
        let result = VecHKT::fold_left(vec![1, 2, 3, 4], 0, |acc, x| acc + x);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_sum_all_option() {
        assert_eq!(sum_all::<OptionHKT>(Some(42)), 42);
        assert_eq!(sum_all::<OptionHKT>(None), 0);
    }

    #[test]
    fn test_sum_all_vec() {
        assert_eq!(sum_all::<VecHKT>(vec![10, 20, 30]), 60);
    }

    // --- Composition: Functor then Foldable ---

    #[test]
    fn test_map_then_sum() {
        // double each element, then sum — via two separate generic calls
        let doubled = VecHKT::fmap(vec![1, 2, 3], |x| x * 2);
        let total = sum_all::<VecHKT>(doubled);
        assert_eq!(total, 12);
    }
}
