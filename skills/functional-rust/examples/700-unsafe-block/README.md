# 700: Unsafe Block

**Difficulty:** 4  **Level:** Expert

Minimise the `unsafe` footprint — do only what must be unsafe, keep everything else safe.

## The Problem This Solves

The Rust compiler enforces memory safety by default, but some operations genuinely cannot be verified statically: dereferencing a raw pointer, calling a C function, mutating a global variable, implementing certain marker traits, or accessing a union field. For exactly these five operations, Rust provides the `unsafe` block — a small, explicitly labelled region where you take over the compiler's job.

The key discipline is containment. An `unsafe` block is not a license to write arbitrary dangerous code throughout a function — it is a precise surgical incision. Everything that can be expressed safely should remain outside the `unsafe` block. The smaller the unsafe region, the smaller the surface area you must reason about and audit.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

An `unsafe` block is a contract between you and the compiler. You are saying: "I've checked this manually. The invariants hold. Trust me here." The compiler records the boundary in the source code so auditors, future maintainers, and tools like `cargo geiger` can find exactly what needs human review.

The five operations that only `unsafe` enables:
1. Dereference a raw pointer
2. Call an `unsafe fn`
3. Implement an `unsafe trait`
4. Mutate a `static mut` variable
5. Access a union field

Every other Rust operation — iterators, closures, arithmetic, string formatting — is always safe and belongs outside the block.

## How It Works in Rust

```rust
static mut GLOBAL_COUNTER: u64 = 0;

fn increment() {
    unsafe {
        // SAFETY: Single-threaded; no concurrent access to GLOBAL_COUNTER.
        // In multi-threaded code, replace with AtomicU64.
        GLOBAL_COUNTER += 1;
    }
    // Safe side-effects live OUTSIDE the unsafe block.
    // Logging, formatting, error handling — all stay here.
}

fn reset() {
    unsafe {
        // SAFETY: Same single-threaded guarantee.
        GLOBAL_COUNTER = 0;
    }
    // ← println! is safe; it goes here, not inside unsafe.
    println!("Counter reset to 0.");
}
```

The pattern to internalise: shrink the `unsafe` block to the minimum number of lines that genuinely require it. Bounds-check before the block, format strings after the block, validate return values after the block.

## What This Unlocks

- **Global mutable state** — counters, caches, or singletons that must outlive any particular scope, without paying the overhead of a `Mutex`.
- **OS kernel and embedded code** — hardware registers, interrupt handlers, and memory-mapped I/O all require direct memory writes that only `unsafe` can express.
- **Wrapping C libraries** — the `unsafe` block marks the exact call site where Rust's guarantees end and the C ABI begins, making the FFI boundary visible and auditable.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Unsafe region | No explicit marker; `Obj.magic` is always "trusted" | `unsafe { }` block — compiler-enforced boundary |
| Mutable global | `let x = ref 0` (always fine) | `static mut` requires `unsafe` to read or write |
| Auditability | Search for known unsafe patterns by convention | `cargo geiger` counts `unsafe` blocks automatically |
| Scope of trust | Entire module | Precisely the `unsafe { }` block |
| Safe default | Type system doesn't express safety | Safe code is the default; unsafe is the opt-in exception |
