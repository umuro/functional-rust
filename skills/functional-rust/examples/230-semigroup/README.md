# 230: Semigroup

**Difficulty:** ⭐⭐  **Level:** Algebra / Type Classes

The minimal combining operation: two things of the same type go in, one comes out, and the operation is associative. No identity element required.

## The Problem This Solves

You often need to combine things — take the minimum of two values, concatenate two lists, keep the first non-null result. These operations share a shape: `(T, T) -> T` with associativity. But they don't all have a natural "empty" or "zero" element. `Min` over integers has no identity (there's no integer that's larger than all others). `First` — "keep the first non-null" — has no sensible default.

Forcing these into `Monoid` would require inventing artificial identity elements or restricting to bounded types. Semigroup is the honest abstraction: just the combination, nothing more.

In practice, semigroups appear everywhere you want to reduce a *non-empty* collection. The caller knows there's at least one element; the semigroup provides the combination rule. `sconcat` over a non-empty list is the canonical operation, returning `Option<S>` to handle the empty-input case explicitly.

## The Intuition

A **semigroup** has exactly one requirement: a binary operation `combine(a, b) -> T` that is **associative** — `combine(combine(a, b), c) == combine(a, combine(b, c))`. Grouping never changes the result.

That's it. No identity. No inverse. Just: two in, one out, in any order of grouping.

Examples you already know:
- `Min` over numbers: `min(min(3, 7), 2) == min(3, min(7, 2)) == 2` ✓
- `Max` over numbers: same structure
- `First` — keep the leftmost: `first(first(a, b), c) == first(a, first(b, c)) == a` ✓
- `NonEmptyList` — concatenation: always associative, always non-empty

What semigroup is **not**: `append(a, b)` where the result could be empty and you can't tell the caller. That's where you'd need `Monoid` with an explicit `empty`.

**Newtypes are necessary** in Rust because `i64` could be a semigroup in multiple ways (min, max, sum, product, first, last...). Wrapping in `Min(i64)`, `Max(i64)` etc. lets each have its own implementation without conflict.

## How It Works in Rust

```rust
pub trait Semigroup {
    fn append(self, other: Self) -> Self;  // associative: (a+b)+c = a+(b+c)
}

// Multiple semigroup instances for i64 — newtypes prevent conflicts
pub struct Min(pub i64);
pub struct Max(pub i64);

impl Semigroup for Min {
    fn append(self, other: Self) -> Self { Min(self.0.min(other.0)) }
}
impl Semigroup for Max {
    fn append(self, other: Self) -> Self { Max(self.0.max(other.0)) }
}

// First: always keep the left element — trivially associative
pub struct First<T>(pub T);
impl<T> Semigroup for First<T> {
    fn append(self, _other: Self) -> Self { self }  // discard right
}

// sconcat: reduce a non-empty slice — returns None if slice is empty
pub fn sconcat<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    let (head, tail) = items.split_first()?;  // None if empty — explicit, not panic
    Some(
        tail.iter()
            .cloned()
            .fold(head.clone(), |acc, x| acc.append(x))
    )
}
```

Usage:
```rust
// Find minimum without an artificial "infinity" identity
let mins = [Min(3), Min(1), Min(4), Min(1), Min(5)];
assert_eq!(sconcat(&mins), Some(Min(1)));

// Keep first non-trivial value
let firsts = [First("hello"), First("world")];
assert_eq!(sconcat(&firsts).map(|f| f.0), Some("hello"));
```

## What This Unlocks

- **Honest non-empty reduction** — `sconcat` is the right function when your invariant is "I always have at least one element." `Option` in the return type makes the empty case explicit.
- **Composable strategies** — `Min`, `Max`, `First`, `Last` are interchangeable combinators. Code that takes `S: Semigroup` works with all of them.
- **Foundation for Monoid** — every `Monoid` is a `Semigroup`. When you can add an `empty`, upgrade. When you can't, semigroup is exactly right.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Abstraction | `module type SEMIGROUP` | `trait Semigroup` |
| Multiple instances | Separate named modules | Newtypes (`Min`, `Max`, `First`) |
| Empty input handling | `failwith` (runtime panic) | `Option::None` (explicit in type) |
| `sconcat` signature | `t list -> t` (partial) | `&[S] -> Option<S>` (total) |
| vs Monoid | Requires empty/identity | No identity needed |
