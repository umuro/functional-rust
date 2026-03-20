📖 **[View on hightechmind.io →](https://hightechmind.io/rust/369-cow-clone-on-write)**

---

# 369: Clone-on-Write (Cow)

## Problem Statement

String processing functions often receive data that usually needs no modification — normalizing whitespace, sanitizing identifiers, trimming. Returning `String` always allocates even when the input is already valid. Returning `&str` requires the caller to own the buffer. Rust's `Cow<'a, str>` (Clone-on-Write) solves the dilemma: it holds either a borrowed reference (`Cow::Borrowed(&str)`) or an owned string (`Cow::Owned(String)`) and deref-transparently exposes `&str` in both cases. Allocation happens only when the data actually needs modification. This pattern appears in serde deserialization, HTTP header parsing, and any API that wants to avoid unnecessary copying.

## Learning Outcomes

- Use `Cow<'a, str>` to return borrowed data when no modification is needed
- Return `Cow::Borrowed(s)` when the input is already valid — zero allocation
- Return `Cow::Owned(s.to_string())` or `Cow::Owned(s.replace(...))` when modification is needed
- Use `Cow` as a function parameter to accept both `&str` and `String` ergonomically
- Understand that `Cow<'a, B>` works for any `B: ToOwned` (slices, paths, etc.)
- Recognize the performance benefit: O(1) for the common no-modification case

## Rust Application

```rust
use std::borrow::Cow;

pub fn ensure_no_spaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_")) // only allocates when spaces found
    } else {
        Cow::Borrowed(s) // zero-copy: returns a view into the original
    }
}

pub fn truncate_to_limit(s: &str, limit: usize) -> Cow<str> {
    if s.len() <= limit {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s[..limit].to_string())
    }
}

// Usage: callers don't need to know which variant was returned
let result = ensure_no_spaces("hello world");
println!("{}", result); // works via Deref<Target = str>

let result2 = ensure_no_spaces("no_spaces_here");
// result2 is Borrowed — no allocation occurred
```

`Cow` implements `Deref<Target = str>`, so you can use it anywhere `&str` is expected. If you need to mutate the `Cow`, `cow.to_mut()` will clone the borrowed data into an owned `String` at that point — true copy-on-write semantics.

## OCaml Approach

OCaml's immutable strings sidestep this problem: you can't mutate a string, so borrowed vs owned is irrelevant:

```ocaml
let ensure_no_spaces s =
  if String.contains s ' '
  then String.map (fun c -> if c = ' ' then '_' else c) s
  else s  (* return original — GC manages the reference *)

let truncate_to_limit s limit =
  if String.length s <= limit then s
  else String.sub s 0 limit
```

In OCaml, `s` is returned directly without a wrapper type — the GC knows both the caller and callee share the same string object. There's no distinction between "borrowed" and "owned" at the type level; the GC handles lifetime tracking automatically.

## Key Differences

| Aspect | Rust `Cow<'a, str>` | OCaml `string` |
|--------|--------------------|-----------------|
| Zero-copy path | `Cow::Borrowed(s)` | Direct return (GC-tracked) |
| Allocation path | `Cow::Owned(...)` | New string allocation |
| Lifetime tracking | `'a` lifetime parameter | GC |
| Mutation | `cow.to_mut()` triggers clone | N/A (strings immutable) |
| Transparency | `Deref<Target = str>` | Direct `string` value |

## Exercises

1. **HTML escape**: Implement `html_escape<'a>(s: &'a str) -> Cow<'a, str>` that replaces `<`, `>`, `&` with their HTML entities — borrow the original if none are present, allocate a new string only when needed.
2. **Path normalization**: Use `Cow<'_, Path>` to normalize a file path: if it's already absolute and clean, borrow it; if it needs `canonicalize()`, return `Cow::Owned`.
3. **to_mut demo**: Create a `Cow::Borrowed` from a string, then call `.to_mut()` to get a mutable reference; verify that the original slice is unchanged and the `Cow` now holds an owned copy.
