# OCaml vs Rust: Lifetime Basics

## OCaml
```ocaml
(* No lifetimes needed — GC manages memory *)
let longer s1 s2 =
  if String.length s1 >= String.length s2 then s1 else s2

let first_word s =
  match String.split_on_char ' ' s with
  | [] -> ""
  | w :: _ -> w
```

## Rust
```rust
// Explicit lifetime: output valid while both inputs valid
pub fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() { s1 } else { s2 }
}

// Elided lifetime: compiler infers input → output
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```

## Key Differences

1. **OCaml**: GC tracks object reachability, no explicit lifetimes
2. **Rust**: Lifetimes express "how long is this reference valid?"
3. **Rust**: 'a is a lifetime parameter, like a generic type
4. **Rust**: Output lifetime tied to input lifetimes
5. Both: Prevent use-after-free, different mechanisms
