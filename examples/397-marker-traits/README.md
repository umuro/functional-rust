📖 **[View on hightechmind.io →](https://hightechmind.io/rust/397-marker-traits)**

---

# 397: Marker Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Some type properties cannot be expressed as methods — they are structural guarantees about how a type behaves, not behaviors you can call. A type is "serializable" (safe to convert to bytes), "immutable" (guarantees no internal mutation), or "thread-safe" (safe to share across threads). Marker traits capture these properties: they have no methods, just a name. Code that requires a guarantee asks for `T: Serializable` in its bounds, and only explicitly-opted-in types pass through. This prevents accidentally passing a non-serializable type to a serialization function.

Marker traits include `Copy`, `Send`, `Sync`, `Unpin`, `UnwindSafe`, and user-defined invariants in domain-specific type systems.

## Learning Outcomes

- Understand marker traits as zero-method types expressing structural invariants
- Learn how marker trait bounds gate access to functions at compile time
- See how `ThreadSafe: Send + Sync` composes existing marker traits into a semantic concept
- Understand when to use marker traits vs. methods (invariants vs. behaviors)
- Learn the `unsafe impl` requirement for traits with safety invariants like `Send`/`Sync`

## Rust Application

In `src/lib.rs`, `Serializable`, `Immutable`, and `ThreadSafe: Send + Sync` are marker traits with no methods. `Config` opts in to both `Serializable` and `Immutable`. `Counter` uses `unsafe impl Send for Counter` and `unsafe impl Sync for Counter` to assert thread-safety manually (the compiler cannot verify this automatically because of the `AtomicU64`). `save<T: Serializable>` and `process_threadsafe<T: ThreadSafe>` gate functionality on marker bounds.

## OCaml Approach

OCaml achieves marker-like behavior through phantom types: `type ('a, 'serializable) t = T of ...` where `'serializable` is a phantom parameter set to a `serializable` type or `not_serializable`. Module signatures achieve the same with abstract types that only appear in certain signatures. OCaml has no equivalent of `unsafe impl` — safety invariants are expressed through module abstraction hiding constructors.

## Key Differences

1. **Methods**: Rust marker traits are truly empty (no methods); OCaml phantom types are also empty type parameters, but they require more infrastructure to encode constraints.
2. **Unsafe impl**: Rust's `Send`/`Sync` require `unsafe impl` to assert invariants the compiler can't verify; OCaml's module-based safety relies on hiding constructors, not unsafe declarations.
3. **Auto-derivation**: Rust auto-derives `Send`/`Sync` for types where all fields are `Send`/`Sync`; OCaml phantom types must be manually propagated.
4. **Documentation**: Rust marker traits appear in rustdoc and IDE autocompletion, making them visible; OCaml phantom type constraints require reading type signatures carefully.

## Exercises

1. **Validated marker**: Define `trait Validated {}` and create a `ValidatedEmail(String)` that implements it, but only constructable via `ValidatedEmail::new(s: &str) -> Option<ValidatedEmail>`. Write a `fn send_email<T: Validated + AsRef<str>>(addr: &T)` that only accepts validated addresses.
2. **Permission markers**: Define `trait ReadPermission {}` and `trait WritePermission {}`. Create a `File<P>` where `P` is a permission marker. Show that `fn write<P: WritePermission>(f: &File<P>)` rejects a read-only file at compile time.
3. **Sealed marker**: Combine the sealed trait pattern with markers: define a sealed `DatabaseType` marker that only your crate's `Postgres`, `Mysql`, and `Sqlite` types can implement. Write a generic `fn connect<D: DatabaseType>(config: &str)`.
