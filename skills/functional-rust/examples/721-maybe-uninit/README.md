# 721: `MaybeUninit` — Safe Uninitialized Memory

**Difficulty:** 4  **Level:** Expert

Allocate memory without initialising it — legally, safely, and without triggering undefined behaviour.

## The Problem This Solves

Sometimes you need to allocate a buffer before you have values to fill it with: a fixed-size array where elements will be initialised one-by-one, a "C output parameter" pattern where a function writes into a provided slot, or a pool where you want to skip the zero-fill of `vec![Default::default(); n]`.

The naive approach — `let mut arr: [T; N] = unsafe { mem::zeroed() }` — is undefined behaviour for types where zero-bytes are not a valid value (references, `NonNull`, enums with no zero variant). And even for types where zero is valid, it's wasteful if you're going to overwrite every element anyway.

`MaybeUninit<T>` is the correct solution: it holds memory that may or may not be initialised. Reading from it before writing is still undefined behaviour, but now *the type makes that obvious*. The `.assume_init()` call is the unsafe operation where you assert "every byte has been validly written". Everything else is safe.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

Think of `MaybeUninit<T>` as a blank form. The form is printed (memory allocated) but the fields are empty (uninitialised). You fill in each field (`.write(value)`). When all fields are filled, you sign it (`.assume_init()`) and it becomes a valid `T`. Looking at a blank form and treating it as if it's filled in (`assume_init` before writing) is the mistake — that's the undefined behaviour you must prevent.

The compiler cannot track which `MaybeUninit` slots have been written. It trusts your `// SAFETY:` comment on `assume_init`. The proof is yours to give.

## How It Works in Rust

```rust
use std::mem::MaybeUninit;

// ── Pattern 1: Single-value output parameter ─────────────────────────────
fn fill_value(out: &mut MaybeUninit<u32>, x: u32) {
    out.write(x * 2);  // safe write — marks the slot as initialised
}

let mut slot = MaybeUninit::<u32>::uninit();
fill_value(&mut slot, 21);
let value = unsafe {
    // SAFETY: fill_value unconditionally called `.write()` above,
    // so `slot` is fully initialised before assume_init.
    slot.assume_init()
};
assert_eq!(value, 42);

// ── Pattern 2: Array initialised element-by-element ─────────────────────
pub fn array_from_fn<T, const N: usize>(mut f: impl FnMut(usize) -> T) -> [T; N] {
    let mut arr: [MaybeUninit<T>; N] = unsafe {
        // SAFETY: An array of MaybeUninit needs no initialisation —
        // MaybeUninit is specifically designed to hold uninitialised memory.
        MaybeUninit::uninit().assume_init()
    };

    for i in 0..N {
        arr[i].write(f(i));  // write each slot — safe
    }

    unsafe {
        // SAFETY: Every element (0..N) was written by the loop above.
        let ptr = arr.as_ptr() as *const [T; N];
        std::mem::forget(arr);  // don't drop the MaybeUninit wrappers
        ptr.read()
    }
}

// ── Pattern 3: Vec without zero-initialisation ───────────────────────────
let mut v: Vec<MaybeUninit<u32>> = Vec::with_capacity(1000);
for i in 0u32..1000 { v.push(MaybeUninit::new(i * i)); }
// Now safely transmute to Vec<u32> — all elements written.
```

## What This Unlocks

- **High-performance buffers** — skip the zero-fill when building output buffers that will be fully overwritten before reading (network packet assembly, compression output).
- **`T: !Default` arrays** — initialise fixed-size arrays of types that don't implement `Default`, like `MutexGuard` or other RAII handles.
- **Memory pools** — allocate slots in a pool without initialising them until an object is actually placed (example 726 uses this technique).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Uninitialized memory | `Bytes.create n` (zeroed; UB-free) | `MaybeUninit::uninit()` — explicitly uninitialized |
| Reading before write | Runtime error or garbage | Undefined behaviour (UB) — compiler may elide the read |
| Type-safe uninit | No — Bytes is untyped | Yes — `MaybeUninit<T>` is generic, typed |
| Initialisation marker | Not tracked | `.write()` marks initialised; `.assume_init()` asserts it |
| GC protection | GC guarantees zero-init | No GC — manual proof required before assume_init |
