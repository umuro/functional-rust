# 742: Type Witnesses and Evidence Passing

**Difficulty:** 4  **Level:** Expert

A type witness is a value whose type *proves* an invariant holds — `Sorted<T>` guarantees the contents are sorted, `NonZeroU64` guarantees non-zero — so operations that require the invariant can accept the witness instead of re-checking.

## The Problem This Solves

Binary search requires a sorted slice. Every time you call binary search, someone has to ensure the slice is sorted — either you check it at the call site (expensive and easy to forget), you add an assertion inside the function (runtime cost on every call, checked once when invariant was established), or you trust the caller (unsafe contract, violated eventually).

The same pattern recurs constantly: divide-by-zero requires a non-zero divisor, authenticated endpoints require an authenticated session, safe indexing requires an in-bounds index. These invariants get re-checked redundantly or, worse, get asserted at the wrong level.

Type witnesses encode the proof in the type. `Sorted<T>` is a wrapper around `Vec<T>` that can only be created by actually sorting. Once you hold a `Sorted<T>`, the sorting invariant is established and any function that requires sorted input accepts `&Sorted<T>` — no re-check needed. The compiler enforces that you can't construct `Sorted<T>` any other way.

## The Intuition

A witness is a value you can only obtain by satisfying a condition. The private inner field is the enforcement mechanism — `Sorted<T>(Vec<T>)` with a private constructor means the only way to get a `Sorted<T>` is through `Sorted::sort(v)`. Functions like `merge()` return `Sorted<T>` because merging two sorted sequences produces a sorted sequence — the type propagates the proof.

This is proof-carrying code in Rust. You don't pass "is this sorted?" as a boolean — you pass the sorting proof itself as the type. A function that requires a sorted sequence makes the requirement visible in its signature: `fn binary_search(sorted: &Sorted<T>, target: &T)`.

## How It Works in Rust

```rust
// ── Sorted witness ─────────────────────────────────────────────────────────────
/// Private inner field prevents construction outside the module.
/// Only `sort()` can produce a `Sorted<T>`.
pub struct Sorted<T>(Vec<T>);  // private field — can't construct directly

impl<T: Ord + Clone> Sorted<T> {
    /// Only entry point — sorting produces the witness
    pub fn sort(mut v: Vec<T>) -> Self {
        v.sort();
        Sorted(v)  // now we KNOW it's sorted
    }

    /// Merging two sorted sequences produces a sorted sequence — type propagates
    pub fn merge(self, other: Sorted<T>) -> Sorted<T> {
        // merge-sort merge step — result is guaranteed sorted
        Sorted(/* merge impl */)
    }

    /// Binary search — safe to call because we KNOW the invariant holds
    pub fn binary_search(&self, target: &T) -> bool {
        self.0.binary_search(target).is_ok()
        // No pre-check needed — Sorted<T> IS the proof it's sorted
    }
}

// ── NonZero witness ────────────────────────────────────────────────────────────
pub struct NonZeroU64(u64);  // private field

impl NonZeroU64 {
    pub fn new(n: u64) -> Option<Self> {
        if n == 0 { None } else { Some(NonZeroU64(n)) }
    }

    /// Division never panics — the non-zero invariant is in the type
    pub fn divide(self, dividend: u64) -> u64 {
        dividend / self.0  // SAFE: self.0 ≠ 0 by construction
    }
}

// ── Authentication witness ─────────────────────────────────────────────────────
use std::marker::PhantomData;
pub struct Authenticated;
pub struct Unauthenticated;

pub struct Session<Auth> { user_id: u64, _auth: PhantomData<Auth> }

impl Session<Unauthenticated> {
    pub fn new() -> Self { Session { user_id: 0, _auth: PhantomData } }

    pub fn authenticate(self, user_id: u64, _password: &str) -> Session<Authenticated> {
        // Verification happens HERE — the Authenticated witness is the proof
        Session { user_id, _auth: PhantomData }
    }
}

impl Session<Authenticated> {
    // This fn only accepts authenticated sessions — no runtime auth check needed
    pub fn access_profile(&self) -> String { format!("Profile for {}", self.user_id) }
}

// Usage:
let data = vec![5, 2, 8, 1, 9, 3];
let sorted = Sorted::sort(data);
println!("{}", sorted.binary_search(&8)); // true — no re-sort needed

let divisor = NonZeroU64::new(5).unwrap();
println!("{}", divisor.divide(100));  // 20 — no division-by-zero check

// session.access_profile(); // COMPILE ERROR — Unauthenticated
let auth = Session::<Unauthenticated>::new().authenticate(42, "secret");
println!("{}", auth.access_profile()); // authenticated — safe
```

## What This Unlocks

- **Eliminate redundant checks** — establish the invariant once (at construction), carry the proof in the type, never re-check inside functions that receive the witness.
- **Self-documenting preconditions** — `fn binary_search(sorted: &Sorted<T>)` communicates the precondition in the type; no doc comment needed.
- **Correct-by-construction APIs** — merge sort, authenticated routes, divisor arguments — each witness carries its proof transitively through operations that preserve the invariant.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sorted witness | `module SortedList : sig type 'a t; val sort: ... end` — private constructor via signature | `Sorted<T>(Vec<T>)` with private field — only `sort()` constructs it |
| Proof-carrying types | GADT witnesses: `type _ is_sorted = IsSorted : sorted is_sorted` | Phantom type parameter or private-field newtype |
| Non-zero witness | `type nonzero = private int` — private in signature | `NonZeroU64(u64)` with private field; `new()` returns `Option` |
| Authentication witness | Abstract type in module signature | `Session<Authenticated>` vs `Session<Unauthenticated>` phantom types |
| Performance | Zero cost (GC; no runtime check after proof) | Zero cost — type erased; no runtime representation of the witness |
