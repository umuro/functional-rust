#![allow(clippy::all)]
//! Example 133: Variance — Covariance, Contravariance, Invariance
//!
//! Variance describes when a generic type `F<T>` can be substituted for `F<U>`
//! given a relationship between `T` and `U`.  In Rust the relationship is
//! expressed through *lifetimes*: `'long` outlives `'short`, so a value valid
//! for `'long` can safely stand in where `'short` is required.
//!
//! | Position          | Variance    | Example                      |
//! |-------------------|-------------|------------------------------|
//! | shared reference  | covariant   | `&'a T` covariant in `'a`, `T` |
//! | mutable reference | invariant   | `&'a mut T` invariant in `T`   |
//! | fn argument       | contravariant | `fn(T)` contravariant in `T` |
//!
//! `PhantomData` lets you declare the variance you need for raw-pointer or
//! zero-sized wrappers without storing an actual value.

use std::marker::PhantomData;

// ── Approach 1: Covariant wrapper (read-only producer) ────────────────────────

/// A type that can *produce* values of type `T`.
///
/// `PhantomData<T>` makes this covariant in `T`: a `Producer<Dog>` can be
/// used where a `Producer<Animal>` is expected, just like `&Dog` can be
/// used where `&Animal` is expected.
pub struct Producer<T> {
    func: fn() -> T,
    _marker: PhantomData<T>,
}

impl<T> Producer<T> {
    pub fn new(func: fn() -> T) -> Self {
        Self {
            func,
            _marker: PhantomData,
        }
    }

    pub fn produce(&self) -> T {
        (self.func)()
    }
}

// ── Approach 2: Contravariant wrapper (write-only consumer) ───────────────────

/// A type that can *consume* values of type `T`.
///
/// `PhantomData<fn(T)>` makes this contravariant in `T`: a `Consumer<Animal>`
/// can be used where a `Consumer<Dog>` is expected (anything that handles any
/// animal can certainly handle a dog).
pub struct Consumer<T> {
    func: fn(T),
    _marker: PhantomData<fn(T)>,
}

impl<T> Consumer<T> {
    pub fn new(func: fn(T)) -> Self {
        Self {
            func,
            _marker: PhantomData,
        }
    }

    pub fn consume(&self, value: T) {
        (self.func)(value)
    }
}

// ── Approach 3: Invariant wrapper (read + write cell) ─────────────────────────

/// A cell that can both read and write `T`.
///
/// `PhantomData<fn(T) -> T>` (or equivalently `*mut T`) makes this invariant
/// in `T`: neither covariant nor contravariant, because you can both produce
/// *and* consume — widening or narrowing would be unsound.
pub struct Invariant<T> {
    value: T,
    _marker: PhantomData<fn(T) -> T>,
}

impl<T> Invariant<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

// ── Approach 4: Lifetime variance with shared vs mutable references ───────────

/// Demonstrates covariance in lifetimes.
///
/// `&'long str` can be passed where `&'short str` is expected: the borrow
/// checker silently shortens the lifetime.  This is covariance in `'a`.
pub fn longest_prefix(s: &str, max_len: usize) -> &str {
    let end = s
        .char_indices()
        .map(|(i, _)| i)
        .nth(max_len)
        .unwrap_or(s.len());
    &s[..end]
}

/// Demonstrates why mutable references must be invariant in their target type.
///
/// If `&mut Vec<T>` were covariant you could push a `Cat` through a
/// `&mut Vec<Animal>` alias and corrupt a `Vec<Dog>`.  Invariance prevents
/// this.  Here we show the *safe* pattern: mutating through a correctly-typed
/// mutable reference.
pub fn append_item<T>(vec: &mut Vec<T>, item: T) {
    vec.push(item);
}

// ── Approach 5: Phantom lifetime for borrowed-handle APIs ─────────────────────

/// A handle that logically borrows data for lifetime `'a` but stores only
/// metadata (an index).  `PhantomData<&'a ()>` makes it covariant in `'a`
/// and tells the borrow checker that the handle must not outlive `'a`.
pub struct Handle<'a> {
    index: usize,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Handle<'a> {
    /// Creates a handle that conceptually borrows `data` for `'a`.
    pub fn new(_data: &'a [u8], index: usize) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── Producer (covariant) ─────────────────────────────────────────────────

    #[test]
    fn test_producer_i32() {
        let p = Producer::new(|| 42_i32);
        assert_eq!(p.produce(), 42);
    }

    #[test]
    fn test_producer_string() {
        let p = Producer::new(|| String::from("hello"));
        assert_eq!(p.produce(), "hello");
    }

    #[test]
    fn test_producer_called_multiple_times() {
        let mut count = 0_u32;
        let p = Producer::new(|| 1_u32);
        for _ in 0..5 {
            count += p.produce();
        }
        assert_eq!(count, 5);
    }

    // ── Consumer (contravariant) ─────────────────────────────────────────────

    #[test]
    fn test_consumer_runs() {
        use std::cell::Cell;
        // We can't easily capture mutable state with a fn pointer, so we use
        // a thread_local to observe the side-effect.
        thread_local! { static LAST: Cell<i32> = Cell::new(0); }
        fn record(v: i32) {
            LAST.with(|c| c.set(v));
        }
        let c = Consumer::new(record);
        c.consume(7);
        LAST.with(|cell| assert_eq!(cell.get(), 7));
    }

    #[test]
    fn test_consumer_str() {
        // Just confirm it compiles and runs for a non-Copy type.
        fn sink(_s: String) {}
        let c = Consumer::new(sink);
        c.consume(String::from("drop me"));
    }

    // ── Invariant cell ───────────────────────────────────────────────────────

    #[test]
    fn test_invariant_get_set() {
        let mut cell = Invariant::new(10_i32);
        assert_eq!(*cell.get(), 10);
        cell.set(20);
        assert_eq!(*cell.get(), 20);
    }

    #[test]
    fn test_invariant_string() {
        let mut cell = Invariant::new(String::from("first"));
        assert_eq!(cell.get().as_str(), "first");
        cell.set(String::from("second"));
        assert_eq!(cell.get().as_str(), "second");
    }

    // ── Lifetime covariance ──────────────────────────────────────────────────

    #[test]
    fn test_longest_prefix_shorter_than_string() {
        let s = "hello, world";
        assert_eq!(longest_prefix(s, 5), "hello");
    }

    #[test]
    fn test_longest_prefix_longer_than_string() {
        let s = "hi";
        assert_eq!(longest_prefix(s, 100), "hi");
    }

    #[test]
    fn test_longest_prefix_empty() {
        let s = "anything";
        assert_eq!(longest_prefix(s, 0), "");
    }

    // ── append_item (mutable reference, invariant in T) ──────────────────────

    #[test]
    fn test_append_item() {
        let mut v: Vec<i32> = vec![1, 2, 3];
        append_item(&mut v, 4);
        assert_eq!(v, [1, 2, 3, 4]);
    }

    #[test]
    fn test_append_item_string() {
        let mut v: Vec<String> = Vec::new();
        append_item(&mut v, String::from("a"));
        append_item(&mut v, String::from("b"));
        assert_eq!(v, ["a", "b"]);
    }

    // ── Handle (phantom lifetime) ─────────────────────────────────────────────

    #[test]
    fn test_handle_index() {
        let data = b"hello";
        let h = Handle::new(data, 2);
        assert_eq!(h.index(), 2);
    }

    #[test]
    fn test_handle_zero_index() {
        let data = b"rust";
        let h = Handle::new(data, 0);
        assert_eq!(h.index(), 0);
    }
}
