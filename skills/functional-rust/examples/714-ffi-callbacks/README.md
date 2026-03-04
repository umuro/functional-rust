# 714: FFI Callbacks — Passing Rust Functions to C

**Difficulty:** 4  **Level:** Expert

Pass Rust functions and closures to C APIs that expect function pointers, using the trampoline pattern for closures with captures.

## The Problem This Solves

C APIs frequently accept function pointers for callbacks: `qsort` takes a comparator, event loops take handler functions, plugin systems take hooks. Rust needs to pass callables across the C ABI boundary — and the rules are strict. The function must use `extern "C"` calling convention, must not panic, and must have a C-compatible signature.

Plain Rust functions (no captures) coerce to `extern "C" fn(...)` automatically — no unsafe needed. The hard case is closures with captures. A closure in Rust is a compiler-generated struct that carries its captured variables. It has no stable ABI and cannot be represented as a C function pointer. The standard solution is the *trampoline pattern*: pass the closure as a `*mut c_void` user-data pointer alongside a thin `extern "C"` wrapper function. The wrapper reconstructs the closure from the user-data pointer and calls it.

This pattern appears throughout libc: `pthread_create` takes a `void*` argument for the thread function, `qsort_r` takes a context pointer, GTK callbacks take `gpointer user_data`. Recognising it is essential for any Rust-to-C integration work.

## The Intuition

A plain function is just an address — C understands addresses. A closure is a struct plus an address — C has no concept of that. The trampoline converts: you give C two things — a function pointer (the trampoline) and a raw data pointer (your closure, boxed). The trampoline's job is to receive the raw data pointer, cast it back to your closure type, and invoke it. The trampoline is stable and C-callable; the closure lives on the Rust side of the curtain.

The lifetime discipline is critical: the closure must outlive the C callback. If C holds a pointer to your closure and the closure is dropped, you get a dangling pointer call. Heap-allocate (`Box`) the closure and ensure C drops the user-data pointer before Rust does.

## How It Works in Rust

```rust
use std::os::raw::c_void;

// ── Simple case: plain function, no captures ──────────────────────────────
extern "C" fn print_elem(v: i32) { print!("{v} "); }

// c_for_each accepts extern "C" fn(i32) — plain function coerces directly.
c_for_each(data.as_ptr(), data.len(), print_elem);

// ── Closure trampoline pattern ────────────────────────────────────────────
extern "C" fn trampoline<F>(val: i32, user_data: *mut c_void)
where F: FnMut(i32),
{
    let closure = unsafe {
        // SAFETY: user_data is a &mut F cast to *mut c_void.
        // We hold a mutable borrow for the duration of this call only.
        &mut *(user_data as *mut F)
    };
    closure(val);
}

fn call_with_closure<F: FnMut(i32)>(data: &[i32], mut f: F) {
    let user_data = &mut f as *mut F as *mut c_void;
    // Pass trampoline::<F> as the callback and &mut f as user_data.
    unsafe { c_for_each_ctx(data.as_ptr(), data.len(), trampoline::<F>, user_data) };
}
```

For long-lived callbacks (stored by C beyond the call), heap-allocate with `Box::into_raw` and free with `Box::from_raw` in a corresponding `free_callback` export.

## What This Unlocks

- **Full libc integration**: Pass Rust closures to `qsort`, `pthread_create`, event loops, and any C API that uses the `callback + user_data` pattern.
- **Plugin systems**: Export a C-callable plugin interface that invokes Rust closures internally — the plugin consumer never knows it's Rust.
- **Safe wrapper libraries**: Build idiomatic Rust wrappers around C callback-heavy APIs (OpenSSL, GTK, libcurl) where the `unsafe` trampoline is encapsulated once.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Function pointer to C | `Callback.register` | `extern "C" fn(A) -> B` |
| Closures to C | `Callback.register` (limited) | Trampoline + `*mut c_void` user_data |
| Context data | Implicit (closure captures) | Explicit `user_data` pointer |
| Lifetime of callback | GC-managed | Manual — closure must outlive C usage |
| Long-lived closure | GC root | `Box::into_raw` + explicit free |
| qsort comparator | `Array.sort` (no C needed) | `extern "C" fn(*const c_void, *const c_void) -> c_int` |
