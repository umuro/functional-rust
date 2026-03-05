📖 **[View on hightechmind.io →](https://hightechmind.io/rust/538-lifetime-variance)**

---

# 538: Variance — Covariant, Contravariant, Invariant

**Difficulty:** 5  **Level:** Advanced

Variance determines how subtyping flows through type constructors. Get it wrong in unsafe code and you create memory unsafety. Understanding it explains why `&mut T` can't be coerced, why `fn(T)` works backwards, and why `PhantomData` has type parameters.

## The Problem This Solves

Variance is why these two cases have opposite rules:

```rust
// OK: &'static str coerces to &str — reading, safe to shorten lifetime
let s: &'static str = "hello";
let shorter: &str = s; // covariant — longer is subtype of shorter

// NOT OK: can't coerce &mut &'long T to &mut &'short T
let mut long_ref: &'static str = "static";
// let r: &mut &str = &mut long_ref; // invariant — would allow replacing with short ref
```

If mutable references were covariant, you could replace a `&'static str` slot with a `&'short str` value, creating a dangling reference when `'short` expires. Invariance prevents this at the type level.

Without understanding variance, you'll be confused by phantom type parameters, certain type errors in generic code, and why unsafe raw pointer types need careful `PhantomData` declarations.

## The Intuition

**Covariant**: "If A is a subtype of B, then `F<A>` is a subtype of `F<B>`." Lifetime gets shorter → type gets more general. Safe for reading because you're only getting *less* specific. Examples: `&T`, `Box<T>`, `Vec<T>`.

**Contravariant**: The opposite — `F<B>` is a subtype of `F<A>` when A <: B. Function arguments flip the subtype direction. A function that accepts any animal (`fn(Animal)`) can stand in for one that only needs a dog (`fn(Dog)`) — it's more flexible. Examples: `fn(T)` in its argument position.

**Invariant**: No subtyping at all — must match exactly. Required when a type allows both reading *and* writing. `&mut T` is invariant because you can both read T and write T — allowing coercion in either direction would break safety.

## How It Works in Rust

**Covariant — `&T` and `Box<T>`:**

```rust
// &'long T is a subtype of &'short T (covariant in lifetime)
let long_str: &'static str = "static";
let shorter: &str = long_str;  // covariant coercion — safe for reading

// Vec<T> is covariant in T
let statics: Vec<&'static str> = vec!["a", "b", "c"];
let dynamic: Vec<&str> = statics;  // covariant — OK!
```

**Invariant — `&mut T`:**

```rust
// If &mut T were covariant, this would compile and break:
fn bad<'short>(r: &mut &'short str, s: &'short str) {
    *r = s; // If &mut &'long were subtypeable to &mut &'short,
            // we could replace a long-lifetime slot with a short-lifetime ref
}
// Rust forbids: &mut T is invariant — prevents lifetime laundering
```

**Contravariant — function argument positions:**

```rust
// fn(&'short str) is a subtype of fn(&'long str) — backwards from reference subtyping!
// A function accepting shorter-lived refs is MORE flexible: it works with anything
let accepts_any: fn(&str) = |s| println!("{}", s);
// fn(&str) can stand in for fn(&'static str) — it's less demanding
```

**`PhantomData` to declare variance in unsafe types:**

```rust
use std::marker::PhantomData;

// Covariant in 'a and T — same as &'a T
struct MyRef<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>,  // covariant: can shorten 'a
}

// Invariant in T — same as &mut T
struct MyMutRef<T> {
    ptr: *mut T,
    _marker: PhantomData<*mut T>,  // invariant: no coercion allowed
}

// Contravariant in T — same as fn(T)
struct Callback<T> {
    f: *const (),
    _marker: PhantomData<fn(T)>,  // contravariant: accepts supertypes of T
}
```

## What This Unlocks

- **Correct unsafe code** — when you use raw pointers in a struct, choosing the right `PhantomData` variance ensures the borrow checker still enforces memory safety correctly.
- **Understanding compiler errors** — "lifetime mismatch in mutable reference" is almost always an invariance error. The compiler can't coerce because allowing it would be unsound.
- **Phantom type state machines** — `PhantomData<State>` in state machine patterns is covariant in `State`, which is why the state type can vary freely without runtime cost.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Subtyping through containers | GC tracks all refs — no variance checking | `&T` is covariant, `&mut T` is invariant — enforced at compile time |
| Function argument subtyping | Contravariance applies to function types | `fn(T)` is contravariant in T — opposite of reference covariance |
| Mutable state and subtyping | Mutable refs can coerce with care | `&mut T` invariant — prevents lifetime laundering through mutation |
| Phantom type parameters | Functor laws express variance | `PhantomData<&'a T>` = covariant; `PhantomData<fn(T)>` = contravariant; `PhantomData<*mut T>` = invariant |
| Runtime safety guarantee | GC prevents all dangling pointers | Variance rules + borrow checker enforce memory safety statically |
