#![allow(clippy::all)]
//! Example 129: Type-Level Natural Numbers — Peano Arithmetic
//!
//! Encode natural numbers as *types* using Peano arithmetic so that the length
//! of a collection becomes part of its type.  Operations like `pop` on an empty
//! list, or calling a function that requires exactly 3 elements with a
//! 2-element list, become compile errors — no runtime checks needed.

use std::marker::PhantomData;

// ── Approach 1: Peano numbers as marker types ─────────────────────────────────

/// The zero natural number at the type level.
pub struct Zero;

/// The successor of `N` — represents N+1.
pub struct Succ<N>(PhantomData<N>);

/// Reflect a type-level natural number to a runtime `usize`.
pub trait Nat {
    const VALUE: usize;
}

impl Nat for Zero {
    const VALUE: usize = 0;
}

impl<N: Nat> Nat for Succ<N> {
    const VALUE: usize = N::VALUE + 1;
}

/// Convenient type aliases.
pub type One = Succ<Zero>;
pub type Two = Succ<One>;
pub type Three = Succ<Two>;
pub type Four = Succ<Three>;
pub type Five = Succ<Four>;

// ── Approach 2: Type-level addition ──────────────────────────────────────────
//
// `Add<B>` computes `Self + B` at the type level.
// Base case:  Zero + B  = B
// Step case:  Succ<A> + B = Succ<A + B>

pub trait Add<B: Nat>: Nat {
    type Sum: Nat;
}

impl<B: Nat> Add<B> for Zero {
    type Sum = B;
}

impl<A: Nat + Add<B>, B: Nat> Add<B> for Succ<A> {
    type Sum = Succ<<A as Add<B>>::Sum>;
}

// ── Approach 3: Length-indexed vector ────────────────────────────────────────
//
// `TypeVec<T, N>` stores T values and carries the exact count N in its type.
// * `push` increments the length type: TypeVec<T,N> → TypeVec<T, Succ<N>>
// * `pop`  only exists for TypeVec<T, Succ<N>>, so popping an empty vec is a
//   *compile* error — the method simply isn't defined on TypeVec<T, Zero>.
// * `first` only exists for non-empty vecs for the same reason.

pub struct TypeVec<T, N: Nat> {
    data: Vec<T>,
    _len: PhantomData<N>,
}

/// Create an empty length-indexed vector.
pub fn empty<T>() -> TypeVec<T, Zero> {
    TypeVec {
        data: Vec::new(),
        _len: PhantomData,
    }
}

// Methods available for *any* length (including zero).
impl<T, N: Nat> TypeVec<T, N> {
    /// Returns the runtime length (always equals N::VALUE).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// True only for the empty vector.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a slice of all elements.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Push an element, returning a vector of length Succ<N>.
    pub fn push(mut self, value: T) -> TypeVec<T, Succ<N>> {
        self.data.push(value);
        TypeVec {
            data: self.data,
            _len: PhantomData,
        }
    }
}

// Methods that require at least one element — defined only for Succ<N>.
impl<T, N: Nat> TypeVec<T, Succ<N>> {
    /// Remove and return the last element, returning the shorter vector.
    pub fn pop(mut self) -> (TypeVec<T, N>, T) {
        // The type Succ<N> guarantees `data` is non-empty.
        let value = self.data.pop().expect("type guarantees non-empty");
        let shorter = TypeVec {
            data: self.data,
            _len: PhantomData,
        };
        (shorter, value)
    }

    /// Reference to the first element — safe because the vec is non-empty.
    pub fn first(&self) -> &T {
        self.data.first().expect("type guarantees non-empty")
    }

    /// Reference to the last element — safe because the vec is non-empty.
    pub fn last(&self) -> &T {
        self.data.last().expect("type guarantees non-empty")
    }
}

// ── Approach 4: Statically-sized function argument ────────────────────────────
//
// Functions can require a vec of *exactly* the right length.
// Passing a vec of the wrong size is a compile error.

/// Sum a length-3 vector of i32 values.
pub fn sum_three(v: &TypeVec<i32, Three>) -> i32 {
    v.as_slice().iter().sum()
}

