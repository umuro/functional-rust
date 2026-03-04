# 701: Unsafe Functions

**Difficulty:** 4  **Level:** Expert

Declare `unsafe fn` to push safety contracts to the caller, then wrap them in safe APIs.

## The Problem This Solves

Some operations have preconditions the compiler cannot check — "this index is in bounds", "these memory regions don't overlap", "this pointer has been initialised". You could document them in a comment and hope callers comply, but Rust provides a better mechanism: `unsafe fn`. Marking a function `unsafe` makes the contract visible in the type system — callers *must* write an `unsafe` block to invoke it, forcing them to acknowledge the preconditions exist.

The safe-wrapper idiom builds on top of this. You write a private `unsafe fn` that is fast and unchecked, then a public safe function that validates the preconditions first. Callers interact only with the safe API; the `unsafe fn` is an implementation detail. This is exactly how `slice::get_unchecked` and `ptr::copy_nonoverlapping` work in the standard library.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

Think of an `unsafe fn` as a professional power tool: it does the job faster than the consumer version, but it ships without the guard rail. The `// # Safety` doc section is the user manual — it lists every condition that must be true before you press the trigger. A safe wrapper is the guard rail: it checks those conditions before handing control to the dangerous function.

The compiler does not verify `// # Safety` comments. It only verifies that the caller acknowledged the contract by writing `unsafe { }`. The human — you, the reviewer, the auditor — reads the comment and decides whether the acknowledgement is justified.

## How It Works in Rust

```rust
/// Copy `n` bytes from `src` to `dst`.
///
/// # Safety
/// - `src` must be valid for `n` bytes of reads.
/// - `dst` must be valid for `n` bytes of writes.
/// - The two regions must not overlap.
unsafe fn raw_copy(src: *const u8, dst: *mut u8, n: usize) {
    for i in 0..n {
        // SAFETY: Caller guarantees validity, non-overlap, and correct size.
        *dst.add(i) = *src.add(i);
    }
}

/// Safe wrapper: validates slice lengths, then delegates to raw_copy.
pub fn safe_copy(src: &[u8], dst: &mut [u8]) -> Result<(), String> {
    if src.len() != dst.len() {
        return Err(format!("length mismatch: {} vs {}", src.len(), dst.len()));
    }
    unsafe {
        // SAFETY: Both slices are valid for their full length.
        // Rust's borrow checker guarantees &[u8] and &mut [u8] cannot alias.
        raw_copy(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
    Ok(())
}
```

The structure is always: (1) document preconditions in `# Safety`, (2) implement the fast path, (3) write a safe wrapper that enforces those preconditions before calling through.

## What This Unlocks

- **Standard library primitives** — `slice::get_unchecked`, `ptr::copy_nonoverlapping`, `str::from_utf8_unchecked` all follow this exact pattern.
- **Custom collection internals** — a sorted map can expose a safe insert and an `unsafe get_unchecked` for callers who can prove the index is valid.
- **Performance-critical inner loops** — skip redundant bounds checks after a single safe validation at the loop entry point.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Precondition declaration | `[@warning "-8"]` or doc comment | `unsafe fn` + `# Safety` doc section |
| Caller acknowledgement | None required | Must write `unsafe { }` to call |
| Unchecked array access | `Array.unsafe_get` | `slice::get_unchecked` (unsafe fn) |
| Safe wrapper pattern | Checked wrapper calls unchecked | `pub fn` validates, then calls `unsafe fn` |
| Compiler enforcement | No | Yes — calling `unsafe fn` without `unsafe` is a compile error |
