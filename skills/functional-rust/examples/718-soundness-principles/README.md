# 718: Soundness, Undefined Behaviour, and Safety Invariants

**Difficulty:** 5  **Level:** Master

The principles behind writing safe abstractions over unsafe internals — and the consequences of getting it wrong.

## The Problem This Solves

An `unsafe` block is not the end of the story — it's the beginning of a responsibility. Writing `unsafe` code that works correctly in isolation is necessary but not sufficient. The real challenge is writing code that is *sound*: code where no sequence of safe Rust operations can ever trigger undefined behaviour, no matter how a caller uses the API.

Unsoundness is particularly insidious because it can appear in code that never contains a single `unsafe` block. A poorly designed `unsafe impl Send` in a library can allow a non-Send type to be moved across threads from entirely safe calling code. A public function that exposes a raw pointer without lifetime constraints can allow safe code to construct a use-after-free. These failures are the worst kind: the unsafe code looks correct, the caller looks correct, but the combination is undefined behaviour.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern. When you do use it, soundness is the standard you must meet.

## The Intuition

**Soundness** means: no safe Rust program can produce undefined behaviour by using your API. This is a global property — you must reason about all possible calling patterns, not just the ones you tested.

**Undefined behaviour (UB)** means: the compiler is allowed to assume this never happens, and can delete, reorder, or corrupt the surrounding code when it does. Common UB in Rust: dereferencing a null or dangling pointer, creating an unaligned reference, violating aliasing rules (two `&mut` to the same memory), out-of-bounds memory access, and reading uninitialised memory.

**Safety invariants** are the conditions your type must maintain to prevent UB. They live in `// SAFETY:` comments and `# Safety` doc sections — the only documentation the compiler can't generate automatically.

## How It Works in Rust

```rust
// ── Pattern 1: Invariant maintained by the type ──────────────────────────
/// A Vec whose elements are always sorted ascending.
/// Invariant: self.0 is sorted at all times.
pub struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> SortedVec<T> {
    pub fn insert(&mut self, val: T) {
        // Binary-search insertion maintains the invariant.
        let pos = self.0.partition_point(|x| x <= &val);
        self.0.insert(pos, val);
    }

    /// # Safety
    /// `idx` must be `< self.len()`. Out-of-bounds → immediate UB.
    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {
        // SAFETY: Caller guarantees idx < self.len(); Vec buffer is valid
        // for self.len() elements; we never expose a &mut alongside this &T.
        unsafe { self.0.get_unchecked(idx) }
    }
}

// ── Pattern 2: SAFETY comments as load-bearing documentation ─────────────
// Bad (no SAFETY comment — future refactors can invalidate silently):
let x = unsafe { *ptr };

// Good (SAFETY comment explains the proof):
let x = unsafe {
    // SAFETY: ptr was derived from a live Box<i32> that was mem::forgotten
    // on line 42; the Box is guaranteed to outlive this function by the
    // calling convention documented in module-level docs.
    *ptr
};

// ── Pattern 3: Minimise the trusted surface area ─────────────────────────
// The smaller the unsafe perimeter, the fewer invariants to maintain.
// Wrap unsafe internals in a private module; expose only safe public APIs.
```

The three disciplines of soundness: (1) make invariants explicit in doc comments, (2) make the type system enforce as many invariants as possible, (3) keep the unsafe perimeter as small as possible so auditing is tractable.

## What This Unlocks

- **Standard library quality** — the standard library is sound: you can never cause UB with safe code alone, regardless of how you combine its APIs.
- **Trusted library ecosystem** — crates marked `#[forbid(unsafe_code)]` guarantee no unsafe internals; crates with unsafe internals need audit for soundness.
- **Formal verification** — tools like Miri (UB detector) and Kani (model checker) check soundness properties automatically for test inputs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Undefined behaviour | Near-absent (GC, no manual memory) | Well-defined set of UB sources; compiler exploits them |
| Safety invariants | Implicit in type design | Explicit `// SAFETY:` comments and `# Safety` doc sections |
| Soundness | Guaranteed by GC + type system | Maintained manually for `unsafe` code |
| UB detection | Not applicable | Miri runs your tests under a UB interpreter |
| Trust boundary | Module visibility | `unsafe` block / `unsafe fn` / `unsafe impl` — all auditable |
