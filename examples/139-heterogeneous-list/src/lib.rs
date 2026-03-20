#![allow(clippy::all)]
//! # HList — Heterogeneous List
//!
//! A type-safe list of mixed types where the full type of each element is
//! preserved at compile time. Unlike `Vec<T>` (uniform type) or
//! `Box<dyn Any>` (runtime erasure), an HList encodes every element's type
//! into its own type signature.
//!
//! `HCons<i32, HCons<&str, HCons<bool, HNil>>>` carries exactly three
//! elements of exactly those types — verified by the compiler, zero runtime cost.

// ── Core types ─────────────────────────────────────────────────────────────

/// The empty HList — the base case of the recursive type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HNil;

/// A non-empty HList: one typed head followed by the rest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HCons<H, T>(pub H, pub T);

// ── Constructor helpers ─────────────────────────────────────────────────────

/// Create an empty HList.
pub fn hnil() -> HNil {
    HNil
}

/// Prepend `head` to `tail`, building a longer HList.
pub fn hcons<H, T>(head: H, tail: T) -> HCons<H, T> {
    HCons(head, tail)
}

// ── HLength — compile-time length ──────────────────────────────────────────

/// Compute the length of an HList **at compile time**.
pub trait HLength {
    const LEN: usize;
}

impl HLength for HNil {
    const LEN: usize = 0;
}

impl<H, T: HLength> HLength for HCons<H, T> {
    const LEN: usize = 1 + T::LEN;
}

// ── Head / Tail accessors ───────────────────────────────────────────────────

/// Access the first element of a non-empty HList.
pub trait Head {
    type Output;
    fn head(&self) -> &Self::Output;
}

impl<H, T> Head for HCons<H, T> {
    type Output = H;
    fn head(&self) -> &H {
        &self.0
    }
}

/// Access everything after the first element.
pub trait Tail {
    type Output;
    fn tail(&self) -> &Self::Output;
}

impl<H, T> Tail for HCons<H, T> {
    type Output = T;
    fn tail(&self) -> &T {
        &self.1
    }
}

// ── HMap — apply a closure to every element (same output type) ──────────────

/// Apply a function to each element of an HList, collecting results into a `Vec<R>`.
///
/// Because a plain closure can't be polymorphic in Rust stable, we require all
/// elements to share a common trait bound (`Display`, `Into<i64>`, …).
/// The trait `HMap<R>` is parameterised on the result type.
pub trait HMapCollect<R> {
    fn map_collect<F: Fn(&dyn std::fmt::Debug) -> R>(&self, f: &F) -> Vec<R>;
}

impl<R> HMapCollect<R> for HNil {
    fn map_collect<F: Fn(&dyn std::fmt::Debug) -> R>(&self, _f: &F) -> Vec<R> {
        Vec::new()
    }
}

impl<H: std::fmt::Debug, T: HMapCollect<R>, R> HMapCollect<R> for HCons<H, T> {
    fn map_collect<F: Fn(&dyn std::fmt::Debug) -> R>(&self, f: &F) -> Vec<R> {
        let mut out = vec![f(&self.0)];
        out.extend(self.1.map_collect(f));
        out
    }
}

// ── HFold — fold over an HList (same element trait bound) ──────────────────

/// Fold an HList whose elements all implement `Into<i64>` into a single `i64`.
/// This mirrors OCaml's `List.fold_left`.
pub trait HFoldI64 {
    fn fold_i64(&self, init: i64, f: &dyn Fn(i64, i64) -> i64) -> i64;
}

impl HFoldI64 for HNil {
    fn fold_i64(&self, init: i64, _f: &dyn Fn(i64, i64) -> i64) -> i64 {
        init
    }
}

impl<H: Into<i64> + Copy, T: HFoldI64> HFoldI64 for HCons<H, T> {
    fn fold_i64(&self, init: i64, f: &dyn Fn(i64, i64) -> i64) -> i64 {
        let acc = f(init, self.0.into());
        self.1.fold_i64(acc, f)
    }
}

// ── Macro for ergonomic construction ───────────────────────────────────────

