📖 **[View on hightechmind.io →](https://hightechmind.io/rust/112-cow-clone-on-write)**

---

# 112-cow-clone-on-write — Cow<T>: Clone on Write
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

String normalization, data sanitization, and text transformation often leave the majority of inputs unchanged. Cloning the entire string just to return it unmodified wastes memory and time. `Cow<'a, str>` (Clone-on-Write) solves this by returning a borrowed reference when no change is needed and allocating only when a modification is required.

`std::borrow::Cow` is used throughout the standard library — `String::from_utf8_lossy`, `Path::to_string_lossy`, and environment variable reading all use `Cow` to avoid unnecessary allocations.

## Learning Outcomes

- Use `Cow::Borrowed(&'a str)` for zero-allocation pass-through
- Use `Cow::Owned(String)` when allocation is required
- Use `to_mut()` to lazily trigger a clone on first mutation
- Recognize `Cow` in standard library APIs
- Apply `Cow<[T]>` to slice processing

## Rust Application

`src/lib.rs` implements three functions. `normalize_whitespace` returns `Borrowed(s)` if no tabs are present — no allocation — and `Owned(s.replace('	', " "))` when tabs need replacing. `strip_prefix_cow` returns `Owned` only when the prefix is present (the result is a new string without the prefix). `ensure_sorted` returns `Borrowed(v)` if the slice is already sorted, allocating only to sort an unsorted input.

These patterns reflect real usage: `String::from_utf8_lossy` in the standard library does exactly this — `Borrowed` for valid UTF-8, `Owned` (with replacement chars) for invalid sequences.

## OCaml Approach

OCaml's GC and structural sharing for immutable values provide similar behavior naturally — sharing is automatic. For string processing, OCaml would either always allocate or use explicit optional return:

```ocaml
let normalize_whitespace s =
  if String.contains s '	' then
    String.map (fun c -> if c = '	' then ' ' else c) s  (* allocates *)
  else s  (* returns same string — GC sharing, no allocation *)
```

OCaml strings are immutable since 4.06, so returning `s` directly shares the original without copying. The GC tracks the reference — equivalent to `Cow::Borrowed` but without the explicit type.

## Key Differences

1. **Explicit vs implicit**: Rust's `Cow` makes the borrow-or-own decision visible in the type; OCaml returns the same string reference implicitly.
2. **`to_mut()` lazy clone**: Rust's `Cow::to_mut()` triggers a clone on first call and then provides `&mut T`; OCaml has no equivalent for mutable promotion.
3. **Type annotation**: `-> Cow<'_, str>` in the return type signals to callers whether allocation might occur; OCaml's `string` return type is opaque.
4. **Slice variant**: `Cow<'_, [T]>` works the same way for slices; OCaml's list/array returns handle this via GC sharing.

## Exercises

1. Write `escape_html(s: &str) -> Cow<'_, str>` that escapes only `<`, `>`, and `&`, returning `Borrowed` when no escaping is needed.
2. Implement `trim_ascii_whitespace(s: &str) -> Cow<'_, str>` that returns `Borrowed` when no trimming is needed.
3. Write a function pipeline that chains multiple `Cow`-returning functions while avoiding unnecessary allocations: normalize → trim → escape.
