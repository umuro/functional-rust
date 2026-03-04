# 702: Unsafe Traits

**Difficulty:** 4  **Level:** Expert

Mark traits `unsafe` when correct implementation requires invariants the compiler cannot check.

## The Problem This Solves

Traits are normally a purely compile-time contract: implement the required methods, and the compiler verifies you did so correctly. But some traits carry guarantees that go beyond method signatures. `Send` says "this type is safe to move between threads" — that's a soundness claim about memory access patterns that no type checker can verify automatically. `Sync` says "sharing a reference across threads is safe" — same story.

Marking a trait `unsafe` embeds this extra responsibility in the type system. Implementors must write `unsafe impl`, explicitly acknowledging that they have proved the invariants hold for their type. A mistake in a safe impl is a logic bug; a mistake in an `unsafe impl` can cause data races, use-after-free, or undefined behaviour — which is why the compiler requires the `unsafe` keyword to make the hazard visible and auditable.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

Think of an `unsafe trait` as a contract with a legal disclaimer: "By signing here (`unsafe impl`), you certify that you have personally verified all conditions listed in the `# Safety` section." The compiler is the notary — it witnesses your signature but doesn't read the fine print. You read the fine print.

`Send` and `Sync` are the canonical examples. Rust auto-implements them for types composed entirely of `Send`/`Sync` components. The moment you add a raw pointer or non-atomic interior mutability, the auto-impl disappears and you must either not implement the trait, or write `unsafe impl` and prove it's sound.

## How It Works in Rust

```rust
/// Marker trait: implementors guarantee safe cross-thread use.
///
/// # Safety
/// The type must not contain non-atomic interior mutability or raw
/// pointers that could alias unsynchronised across threads.
pub unsafe trait ThreadSafe: Send + Sync {
    fn describe(&self) -> String;
}

pub struct AtomicCounter {
    value: std::sync::atomic::AtomicI64,
}

// SAFETY: AtomicI64 provides sequentially-consistent atomic operations.
// No raw pointers, no non-Sync interior mutability.
unsafe impl ThreadSafe for AtomicCounter {
    fn describe(&self) -> String {
        format!("AtomicCounter({})", self.value.load(std::sync::atomic::Ordering::SeqCst))
    }
}

// This would NOT compile — *mut T is !Send:
// struct NotSend { _ptr: *mut i32 }
// thread::spawn(move || drop(NotSend { _ptr: std::ptr::null_mut() })); // error
```

The discipline: when you add `unsafe trait`, document *exactly* what the implementor must prove. When you write `unsafe impl`, verify each point in the `# Safety` section before you type `unsafe`.

## What This Unlocks

- **Thread-safe wrappers** — wrap a raw pointer in a struct, prove it's safe to share, and `unsafe impl Sync` to unlock `Arc<YourType>`.
- **Custom allocators** — the `GlobalAlloc` trait is `unsafe` because a buggy allocator corrupts the entire process heap.
- **Embedded HAL traits** — peripheral drivers mark traits `unsafe` when concurrent access to a hardware register would cause undefined hardware behaviour.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread-safety marker | No language-level concept | `Send` / `Sync` unsafe traits |
| Trait implementation trust | Compiler checks signatures only | `unsafe impl` for `unsafe trait` — manual proof required |
| Auto-derivation | N/A | Rust auto-impls `Send`/`Sync` when all fields are Send/Sync |
| Opt-out | N/A | `impl !Send for T {}` (negative impl) |
| Unsafe impl visibility | N/A | `cargo geiger` counts unsafe impls in dependency tree |
