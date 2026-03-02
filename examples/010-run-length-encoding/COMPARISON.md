# Comparison: Run-Length Encoding

## OCaml — Pack then encode

```ocaml
let encode lst =
  List.map (fun group -> (List.length group, List.hd group)) (pack lst)
```

## Rust — Idiomatic (compose with pack)

```rust
pub fn encode<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    pack(list)
        .into_iter()
        .map(|group| (group.len(), group[0].clone()))
        .collect()
}
```

## Rust — Functional (single-pass fold)

```rust
pub fn encode_fold<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    list.iter().fold(Vec::new(), |mut acc, item| {
        match acc.last_mut() {
            Some((count, ref val)) if val == item => *count += 1,
            _ => acc.push((1, item.clone())),
        }
        acc
    })
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Tuple type | `int * 'a` | `(usize, T)` |
| Count type | `int` (signed) | `usize` (unsigned) |
| Composition | `List.map f (pack lst)` | `pack().into_iter().map().collect()` |
| Single-pass | Separate recursive function | `fold` with `last_mut()` |
| Ownership | GC handles pack result | `into_iter()` consumes packed groups |

## Type Signatures

- OCaml: `val encode : 'a list -> (int * 'a) list`
- Rust: `fn encode<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)>`

## Takeaways

1. Both languages naturally compose: pack first, then transform — functional composition at work
2. Rust's `into_iter()` consumes the intermediate packed groups — no waste
3. The fold version shows Rust can match OCaml's single-pass elegance with `last_mut()`
4. `usize` for counts is more semantically correct (counts can't be negative) than OCaml's `int`
5. The pattern of `match acc.last_mut()` is a Rust idiom for stateful folds — learn it well
