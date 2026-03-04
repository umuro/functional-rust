# 150: Coherence Rules — One Implementation Per Type

**Difficulty:** 4  **Level:** Advanced

Understand why Rust allows only one trait implementation per type — and how the newtype pattern works around it.

## The Problem This Solves

What if two crates both define `impl Display for Vec<i32>`? When you use both crates, the compiler doesn't know which `display` to call. This is the *incoherence problem* — and it causes subtle, hard-to-debug bugs in languages that allow it (Haskell orphan instances, Python multiple inheritance conflicts).

Rust prevents this with the *orphan rule*: you can only implement a trait for a type if either the **trait** or the **type** is defined in your crate. One impl per type per trait, globally. The compiler enforces this.

The practical consequence: you sometimes want two different `Semigroup` implementations for `i32` — one for addition, one for multiplication. You can't do both as `impl Semigroup for i32`. The solution: newtype wrappers.

## The Intuition

Wrap the type you want multiple instances for in distinct named structs. `Sum(i64)` and `Product(i64)` are different types — each can have its own `Semigroup` impl. When you want "sum semantics", wrap in `Sum`. When you want "product semantics", wrap in `Product`. Unwrap when done.

This is Haskell's `newtype` in practice, and it's idiomatic Rust. The standard library uses it: `std::cmp::Reverse<T>` wraps any ordered type to reverse its comparison order.

The newtype also solves the foreign-foreign case: if both the trait and the type come from other crates, you can't impl directly — but you can wrap the type in your local newtype and impl the trait for that.

## How It Works in Rust

```rust
pub trait Semigroup {
    fn append(self, other: Self) -> Self;
}

// Can't write two different impls for i64 directly —
// instead, use newtypes to select the instance:

pub struct Sum(pub i64);       // "addition" semantics
pub struct Product(pub i64);   // "multiplication" semantics

impl Semigroup for Sum {
    fn append(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

impl Semigroup for Product {
    fn append(self, other: Self) -> Self { Product(self.0 * other.0) }
}
```

Generic fold using `Monoid` (identity element):

```rust
pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

pub fn fold<M: Monoid>(iter: impl IntoIterator<Item = M>) -> M {
    iter.into_iter().fold(M::empty(), |acc, x| acc.append(x))
}

pub fn fold_map<A, M: Monoid>(iter: impl IntoIterator<Item = A>, f: impl Fn(A) -> M) -> M {
    iter.into_iter().map(f).fold(M::empty(), |acc, x| acc.append(x))
}

// Choose the instance by selecting the newtype:
let nums = vec![1i64, 2, 3, 4, 5];
let sum     = fold_map(nums.iter().copied(), Sum);      // Sum(15)
let product = fold_map(nums.iter().copied(), Product);  // Product(120)
let max     = fold_map(nums.iter().copied(), Max);      // Max(5)
```

The newtype pattern for a foreign type:

```rust
// Imagine Vec<T> is from another crate and Semigroup is also external.
// We can't impl Semigroup for Vec<T>, but we CAN for our own newtype:
pub struct Concat<T>(pub Vec<T>);

impl<T: Clone> Semigroup for Concat<T> {
    fn append(mut self, other: Self) -> Self {
        self.0.extend_from_slice(&other.0);
        self
    }
}
```

## What This Unlocks

- **Multiple interpretations** — `Sum`/`Product`/`Max`/`Min`/`All`/`Any` for any numeric type
- **Safe foreign-type extensions** — implement your traits for third-party types via newtype
- **Correctness guarantees** — coherence means you always know exactly which impl is running

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Instance uniqueness | Enforced by modules — instances are explicit values | Orphan rule — compiler error on duplicate impls |
| Multiple instances for one type | Different modules, explicitly applied | Newtype wrappers — different types, different impls |
| Selecting an instance | Pass the module explicitly | Wrap the value in the appropriate newtype |
| Foreign-type restriction | None — functors are explicit | Orphan rule applies — newtype required |
| Coherence mechanism | No global coherence — users manage | Global coherence guaranteed by compiler |
