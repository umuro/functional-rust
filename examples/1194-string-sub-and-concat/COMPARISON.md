# OCaml vs Rust: String.sub and String.concat

## Side-by-Side Code

### OCaml
```ocaml
let s = "Hello, World!"
let hello = String.sub s 0 5       (* "Hello" — allocates new string *)
let world = String.sub s 7 5       (* "World" — allocates new string *)

let parts = ["one"; "two"; "three"]
let joined = String.concat " | " parts  (* "one | two | three" *)
```

### Rust (idiomatic)
```rust
let s = "Hello, World!";
let hello = &s[0..5];    // "Hello" — zero-cost borrow, no allocation
let world = &s[7..12];   // "World" — zero-cost borrow, no allocation

let parts = ["one", "two", "three"];
let joined = parts.join(" | ");  // "one | two | three"
```

### Rust (safe / Option-based)
```rust
pub fn substring_safe(s: &str, start: usize, len: usize) -> Option<&str> {
    s.get(start..start + len)
}

// Instead of catching Invalid_argument, callers pattern-match on None:
match substring_safe("Hello", 3, 10) {
    Some(sub) => println!("{}", sub),
    None      => println!("out of bounds"),
}
```

### Rust (fold-based join, mirrors OCaml's List.fold_left)
```rust
pub fn join_fold(parts: &[&str], sep: &str) -> String {
    parts
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (i, part)| {
            if i > 0 { acc.push_str(sep); }
            acc.push_str(part);
            acc
        })
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Substring (unsafe) | `val sub : string -> int -> int -> string` | `fn substring(s: &str, start: usize, len: usize) -> &str` |
| Substring (safe) | `(raises Invalid_argument)` | `fn substring_safe(s: &str, start: usize, len: usize) -> Option<&str>` |
| Join | `val concat : string -> string list -> string` | `fn join(parts: &[&str], sep: &str) -> String` |
| Slice view | `string` (always owned) | `&str` (borrowed, zero-cost) |
| Owned string | `string` (always) | `String` (heap-allocated, mutable) |

## Key Insights

1. **Zero-cost slicing:** OCaml `String.sub` always allocates a new string. Rust `&s[start..end]` produces a `&str` that points into the original buffer — no allocation, no copy. This is one of Rust's biggest advantages for string processing.

2. **Error discipline:** OCaml raises `Invalid_argument` on out-of-bounds access; you either catch it with `try ... with` or let it crash. Rust gives you a choice: panic immediately (`&s[..]`) or handle gracefully (`s.get(..)` returning `Option<&str>`). The safe variant forces callers to handle the error at compile time.

3. **`join` vs `concat`:** OCaml's `String.concat sep list` takes the separator first, then the list. Rust's `.join(sep)` is a method on `&[&str]` — it reads left-to-right and is often more ergonomic in a pipeline. Both allocate exactly once.

4. **`&str` vs `String`:** OCaml has one string type (immutable, reference-counted or copied). Rust has two: `&str` (borrowed, read-only view) and `String` (owned, growable). Substring operations return `&str`; join returns `String` because it must allocate new memory to combine parts.

5. **Fold-based join mirrors OCaml style:** OCaml programmers often reach for `List.fold_left` to build strings. The Rust equivalent with `.fold(String::new(), ...)` works but is less efficient than `.join()` (which can pre-compute the total length). Use `.join()` in production; use fold when teaching functional patterns.

## When to Use Each Style

**Use idiomatic Rust (`&s[..]` and `.join()`) when:** you want maximum performance with clear, readable code. These are zero-allocation where possible and idiomatically Rust.

**Use `s.get(start..end)` when:** the range may be invalid and you want to handle it gracefully without panicking — e.g., parsing untrusted input.

**Use fold-based join when:** you're building a string incrementally with non-uniform separators or conditionally including elements, where `.join()` does not fit.
