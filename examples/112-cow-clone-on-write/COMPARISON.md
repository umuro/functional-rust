# OCaml vs Rust: Cow<T> — Clone on Write

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml's GC handles sharing automatically — no explicit CoW needed.
   The closest idiom is a conditional copy: return the input unchanged
   when no modification is required, otherwise build a new value. *)

let normalize_whitespace s =
  if String.contains s '\t' then
    String.map (fun c -> if c = '\t' then ' ' else c) s
  else
    s  (* no allocation: same string object *)

let escape_html s =
  let needs_escape = String.exists (fun c ->
    c = '<' || c = '>' || c = '&' || c = '"') s in
  if needs_escape then
    (* allocate only when needed *)
    let buf = Buffer.create (String.length s) in
    String.iter (fun c ->
      Buffer.add_string buf (match c with
        | '<' -> "&lt;" | '>' -> "&gt;"
        | '&' -> "&amp;" | '"' -> "&quot;"
        | c   -> String.make 1 c)) s;
    Buffer.contents buf
  else
    s
```

### Rust (idiomatic — `Cow<str>`)
```rust
use std::borrow::Cow;

fn normalize_whitespace(s: &str) -> Cow<str> {
    if s.contains('\t') {
        Cow::Owned(s.replace('\t', " "))   // allocates only here
    } else {
        Cow::Borrowed(s)                    // zero-cost borrow
    }
}
```

### Rust (functional — `ensure_sorted` with `Cow<[T]>`)
```rust
use std::borrow::Cow;

fn ensure_sorted(v: &[i32]) -> Cow<[i32]> {
    if v.windows(2).all(|w| w[0] <= w[1]) {
        Cow::Borrowed(v)           // already sorted — share the slice
    } else {
        let mut owned = v.to_vec();
        owned.sort();
        Cow::Owned(owned)          // cloned only when reordering is needed
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| String type | `string` (immutable) | `&str` (borrowed) / `String` (owned) |
| "Maybe owned" | no built-in — use `string` (GC shares) | `Cow<'a, str>` |
| Slice type | `'a array` | `&[T]` / `Vec<T>` |
| "Maybe owned" slice | no built-in | `Cow<'a, [T]>` |
| Deref to common type | automatic (GC reference) | `Deref` trait → `&str` / `&[T]` |

## Key Insights

1. **No allocation by default.** In OCaml the GC transparently shares immutable values, so "copy-on-write" is free. In Rust there is no GC; `Cow` gives you the same guarantee explicitly — `Borrowed` variant costs nothing.

2. **Explicit variant, uniform API.** `Cow` is a plain `enum` (`Borrowed` / `Owned`). You `Deref` it to `&str` or `&[T]` in both cases, so callers never have to match on which variant they received.

3. **`to_mut()` triggers the lazy clone.** If you hold a `Cow::Borrowed` and call `to_mut()`, Rust clones the data into `Cow::Owned` at that moment — and only at that moment. Subsequent `to_mut()` calls on the same `Owned` value are free.

4. **Lifetime tracking.** `Cow<'a, T>` carries a lifetime `'a` that ties the `Borrowed` variant to its source. The compiler enforces that the borrow stays valid, something OCaml's GC handles invisibly.

5. **Zero-cost abstraction.** When the `Borrowed` path is taken, the entire `Cow` compiles down to a fat pointer — identical to returning `&str` directly. There is no runtime overhead from the enum wrapper on the common, unmodified path.

## When to Use Each Style

**Use `Cow::Borrowed` (return the borrow) when:** the data requires no transformation in the common case and you want to avoid heap allocation entirely.

**Use `Cow::Owned` (allocate) when:** the function must modify the data — e.g. replacing characters, sorting, escaping — and the original is no longer sufficient.

**Use `to_mut()` when:** you start with a borrowed slice but may need to mutate it in-place; `to_mut()` lazily promotes `Borrowed` to `Owned` exactly once.
