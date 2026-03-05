# OCaml vs Rust: String — Trim, Uppercase, Contains

## Side-by-Side Code

### OCaml

```ocaml
let s = "  Hello, World!  "
let trimmed = String.trim s
let upper   = String.uppercase_ascii trimmed
let lower   = String.lowercase_ascii trimmed

(* OCaml stdlib has no String.contains for substrings;
   idiomatic solution is a hand-written recursive search *)
let rec find s needle i =
  if i > String.length s - String.length needle then false
  else if String.sub s i (String.length needle) = needle then true
  else find s needle (i + 1)

let has_world = find s "World" 0
```

### Rust (idiomatic)

```rust
pub fn trim_and_upper(s: &str) -> String {
    s.trim().to_uppercase()
}

pub fn trim_and_lower(s: &str) -> String {
    s.trim().to_lowercase()
}

pub fn contains_substring(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}
```

### Rust (functional/recursive — mirrors OCaml)

```rust
pub fn contains_recursive(s: &str, needle: &str) -> bool {
    fn find(s: &str, needle: &str, i: usize) -> bool {
        if i + needle.len() > s.len() {
            false
        } else if s[i..].starts_with(needle) {
            true
        } else {
            find(s, needle, i + 1)
        }
    }
    find(s, needle, 0)
}
```

### Rust (iterator window)

```rust
pub fn contains_windowed(s: &str, needle: &str) -> bool {
    let n = needle.len();
    if n == 0 { return true; }
    (0..=s.len().saturating_sub(n)).any(|i| s[i..].starts_with(needle))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Trim | `String.trim : string -> string` | `str::trim(&self) -> &str` |
| Uppercase | `String.uppercase_ascii : string -> string` | `str::to_uppercase(&self) -> String` |
| Lowercase | `String.lowercase_ascii : string -> string` | `str::to_lowercase(&self) -> String` |
| Substring search | manual recursion | `str::contains<P: Pattern>(&self, pat: P) -> bool` |
| String slice type | `string` (heap, mutable) | `&str` (borrowed slice, immutable view) |

## Key Insights

1. **`trim()` is zero-copy in Rust.** `str::trim()` returns `&str` — a slice into
   the original buffer. No heap allocation occurs until `to_uppercase()` is called.
   OCaml's `String.trim` always allocates a fresh string.

2. **Case conversion allocates.** Both languages produce a new string for
   `uppercase`/`lowercase`. Rust names are `to_uppercase` / `to_lowercase`; OCaml
   appends `_ascii` to signal ASCII-only handling. Rust's versions are Unicode-aware
   by default.

3. **OCaml has no `String.contains` for substrings.** The standard library only
   offers index-based functions like `String.sub`, so OCaml developers write explicit
   recursive search. Rust's `str::contains` accepts a flexible `Pattern` — a `&str`,
   a `char`, or a closure — making it vastly more ergonomic.

4. **Nested `fn` mirrors OCaml's inner `let rec`.** The recursive Rust solution uses
   a nested `fn find(...)` inside the public function body, exactly mirroring OCaml's
   `let rec find ... in find 0`. Both compilers can optimise tail calls; Rust does not
   guarantee TCO but the depth here is bounded by string length.

5. **Iterator `.any()` replaces explicit recursion.** The windowed variant expresses
   the same search as a range + closure — idiomatic Rust prefers this over raw index
   arithmetic whenever the logic is simple enough to fit in a one-liner.

## When to Use Each Style

**Use idiomatic Rust (`contains`) when:** searching for a fixed substring or
character in production code — it is optimised, Unicode-correct, and reads as intent.

**Use recursive Rust when:** translating OCaml algorithms directly or teaching the
structural equivalence between ML-style recursion and Rust tail calls.

**Use iterator windowing when:** the search condition is a closure or you need to
collect matching positions rather than just a boolean.
