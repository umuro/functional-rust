# 704: Dereferencing Raw Pointers Safely

**Difficulty:** 4  **Level:** Expert

Safely read through a `*const T` / `*mut T` by proving four invariants before entering `unsafe`.

## The Problem This Solves

Raw pointers — `*const T` and `*mut T` — are Rust's escape hatch for C interop, custom allocators, and intrusive data structures. Unlike references, they carry no lifetime and no borrow-checker protection. The compiler trusts you to know what you're doing. When you get it wrong, you get undefined behaviour: memory corruption, stack smashing, data races.

The challenge is that dereferencing a raw pointer is safe in *intent* far more often than the type system can prove. You receive a pointer from a C library; you know it's valid. You have a `*mut Node` in a linked list you built yourself; you know no other reference exists. The compiler can't see any of that — you have to write the proof yourself.

The canonical pattern wraps the dereference in a function that explicitly checks: (1) non-null, (2) properly aligned, (3) points to an initialized value, and (4) no exclusive `&mut` reference to the same location exists. Document these proofs in `// SAFETY:` comments — they are the contract future maintainers will rely on.

## The Intuition

Think of a raw pointer as a street address written on a piece of paper. It might be valid. It might be a demolished building. It might be in a country that uses different door-numbering rules. Before you walk through the door, you need to verify the address is real (`is_null()`), that your key fits the lock (alignment), that something actually lives there (initialized), and that nobody else has a key right now (no `&mut` alias).

The `safe_deref<T: Copy>` pattern returns `Option<T>` — `None` on any check failure, `Some(value)` on success. The `Copy` bound lets you take a snapshot of the value without worrying about aliasing after the fact.

## How It Works in Rust

```rust
pub fn safe_deref<T: Copy>(ptr: *const T) -> Option<T> {
    // Check 1: non-null
    if ptr.is_null() { return None; }
    
    // Check 2: alignment
    if (ptr as usize) % std::mem::align_of::<T>() != 0 {
        return None;
    }
    
    Some(unsafe {
        // SAFETY:
        // 1. ptr is non-null (checked above).
        // 2. ptr is properly aligned (checked above).
        // 3. Caller guarantees pointee is initialized.
        // 4. T: Copy — shared read, no exclusive alias needed.
        *ptr
    })
}
```

For mutable writes, the same checks apply plus you must prove exclusive access:

```rust
pub fn safe_write<T>(ptr: *mut T, val: T) -> bool {
    if ptr.is_null() { return false; }
    if (ptr as usize) % std::mem::align_of::<T>() != 0 { return false; }
    unsafe {
        // SAFETY: non-null, aligned, and caller guarantees no other reference.
        ptr.write(val);
    }
    true
}
```

Use `NonNull<T>` (example 705) when you want to bake the non-null invariant into the type itself.

## What This Unlocks

- **C interop**: Receive `*mut T` from a C library and safely wrap it in Rust logic without UB.
- **Custom allocators**: Manage raw memory in intrusive data structures (linked lists, arenas) while keeping all unsafe contained in one place.
- **Zero-cost abstractions**: Build safe APIs over raw-pointer internals — callers never touch `unsafe`, but the hot path has no overhead.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Dereference | Transparent, GC-managed | `*ptr` inside `unsafe {}` |
| Null check | Not needed (no null) | `.is_null()` or `NonNull<T>` |
| Alignment | Guaranteed by runtime | Must be verified manually |
| Dangling risk | GC prevents it | Caller must track lifetimes |
| Safety documentation | None needed | `// SAFETY:` comment is the proof |
| Exclusive access | GC tracks via value semantics | Caller proves no `&mut` alias |
