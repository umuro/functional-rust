📖 **[View on hightechmind.io →](https://hightechmind.io/rust/484-string-cow-usage)**

---

# Cow<str> Usage

`Cow<'a, str>` (Clone-on-Write) is a smart pointer that holds either a borrowed `&'a str` or an owned `String`, allocating only when mutation is actually needed — enabling zero-copy fast paths with a unified API.

## Problem Statement

Many string transformations are conditional: sanitise input that may or may not contain bad characters, uppercase a string that may already be uppercase, strip a prefix that may or may not be present. A function that always returns `String` always allocates even when the input is already correct. A function that always returns `&str` cannot return a modified string. `Cow<str>` resolves this: it carries the borrowed slice when no change is needed and the owned string when a change was made, with a single return type and zero overhead in the no-change case.

## Learning Outcomes

- Construct `Cow::Borrowed(s)` and `Cow::Owned(owned)` variants
- Return `Cow<'_, str>` from a function to avoid unnecessary allocation
- Use `matches!(val, Cow::Borrowed(_))` to verify no allocation occurred
- Deref `Cow<str>` to `&str` for use with any string-accepting API
- Chain `Cow` transformations without intermediate allocations

## Rust Application

`ensure_no_spaces` returns a borrowed slice when the input has no spaces, allocating only when replacement is needed:

```rust
fn ensure_no_spaces(s: &str) -> Cow<'_, str> {
    if !s.contains(' ') {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.replace(' ', "_"))
    }
}
```

The caller can pattern-match to verify allocation behaviour:

```rust
assert!(matches!(ensure_no_spaces("nospace"), Cow::Borrowed(_)));
assert!(matches!(ensure_no_spaces("has space"), Cow::Owned(_)));
```

Any `Cow<str>` derefs to `&str`, so standard string methods work uniformly via `Deref`.

## OCaml Approach

OCaml has no `Cow` equivalent in the standard library. The pattern is approximated with `Either` or by returning `option` (where `None` means "unchanged"):

```ocaml
let ensure_no_spaces s =
  if not (String.contains s ' ') then s  (* same pointer *)
  else String.map (fun c -> if c = ' ' then '_' else c) s

(* Caller cannot distinguish borrowed vs. allocated without pointer comparison *)
```

The `Slice_and_rope` library provides rope structures for efficient persistent string manipulation. `Stringext` provides `replace_all` utilities. There is no idiomatic zero-copy conditional allocation pattern.

## Key Differences

1. **Allocation transparency**: Rust's `Cow` makes allocation vs. borrow explicit in the type; OCaml's string functions always return a new string or the same pointer (without type-level distinction).
2. **Unified API**: `Cow<str>` derefs to `&str` so callers need no special handling; OCaml would need explicit `match` or an `option` type.
3. **Lifetime tracking**: `Cow<'a, str>` carries a lifetime; OCaml strings are GC-managed so lifetimes are irrelevant.
4. **`into_owned`**: Rust's `Cow::into_owned()` cheaply converts a borrowed cow to an owned `String` via `.to_string()`; OCaml has no equivalent operation with the same allocation transparency.

## Exercises

1. **Normalise whitespace**: Write `normalize_spaces(s: &str) -> Cow<str>` that collapses multiple spaces into one, returning `Borrowed` when the input already has no consecutive spaces.
2. **Escape HTML**: Write `escape_html(s: &str) -> Cow<str>` that returns `Borrowed(s)` when no `<`, `>`, `&`, or `"` are present, and `Owned(escaped)` otherwise.
3. **Chain Cows**: Implement a pipeline `trim → ensure_no_spaces → to_ascii_lowercase` where each step returns `Cow<str>` and measure the total number of allocations for inputs that require 0, 1, and 3 modifications.
