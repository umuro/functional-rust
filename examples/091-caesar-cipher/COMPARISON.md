# Comparison: Caesar Cipher — OCaml vs Rust

## Core Insight

OCaml's `String.map` is a single function call that transforms every character. Rust requires the iterator chain `chars().map(f).collect()` — more explicit but equally expressive. The key difference is that Rust distinguishes `char` (Unicode scalar) from `u8` (byte), giving you a choice between correctness and performance for ASCII workloads.

## OCaml

```ocaml
let shift_char n c =
  if c >= 'a' && c <= 'z' then
    Char.chr ((Char.code c - Char.code 'a' + n) mod 26 + Char.code 'a')
  else c

let caesar n s = String.map (shift_char n) s
```

## Rust — Idiomatic

```rust
pub fn caesar(n: u8, s: &str) -> String {
    s.chars().map(|c| shift_char(n, c)).collect()
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| String map | `String.map f s` | `s.chars().map(f).collect()` |
| Char to int | `Char.code c` | `c as u8` |
| Int to char | `Char.chr n` | `n as char` |
| Range check | `c >= 'a' && c <= 'z'` | `'a'..='z'` pattern |
| Byte access | `s.[i]` | `s.as_bytes()[i]` |
| Mutability | Strings immutable | Can mutate byte vec |

## Learner Notes

- **Range patterns**: Rust's `'a'..='z'` in match arms is cleaner than OCaml's boolean comparisons
- **Byte optimization**: For ASCII-only ciphers, `as_bytes().to_vec()` avoids UTF-8 overhead
- **Type safety**: Rust's `u8` arithmetic catches overflow at compile time with checked ops
- **No `String.map`**: Rust deliberately omits it because strings are UTF-8; the iterator chain makes the per-char cost visible
