📖 **[View on hightechmind.io →](https://hightechmind.io/rust/701-unsafe-function)**

---

# Unsafe Functions
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

An `unsafe fn` declaration means "calling this function has preconditions that the compiler cannot verify — the caller is responsible for upholding them." This is different from an `unsafe {}` block inside a function body. `unsafe fn` shifts the burden of proof to the caller and makes the unsafety visible at every call site. The safe-wrapper idiom pairs `unsafe fn` with a public safe function that validates the preconditions before calling the unsafe version.

## Learning Outcomes

- How `unsafe fn` declares that a function has unchecked preconditions
- How `# Safety` documentation in doc comments specifies the preconditions
- How the safe-wrapper idiom hides unsafe fn behind a validated public API
- Why `unsafe fn` is different from a regular function with an `unsafe {}` block
- Where `unsafe fn` appears: std library internals, allocator APIs, SIMD intrinsics

## Rust Application

`unsafe fn raw_copy(src, dst, n)` documents three preconditions: src valid for n bytes, dst valid for n bytes, non-overlapping. The `# Safety` doc section specifies exactly what the caller must guarantee. The safe wrapper `fn safe_copy(src: &[u8], dst: &mut [u8])` validates that both slices are the same length, then calls `raw_copy` inside `unsafe { }` with a SAFETY comment explaining why the preconditions hold.

Key patterns:
- `unsafe fn name(...) { ... }` — function with caller-enforced preconditions
- `/// # Safety\n/// - precondition 1\n/// - precondition 2` — required documentation
- Safe wrapper validates then calls: `if valid { unsafe { raw_fn(...) } }`
- Caller syntax: `unsafe { raw_fn(...) }` — explicit opt-in at call site

## OCaml Approach

OCaml has no `unsafe fn` concept — all functions are safe by default. Preconditions are documented via comments and enforced by convention:

```ocaml
(** [raw_copy src dst n]
    Precondition: src and dst have at least n bytes; they must not overlap. *)
let raw_copy src dst n = Bytes.blit src 0 dst 0 n
```

The compiler does not enforce these preconditions — they are purely documentary.

## Key Differences

1. **Compile-time enforcement**: Rust `unsafe fn` makes the caller's opt-in explicit at the call site; OCaml preconditions are purely documentary with no enforcement.
2. **Audit tool**: `cargo audit` and `cargo geiger` can count `unsafe fn` call sites; there is no OCaml equivalent.
3. **API design**: Rust APIs prefer safe wrappers hiding `unsafe fn`; OCaml APIs expose functions with documented preconditions.
4. **Standard library**: Rust std has many `unsafe fn` with safe wrappers (e.g., `String::from_utf8_unchecked` vs `from_utf8`); OCaml stdlib validates inputs in safe functions.

## Exercises

1. **Write an unsafe fn**: Implement `unsafe fn get_unchecked(slice: &[i32], idx: usize) -> i32` and document its precondition — then write a safe wrapper that validates the index.
2. **# Safety audit**: For `std::slice::from_raw_parts`, read the documentation and list all preconditions — then write a safe wrapper that validates as many as possible.
3. **Precondition encoding**: Redesign `raw_copy` to accept `NonNull<u8>` instead of `*const u8`/`*mut u8` — how does this change the preconditions and safety documentation?