/// Return the pair from a length-2 vector.
pub fn pair_to_tuple<T: Copy>(v: &TypeVec<T, Two>) -> (T, T) {
    (v.as_slice()[0], v.as_slice()[1])
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Nat trait ────────────────────────────────────────────────────────────

    #[test]
    fn test_nat_values() {
        assert_eq!(Zero::VALUE, 0);
        assert_eq!(One::VALUE, 1);
        assert_eq!(Two::VALUE, 2);
        assert_eq!(Three::VALUE, 3);
        assert_eq!(Four::VALUE, 4);
        assert_eq!(Five::VALUE, 5);
    }

    #[test]
    fn test_nat_const_eval() {
        const N: usize = Three::VALUE;
        assert_eq!(N, 3);
    }

    // ── Type-level addition ───────────────────────────────────────────────────

    #[test]
    fn test_add_zero_identity() {
        assert_eq!(<Zero as Add<Three>>::Sum::VALUE, 3);
        assert_eq!(<Three as Add<Zero>>::Sum::VALUE, 3);
    }

    #[test]
    fn test_add_non_zero() {
        assert_eq!(<Two as Add<Three>>::Sum::VALUE, 5);
        assert_eq!(<One as Add<One>>::Sum::VALUE, 2);
    }

    // ── TypeVec: empty ────────────────────────────────────────────────────────

    #[test]
    fn test_empty_vec() {
        let v: TypeVec<i32, Zero> = empty();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
        assert_eq!(v.len(), Zero::VALUE);
    }

    // ── TypeVec: push increments the type ─────────────────────────────────────

    #[test]
    fn test_push_increments_type() {
        let v0: TypeVec<i32, Zero> = empty();
        let v1: TypeVec<i32, One> = v0.push(10);
        let v2: TypeVec<i32, Two> = v1.push(20);
        let v3: TypeVec<i32, Three> = v2.push(30);

        assert_eq!(v3.len(), 3);
        assert_eq!(v3.len(), Three::VALUE);
        assert_eq!(v3.as_slice(), &[10, 20, 30]);
    }

    // ── TypeVec: pop decrements the type ──────────────────────────────────────

    #[test]
    fn test_pop_decrements_type() {
        let v3: TypeVec<i32, Three> = empty().push(1).push(2).push(3);
        let (v2, last): (TypeVec<i32, Two>, i32) = v3.pop();
        assert_eq!(last, 3);
        assert_eq!(v2.len(), 2);

        let (v1, mid): (TypeVec<i32, One>, i32) = v2.pop();
        assert_eq!(mid, 2);
        assert_eq!(v1.len(), 1);

        let (v0, first): (TypeVec<i32, Zero>, i32) = v1.pop();
        assert_eq!(first, 1);
        assert_eq!(v0.len(), 0);
        // v0.pop() would not compile — pop is not defined for TypeVec<T, Zero>.
    }

    // ── TypeVec: first / last ─────────────────────────────────────────────────

    #[test]
    fn test_first_and_last() {
        let v: TypeVec<&str, Three> = empty().push("a").push("b").push("c");
        assert_eq!(*v.first(), "a");
        assert_eq!(*v.last(), "c");
    }

    // ── Statically-sized function arguments ───────────────────────────────────

    #[test]
    fn test_sum_three() {
        let v: TypeVec<i32, Three> = empty().push(10).push(20).push(30);
        assert_eq!(sum_three(&v), 60);
    }

    #[test]
    fn test_pair_to_tuple() {
        let v: TypeVec<i32, Two> = empty().push(7).push(13);
        assert_eq!(pair_to_tuple(&v), (7, 13));
    }

    // ── Type-level length matches runtime length ──────────────────────────────

    #[test]
    fn test_type_level_equals_runtime() {
        let v4: TypeVec<u8, Four> = empty().push(0).push(1).push(2).push(3);
        assert_eq!(v4.len(), Four::VALUE);
        assert_eq!(v4.len(), 4);
    }
}
