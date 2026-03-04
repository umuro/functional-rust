# 740: PhantomData for Variance Control

**Difficulty:** 4  **Level:** Expert

Choose whether a lifetime or type parameter is covariant, contravariant, or invariant by selecting the right `PhantomData` variant — the key to writing correct unsafe code and custom smart pointers.

## The Problem This Solves

When you build a custom data structure using raw pointers or unsafe code, the Rust compiler doesn't know how your type relates to its lifetime or type parameters. It can't automatically determine whether `MyType<&'long T>` can be used where `MyType<&'short T>` is expected, or vice versa.

Get variance wrong and you get one of two bad outcomes: the compiler rejects valid code (overly strict), or it accepts code that creates use-after-free bugs (dangerously permissive). In safe Rust, variance is inferred automatically from struct fields. In unsafe Rust — when you have a `*const T` or `*mut T` field — you must tell the compiler what variance you intend by choosing the right `PhantomData`.

This matters in real code: implementing `Vec<T>` requires covariance so `Vec<&'long str>` can be used as `Vec<&'short str>`. Implementing a mutable iterator requires invariance so you can't shorten the lifetime of the thing being iterated. The `PhantomData` you choose is a contract with the borrow checker.

## The Intuition

Variance describes the direction of substitutability for a type parameter:

- **Covariant**: if `T` is a subtype of `U`, then `Container<T>` is a subtype of `Container<U>`. You can use a longer-lived reference where a shorter-lived one is expected. `PhantomData<T>` is covariant in `T`.

- **Contravariant**: the opposite — `Container<U>` is a subtype of `Container<T>`. A function that *accepts* `&'long` is more powerful than one that *accepts* `&'short`. `PhantomData<fn(T)>` is contravariant in `T`.

- **Invariant**: no substitution allowed — the lifetimes must match exactly. Mutable access requires invariance to prevent soundness holes. `PhantomData<Cell<T>>` or `PhantomData<*mut T>` is invariant in `T`.

The right choice depends on what your type does with `T`: produces it (covariant), consumes it (contravariant), or both/mutates it (invariant).

## How It Works in Rust

```rust
use std::marker::PhantomData;
use std::cell::Cell;

// ── Covariant: type "produces" T ─────────────────────────────────────────────
// PhantomData<T> — covariant in T
// If T: 'long, Producer<T> can be used where Producer with 'short T is needed
pub struct Producer<T> {
    _phantom: PhantomData<T>,   // covariant: "I give out T values"
}

// ── Contravariant: type "consumes" T ─────────────────────────────────────────
// PhantomData<fn(T)> — contravariant in T
// Consumer<&'short str> can be used where Consumer<&'long str> is needed
pub struct Consumer<T> {
    _phantom: PhantomData<fn(T)>,  // contravariant: "I accept T values"
}

// ── Invariant: type mutates T ─────────────────────────────────────────────────
// PhantomData<Cell<T>> or PhantomData<*mut T> — invariant in T
// Exact lifetime required — no covariant or contravariant substitution
pub struct Invariant<T> {
    _phantom: PhantomData<Cell<T>>,  // invariant: "I read AND write T"
}

// ── Practical: custom slice view with correct covariance ─────────────────────
// Without PhantomData<&'a T>, the compiler would reject the lifetime parameter
pub struct SliceView<'a, T> {
    ptr: *const T,
    len: usize,
    _phantom: PhantomData<&'a T>,  // covariant in 'a and T — correct for read-only view
}

impl<'a, T> SliceView<'a, T> {
    pub fn from_slice(s: &'a [T]) -> Self {
        SliceView { ptr: s.as_ptr(), len: s.len(), _phantom: PhantomData }
    }

    pub fn len(&self) -> usize { self.len }

    pub fn get(&self, i: usize) -> Option<&'a T> {
        if i < self.len {
            // SAFETY: ptr is valid for 'a (same lifetime as original slice)
            Some(unsafe { &*self.ptr.add(i) })
        } else {
            None
        }
    }
}

// All phantom types are zero-sized:
assert_eq!(std::mem::size_of::<Producer<String>>(), 0);
assert_eq!(std::mem::size_of::<Consumer<String>>(), 0);
assert_eq!(std::mem::size_of::<Invariant<String>>(), 0);
```

**Variance cheat sheet:**

| You want | Use |
|----------|-----|
| Covariant in `T` | `PhantomData<T>` |
| Contravariant in `T` | `PhantomData<fn(T)>` |
| Invariant in `T` | `PhantomData<Cell<T>>` or `PhantomData<*mut T>` |
| Covariant in `'a` (read-only view) | `PhantomData<&'a T>` |
| Invariant in `'a` (mutable view) | `PhantomData<&'a mut T>` |

## What This Unlocks

- **Correct custom smart pointers** — implementing `Box<T>`, `Arc<T>`, or a custom arena requires the right variance so the borrow checker accepts valid lifetimes.
- **Safe raw-pointer wrappers** — wrapping `*const T` for a read-only view needs covariance; wrapping `*mut T` for a mutable view needs invariance — `PhantomData` is how you communicate this.
- **Understanding stdlib internals** — `Vec<T>` uses `PhantomData<T>` (covariant), `Cell<T>` uses `PhantomData<Cell<T>>` (invariant), `fn(T) -> U` is contravariant in `T` — all follow these rules.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Variance | Inferred by the type-checker from usage | Inferred for safe structs; must be declared explicitly with `PhantomData` for raw-pointer structs |
| Covariant phantom | `type +'a t` annotation | `PhantomData<&'a T>` or `PhantomData<T>` |
| Contravariant phantom | `type -'a t` annotation | `PhantomData<fn(T)>` |
| Invariant | Default for mutable types | `PhantomData<Cell<T>>` or `PhantomData<*mut T>` |
| Why it matters | Type-checker soundness for abstract types | Borrow checker soundness for unsafe raw-pointer abstractions |
