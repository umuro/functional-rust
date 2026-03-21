📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1046-cow-collections)**

---

# 1046-cow-collections — Clone-on-Write Collections
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Many operations on collections are read-only most of the time: a configuration lookup, a data validation pass, or a rendering pipeline. Cloning the entire collection for a rare mutation is wasteful. Clone-on-write (Cow) defers allocation until mutation is actually needed, returning a borrowed reference for the common read-only path.

Rust's `Cow<'_, [T]>` and `Cow<'_, str>` implement this: `Borrowed` holds a reference with zero allocation, `Owned` holds a fully-owned copy. The transition from borrowed to owned happens lazily via `.to_mut()`.

## Learning Outcomes

- Use `Cow::Borrowed` for zero-allocation read-only access to slices and strings
- Trigger `Cow::Owned` allocation only when mutation is needed
- Use `to_mut()` for lazy clone-on-first-write
- Recognize `Cow` in standard library APIs (`String::from_utf8_lossy`)
- Understand when `Cow` is appropriate vs always cloning

## Rust Application

`src/lib.rs` demonstrates `process_data(data: &[i32], threshold: i32) -> Cow<'_, [i32]>`: when all values are within threshold, returns `Cow::Borrowed(data)` — no allocation. When some values exceed the threshold, returns `Cow::Owned(...)` with clamped values. `normalize_name` returns `Cow::Borrowed(s)` when the name is already lowercase, allocating only when case conversion is needed.

`String::from_utf8_lossy` in the standard library uses `Cow<'_, str>` — returning `Borrowed` for valid UTF-8 and `Owned` (with replacement characters) only for invalid sequences.

## OCaml Approach

OCaml's GC + structural sharing achieves copy-on-write semantics naturally for immutable structures:

```ocaml
(* No mutation needed — structural sharing is automatic *)
let filter_evens lst = List.filter (fun x -> x mod 2 = 0) lst
(* If input is already filtered, no benefit — always rebuilds *)
```

For mutable structures, OCaml's `Bytes` has explicit `Bytes.copy` for explicit copying. The Cow pattern is less necessary in OCaml because immutable structures share memory automatically via the GC.

## Key Differences

1. **Explicit vs implicit**: Rust's `Cow` makes the borrow-or-own decision explicit in the type; OCaml's GC sharing is implicit and applies only to immutable values.
2. **Lifetime tracking**: Rust's `Cow<'_, [T]>` carries a lifetime annotation; OCaml has no equivalent.
3. **String processing**: Rust's `Cow<'_, str>` is common for string normalization; OCaml's strings are bytes and lack direct `Cow` equivalents.
4. **`to_mut()`**: Rust's `Cow::to_mut()` triggers a clone on first call, then returns `&mut T` for subsequent mutations; OCaml has no equivalent lazy promotion.

## Exercises

1. Write a `sanitize_html(s: &str) -> Cow<'_, str>` function that returns `Borrowed` if no HTML characters are present and `Owned` only when replacing `<`, `>`, `&`.
2. Implement `deduplicate(data: &[i32]) -> Cow<'_, [i32]>` that returns `Borrowed` if the slice has no duplicates and `Owned` with duplicates removed otherwise.
3. Build a cache-aware normalizer that takes `Cow<'_, str>` as input and avoids cloning when the input is already normalized.
