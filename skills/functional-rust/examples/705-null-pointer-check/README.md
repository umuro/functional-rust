# 705: Null Pointer Handling with NonNull<T>

**Difficulty:** 4  **Level:** Expert

Encode the non-null invariant into the type system using `NonNull<T>` and get the null-pointer optimisation for free.

## The Problem This Solves

When writing custom collections in Rust — linked lists, trees, arena allocators — you often need optional pointers: "this node has a next node, or it doesn't." The obvious choice is `Option<*mut Node<T>>`, but that carries two problems. First, it gives up nothing: the compiler doesn't help you avoid null dereferences any more than a raw pointer would. Second, it wastes memory — `Option<*mut T>` is two words wide on most platforms because the compiler can't prove the pointer is non-null.

C FFI brings the same problem from the other direction. A C function returning `*T` uses NULL to signal absence or error. Rust needs to model that contract precisely — not just "here's a raw pointer" but "this might be null, and you must check before using it."

`NonNull<T>` solves both sides. It's a `*mut T` that is statically guaranteed to be non-null. The compiler knows this guarantee, so `Option<NonNull<T>>` compresses to pointer size — exactly like C's nullable pointer, but type-safe. Null handling moves to `NonNull::new()`, which returns `Option<NonNull<T>>`, forcing explicit handling at construction time rather than silently at dereference time.

## The Intuition

Think of `NonNull<T>` as a `*mut T` with a pinky-swear to the compiler: "this pointer is never null." The compiler takes that promise seriously enough to let `Option<NonNull<T>>` use the null bit pattern for `None` — the same trick C++ `std::optional<T*>` uses, but guaranteed at the type level.

You pay for the guarantee once, at `NonNull::new(ptr)` — which returns `Option<NonNull<T>>` and forces you to handle the null case. After that, every use of the `NonNull` is null-free with no runtime cost.

## How It Works in Rust

```rust
use std::ptr::NonNull;

// Construct from a raw pointer — null check happens here, once.
fn from_raw<T>(ptr: *mut T) -> Option<NonNull<T>> {
    NonNull::new(ptr)  // None if ptr is null
}

// Dereference — still unsafe, but null is ruled out by the type.
unsafe fn read_nonnull<T: Copy>(p: NonNull<T>) -> T {
    // SAFETY: NonNull guarantees non-null.
    // Caller guarantees: aligned, initialized, no exclusive alias.
    *p.as_ptr()
}

// The null-pointer optimisation in action:
assert_eq!(
    std::mem::size_of::<Option<NonNull<u8>>>(),
    std::mem::size_of::<*mut u8>()  // same size — no overhead!
);
```

For a linked list node:

```rust
struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,  // None = end of list
}
```

`next` is pointer-sized. `None` is represented as a null pointer. Traversal: `while let Some(p) = cursor { let node = unsafe { p.as_ref() }; ... cursor = node.next; }`.

## What This Unlocks

- **Zero-overhead optional pointers**: `Option<NonNull<T>>` is pointer-sized — use it anywhere you'd use a nullable C pointer, without any size penalty.
- **Safer FFI**: Wrap C's nullable return values in `NonNull::new(ptr).ok_or(MyError::Null)` at the boundary — null handling is explicit and typed.
- **Custom collections**: Build intrusive linked lists, tree nodes, and arenas where the "no next" state is expressed in the type, not a sentinel value.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Absence of value | `None` (option type) | `Option<T>` or null raw pointer |
| Null-optimised option | `option` always pointer-sized | `Option<NonNull<T>>` = pointer size |
| Guaranteed non-null | Every value is non-null | `NonNull<T>` wrapper |
| Null check location | Not needed | `NonNull::new(ptr)` at construction |
| FFI null handling | Not applicable | `NonNull::new(ptr).ok_or(err)` |
| Dereference safety | GC-managed | Still `unsafe`, but null is ruled out |