/// Build an HList without chaining `hcons(…, hcons(…, hnil()))` manually.
///
/// ```
/// # use heterogeneous_list::*;
/// let list = hlist![1_i32, "hello", true];
/// assert_eq!(list.head(), &1_i32);
/// assert_eq!(list.tail().head(), &"hello");
/// ```
#[macro_export]
macro_rules! hlist {
    () => { $crate::hnil() };
    ($head:expr $(, $tail:expr)* $(,)?) => {
        $crate::hcons($head, $crate::hlist![$($tail),*])
    };
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── length ──────────────────────────────────────────────────────────────

    #[test]
    fn test_hnil_length_is_zero() {
        assert_eq!(HNil::LEN, 0);
    }

    #[test]
    fn test_hcons_length_counts_all_elements() {
        type L = HCons<i32, HCons<&'static str, HCons<bool, HNil>>>;
        assert_eq!(L::LEN, 3);
    }

    // ── head / tail ─────────────────────────────────────────────────────────

    #[test]
    fn test_head_returns_first_element() {
        let list = hcons(42_i32, hcons("hello", hnil()));
        assert_eq!(list.head(), &42_i32);
    }

    #[test]
    fn test_tail_head_returns_second_element() {
        let list = hcons(42_i32, hcons("hello", hnil()));
        assert_eq!(list.tail().head(), &"hello");
    }

    #[test]
    fn test_nested_tail_access() {
        let list = hcons(1_u8, hcons(2_u16, hcons(3_u32, hnil())));
        assert_eq!(list.tail().tail().head(), &3_u32);
    }

    // ── macro ───────────────────────────────────────────────────────────────

    #[test]
    fn test_macro_builds_correct_hlist() {
        let list = hlist![10_i32, "world", false];
        assert_eq!(list.head(), &10_i32);
        assert_eq!(list.tail().head(), &"world");
        assert_eq!(list.tail().tail().head(), &false);
    }

    #[test]
    fn test_macro_empty_is_hnil() {
        let _: HNil = hlist![];
    }

    // ── map_collect ─────────────────────────────────────────────────────────

    #[test]
    fn test_map_collect_formats_debug_strings() {
        let list = hlist![1_i32, "hi", true];
        let strings = list.map_collect(&|v| format!("{v:?}"));
        assert_eq!(strings, vec!["1", "\"hi\"", "true"]);
    }

    #[test]
    fn test_map_collect_on_hnil_is_empty_vec() {
        let strings: Vec<String> = HNil.map_collect(&|v| format!("{v:?}"));
        assert!(strings.is_empty());
    }

    // ── fold ────────────────────────────────────────────────────────────────

    #[test]
    fn test_fold_sums_i64_list() {
        let list: HCons<i32, HCons<i32, HCons<i32, HNil>>> = hlist![1_i32, 2_i32, 3_i32];
        let sum = list.fold_i64(0, &|acc, x| acc + x);
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_fold_empty_returns_init() {
        let sum = HNil.fold_i64(42, &|acc, x| acc + x);
        assert_eq!(sum, 42);
    }

    // ── equality ────────────────────────────────────────────────────────────

    #[test]
    fn test_hlists_with_same_values_are_equal() {
        let a = hlist![1_i32, "x"];
        let b = hlist![1_i32, "x"];
        assert_eq!(a, b);
    }

    #[test]
    fn test_hlists_with_different_values_are_not_equal() {
        let a = hlist![1_i32, "x"];
        let b = hlist![2_i32, "x"];
        assert_ne!(a, b);
    }

    // ── type-level ──────────────────────────────────────────────────────────

    /// Confirms that the compiler accepts two HLists with different type
    /// signatures as genuinely distinct types (would not compile if confused).
    #[test]
    fn test_type_distinctness() {
        let int_str: HCons<i32, HCons<&str, HNil>> = hcons(1, hcons("a", hnil()));
        let str_int: HCons<&str, HCons<i32, HNil>> = hcons("a", hcons(1, hnil()));
        // Different types — can't compare directly, but both exist independently.
        assert_eq!(int_str.head(), &1_i32);
        assert_eq!(str_int.head(), &"a");
    }
}
