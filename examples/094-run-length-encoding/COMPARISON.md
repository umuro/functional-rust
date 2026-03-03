# Comparison: Run-Length Encoding — OCaml vs Rust

## Core Insight

Both languages build the result string incrementally. OCaml's `Buffer` module is the explicit choice for this; Rust's `String` is inherently a growable UTF-8 buffer. The recursive vs iterative style differs, but the core algorithm — tracking the current character and its count — is identical.

## OCaml

```ocaml
let encode s =
  let buf = Buffer.create (String.length s) in
  let rec go i c count =
    if i = String.length s then begin
      if count > 1 then Buffer.add_string buf (string_of_int count);
      Buffer.add_char buf c end
    else if s.[i] = c then go (i+1) c (count+1)
    else begin
      if count > 1 then Buffer.add_string buf (string_of_int count);
      Buffer.add_char buf c; go (i+1) s.[i] 1 end
  in go 1 s.[0] 1; Buffer.contents buf
```

## Rust — Fold-based

```rust
pub fn encode_fold(s: &str) -> String {
    s.chars()
        .fold(Vec::<(char, usize)>::new(), |mut acc, c| {
            match acc.last_mut() {
                Some((last, count)) if *last == c => *count += 1,
                _ => acc.push((c, 1)),
            }; acc
        })
        .iter()
        .map(|&(c, n)| if n > 1 { format!("{n}{c}") } else { c.to_string() })
        .collect()
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| String builder | `Buffer.create n` | `String::new()` / `String::with_capacity` |
| Char access | `s.[i]` | `chars[i]` after collect |
| Int to string | `string_of_int count` | `count.to_string()` or `format!` |
| Grouping | Recursive with acc | `fold` with `last_mut()` |
| String concat | `Buffer.add_string` | `push_str` / `push` |

## Learner Notes

- **`last_mut()`**: Rust lets you mutably borrow the last vector element — perfect for accumulating groups
- **No `chunk_by` in stable**: Rust nightly has `slice::chunk_by`, but stable requires manual grouping
- **`format!` cost**: Each `format!("{n}{c}")` allocates; for hot paths, use `write!` into a single `String`
- **OCaml's Buffer**: Similar to Java's `StringBuilder` — explicit mutable string building in an otherwise immutable world
