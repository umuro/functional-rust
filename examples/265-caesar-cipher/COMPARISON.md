# OCaml vs Rust: Caesar Cipher — Functional Encryption

## Side-by-Side Code

### OCaml
```ocaml
let shift_char n c =
  if c >= 'a' && c <= 'z' then
    Char.chr ((Char.code c - Char.code 'a' + n) mod 26 + Char.code 'a')
  else if c >= 'A' && c <= 'Z' then
    Char.chr ((Char.code c - Char.code 'A' + n) mod 26 + Char.code 'A')
  else c

let caesar n s = String.map (shift_char n) s
let decrypt n = caesar (26 - n)
```

### Rust (idiomatic)
```rust
fn shift_char(n: u8, c: char) -> char {
    match c {
        'a'..='z' => ((c as u8 - b'a' + n) % 26 + b'a') as char,
        'A'..='Z' => ((c as u8 - b'A' + n) % 26 + b'A') as char,
        _ => c,
    }
}

pub fn caesar(n: u8, s: &str) -> String {
    s.chars().map(|c| shift_char(n, c)).collect()
}

pub fn decrypt(n: u8, s: &str) -> String {
    caesar(26 - (n % 26), s)
}
```

### Rust (iterator-based — lazy)
```rust
pub fn caesar_iter(n: u8, s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars().map(move |c| shift_char(n, c))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Shift char | `val shift_char : int -> char -> char` | `fn shift_char(n: u8, c: char) -> char` |
| Encrypt | `val caesar : int -> string -> string` | `fn caesar(n: u8, s: &str) -> String` |
| Decrypt | `val decrypt : int -> string -> string` | `fn decrypt(n: u8, s: &str) -> String` |
| Char type | `char` (8-bit) | `char` (32-bit Unicode scalar) |
| String type | `string` (byte sequence) | `&str` (UTF-8 borrowed) / `String` (owned) |

## Key Insights

1. **Range patterns are elegant:** Rust's `'a'..='z'` in match arms is cleaner than OCaml's `if c >= 'a' && c <= 'z'` — pattern matching at its best
2. **Byte literals:** Rust's `b'a'` is equivalent to OCaml's `Char.code 'a'` — both give the numeric value of the character
3. **String ownership:** OCaml's `String.map` returns a new string (strings are mutable but `map` creates new); Rust borrows `&str` and returns owned `String`
4. **Lazy iteration:** Rust's `impl Iterator` approach delays computation; OCaml would need `Seq` for the same laziness
5. **Type safety on shift:** Rust uses `u8` for the shift amount, preventing negative shifts at the type level; OCaml uses `int` which could be negative (handled by `mod`)

## When to Use Each Style

**Use eager collection (`caesar`) when:** you need the full encrypted string as a `String` for storage or further processing  
**Use lazy iteration (`caesar_iter`) when:** you're chaining with other transformations or writing to a stream character-by-character
