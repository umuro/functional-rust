# 135: Generic Newtype Patterns

**Difficulty:** ⭐⭐  **Level:** Intermediate

Wrap primitives and collections in named types to prevent mix-ups, add invariants, and give behavior to types you don't own.

## The Problem This Solves

You have a user ID and a product ID, both stored as `u64`. A function accepts a user ID. Nothing in the type system stops you from passing a product ID — they're both `u64`. You find the bug when a product ID gets stored in a user table at runtime.

Or you have a `String` email field and a `String` username field in a struct. Can you accidentally assign one to the other? Yes. The compiler sees the same type. Any `String` fits where any other `String` is expected.

Newtypes fix both problems. `struct UserId(u64)` and `struct ProductId(u64)` are different types. You can't pass one where the other is expected. The cost is essentially zero — Rust typically compiles newtypes to the same machine code as the inner type. As a bonus, you can attach validation logic: `Email::new(s)` returns a `Result` and an `Email` can only be constructed from a valid email string. Once you have an `Email`, you know it's valid — no need to re-validate.

## The Intuition

A newtype is a single-field tuple struct wrapping another type: `struct Email(String)`. That's it. The wrapper has zero runtime overhead — it compiles to the same memory layout. But the compiler treats it as a completely distinct type.

You choose what to expose. Implement `Deref<Target = String>` to let it behave like a `String` in read-only contexts. Implement `AsRef<str>` for interop. Keep the constructor private and expose only a validated `Email::new()`. Add custom `Display`, `Debug`, `PartialEq` impls as needed.

Generic newtypes extend this further: `struct SortedVec<T: Ord>(Vec<T>)` wraps a `Vec` and guarantees it's always sorted. The invariant is maintained because the only way to construct or modify the inner vec is through the newtype's methods.

## How It Works in Rust

```rust
// Basic newtype — different type, zero runtime overhead
#[derive(Debug, Clone, PartialEq)]
struct Email(String);

impl Email {
    // Constructor validates the invariant — only way to create an Email
    fn new(s: &str) -> Result<Self, &'static str> {
        if s.contains('@') { Ok(Email(s.to_string())) }
        else { Err("Invalid email: missing @") }
    }
    fn as_str(&self) -> &str { &self.0 }
}

// You CAN'T write: let e: Email = "hello".to_string(); — different type
// You CAN write: let e = Email::new("user@example.com")?;

// Deref lets a newtype behave like its inner type in read-only contexts
use std::ops::Deref;

struct Username(String);
impl Deref for Username {
    type Target = str;
    fn deref(&self) -> &str { &self.0 }  // Username transparently becomes &str
}

let name = Username::new("alice").unwrap();
let len = name.len();  // calls str::len() via Deref — no boilerplate needed

// Generic newtype that maintains a sorted invariant
#[derive(Debug, Clone)]
struct SortedVec<T: Ord> {
    inner: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    fn new() -> Self { SortedVec { inner: vec![] } }

    fn from_vec(mut v: Vec<T>) -> Self {
        v.sort();                    // sort on construction — invariant established
        SortedVec { inner: v }
    }

    fn insert(&mut self, val: T) {
        // Binary search maintains sort order on insert — invariant preserved
        let pos = self.inner.binary_search(&val).unwrap_or_else(|e| e);
        self.inner.insert(pos, val);
    }

    // min/max are free when sorted — no scanning needed
    fn min(&self) -> Option<&T> { self.inner.first() }
    fn max(&self) -> Option<&T> { self.inner.last() }

    // Expose inner slice read-only — callers can't break sort order
    fn as_slice(&self) -> &[T] { &self.inner }
}
```

Usage:
```rust
let mut sv = SortedVec::from_vec(vec![3, 1, 4, 1, 5]);
// sv.inner is always sorted: [1, 1, 3, 4, 5]
sv.insert(2);
// [1, 1, 2, 3, 4, 5]

// Non-empty wrapper — provably non-empty at compile time
struct NonEmpty<T> { head: T, tail: Vec<T> }
impl<T> NonEmpty<T> {
    fn first(&self) -> &T { &self.head }   // always safe — no Option needed
}
```

## What This Unlocks

- **Domain modeling** — `OrderId`, `UserId`, `SessionToken` can't be accidentally swapped; the type prevents an entire class of logic errors.
- **Validated data** — `NonNegativeInt`, `NonEmptyString`, `ValidUrl` carry the guarantee of their validation; callers don't need to re-check.
- **Behavior addition** — wrap third-party types to add methods, impls, or invariants without forking; `SortedVec` adds guaranteed ordering to `Vec`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Basic newtype | `type email = Email of string` — algebraic type | `struct Email(String)` — tuple struct |
| Constructor | Pattern match to deconstruct: `let Email s = e` | `e.0` or expose a method; field is private by default |
| Deref / inheritance | Modules, or explicit field access | `impl Deref` lets newtype act as inner type in read contexts |
| Generic wrapper | `module SortedList` with explicit `of_list` | `struct SortedVec<T: Ord>` with trait bounds on T |
