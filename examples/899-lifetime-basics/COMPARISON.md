# OCaml vs Rust: Lifetime Basics

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml's GC ensures values live as long as they're referenced.
   No annotations needed — the runtime tracks everything. *)

let longest a b =
  if String.length a >= String.length b then a else b

let () =
  let result = longest "long string" "short" in
  assert (result = "long string");
  print_endline "ok"
```

### Rust (idiomatic — explicit lifetime annotation)
```rust
// 'a names the overlap of both input lifetimes.
// The returned reference cannot outlive either argument.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

### Rust (struct holding a reference)
```rust
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}
```

### Rust (functional — longest in a slice)
```rust
pub fn longest_in<'a>(strs: &[&'a str]) -> Option<&'a str> {
    strs.iter().copied().max_by_key(|s| s.len())
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| String comparison function | `val longest : string -> string -> string` | `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` |
| Owned vs borrowed | Always GC-managed | `String` (owned) vs `&str` (borrowed) |
| Struct with reference | Any field, GC handles it | Requires `'a` on struct and `impl` block |
| Optional reference | `string option` | `Option<&'a str>` |

## Key Insights

1. **OCaml never needs annotations** — the garbage collector tracks all live references at runtime, making dangling pointers impossible without any programmer effort.

2. **Rust does it at compile time** — lifetime annotations (`'a`) are not runtime metadata; they are hints to the borrow checker that are erased before codegen. Zero overhead.

3. **Lifetime elision hides the noise** — most single-input functions (`fn first_word(&self) -> &str`) don't require explicit annotations; the compiler infers them via three elision rules.

4. **Structs that hold references must be annotated** — `Excerpt<'a>` tells the compiler "this struct cannot outlive the string slice it borrows," preventing use-after-free at the call site.

5. **The annotation describes relationships, not durations** — `'a` on `longest` doesn't say "live for exactly this long"; it says "the output reference comes from one of the inputs," letting the caller reason about scope.

## When to Use Each Style

**Use explicit lifetime annotations when:** a function takes multiple reference parameters and returns a reference — the compiler cannot infer which input the output borrows from.

**Use lifetime elision (no annotation) when:** a function takes exactly one reference parameter and returns a reference from it, or the function is a `&self` method returning a reference (the compiler handles these automatically).
