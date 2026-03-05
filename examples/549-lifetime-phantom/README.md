📖 **[View on hightechmind.io →](https://hightechmind.io/rust/549-lifetime-phantom)**

---

# 549: PhantomData for Lifetime Variance

**Difficulty:** 5  **Level:** Advanced

`PhantomData<T>` tells the compiler "this type logically contains a T even though no T appears in the struct fields." It controls variance, drop checking, and type-state encoding — all at zero runtime cost.

## The Problem This Solves

When you write a struct using raw pointers, the compiler has no idea what lifetime relationship exists between the struct and the pointed-at data. Without guidance, it can't enforce borrow rules correctly:

```rust
struct MyRef<'a, T> {
    ptr: *const T,      // raw pointer — compiler doesn't track 'a
    // no 'a usage → compiler warns: 'a is unused
}
// ERROR or warning: lifetime parameter 'a is never used
```

You need to tell the compiler: "treat this `*const T` as if it were `&'a T` for borrow-checking purposes." That's what `PhantomData<&'a T>` does — it adds no data, but declares the variance relationship.

Beyond lifetimes, `PhantomData` enables zero-cost type state machines: the compiler enforces state transitions at compile time by using a phantom type parameter.

## The Intuition

`PhantomData<T>` is a zero-sized type that says "pretend I contain a T." It costs nothing at runtime but provides the compiler with the type information it needs for:

- **Variance**: `PhantomData<&'a T>` → covariant in `'a` (same as `&'a T`). `PhantomData<*mut T>` → invariant. `PhantomData<fn(T)>` → contravariant.
- **Drop checking**: `PhantomData<T>` tells the drop checker that your type "owns" a T, so it runs T's destructor logic.
- **Type state**: `PhantomData<State>` makes the type `Struct<State>` where `State` is a phantom — different states are different types, enforced by which methods are available.

## How It Works in Rust

**Variance via PhantomData:**

```rust
use std::marker::PhantomData;

// Covariant in 'a — same behavior as &'a T
struct CovariantRef<'a, T> {
    ptr: *const T,
    _marker: PhantomData<&'a T>,  // covariant: &'long T works where &'short T expected
}

// Invariant in T — same behavior as &mut T
struct InvariantWrapper<T> {
    ptr: *mut T,
    _marker: PhantomData<*mut T>,  // invariant: must match exactly
}

// Contravariant in T — same behavior as fn(T)
struct Contravariant<T> {
    f: fn(T),
    _marker: PhantomData<fn(T)>,  // contravariant in T
}
```

**Type state machine — zero runtime cost:**

```rust
struct Locked;    // zero-sized state types
struct Unlocked;

struct Door<State> {
    name: String,
    _state: PhantomData<State>,  // State is purely compile-time
}

impl Door<Locked> {
    fn new(name: &str) -> Self {
        Door { name: name.to_string(), _state: PhantomData }
    }
    
    fn unlock(self) -> Door<Unlocked> {
        println!("Unlocking {}", self.name);
        Door { name: self.name, _state: PhantomData }
    }
    // No `open` method here — can't open a locked door
}

impl Door<Unlocked> {
    fn open(&self) { println!("Opening {}", self.name); }
    fn lock(self) -> Door<Locked> {
        Door { name: self.name, _state: PhantomData }
    }
}

// Compile-time enforcement:
let door = Door::<Locked>::new("front");
// door.open();   // ERROR: Door<Locked> has no method `open`
let door = door.unlock();
door.open();     // fine — Door<Unlocked>
```

**Ownership signaling for drop check:**

```rust
// PhantomData<T> (not &T) tells drop checker we OWN a T — its destructor must run
struct OwnedPtr<T> {
    ptr: *mut T,
    _owned: PhantomData<T>,  // "I own this T — run T's Drop when I'm dropped"
}

impl<T> Drop for OwnedPtr<T> {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ptr)); }
    }
}
```

## What This Unlocks

- **Safe unsafe abstractions** — raw pointer types with correct variance declarations behave exactly like their safe counterparts from a borrow-checker perspective. Wrap `*const T` with `PhantomData<&'a T>` and the compiler enforces the same lifetime rules as `&'a T`.
- **Zero-cost type state machines** — enforce protocol correctness (connection must be opened before used, file must be closed after write) with zero runtime overhead. The state is purely a type parameter.
- **Correct custom containers** — a custom `Vec`-like type needs `PhantomData<T>` to signal ownership so that the compiler's drop checker works correctly and variance is covariant (matching `Vec<T>`).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom type parameters | `type 'state lock = unit` — phantom type common in OCaml | `PhantomData<State>` — zero-sized, explicit, with variance control |
| Type state machines | Phantom types in modules or GADTs | `Struct<PhantomData<State>>` — different methods per state type |
| Variance declarations | Functor laws / explicit annotations | `PhantomData<&'a T>` = covariant, `PhantomData<*mut T>` = invariant |
| Raw pointer safety | Not applicable (GC) | `PhantomData` tells borrow checker how to treat the raw pointer |
| Drop semantics | GC handles all | `PhantomData<T>` signals ownership — drop checker runs T's destructor |
