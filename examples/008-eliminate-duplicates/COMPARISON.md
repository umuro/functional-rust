# Comparison: Eliminate Consecutive Duplicates

## OCaml — Pattern matching

```ocaml
let rec compress = function
  | [] -> []
  | [x] -> [x]
  | h1 :: (h2 :: _ as t) ->
      if h1 = h2 then compress t
      else h1 :: compress t
```

## Rust — Idiomatic (in-place mutation)

```rust
pub fn compress_mut<T: PartialEq>(list: &mut Vec<T>) {
    list.dedup();
}
```

## Rust — Functional (accumulator)

```rust
pub fn compress<T: PartialEq + Clone>(list: &[T]) -> Vec<T> {
    if list.is_empty() { return vec![]; }
    let mut result = vec![list[0].clone()];
    for item in &list[1..] {
        if result.last() != Some(item) {
            result.push(item.clone());
        }
    }
    result
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Pattern | `h1 :: h2 :: _` (cons cell) | `&list[1..]` (slice) |
| Mutation | Not available on lists | `Vec::dedup()` in-place |
| Return | New list (GC frees old) | New `Vec` or mutate existing |
| Equality | `=` (polymorphic) | `PartialEq` trait |
| Edge cases | Pattern match `[]`, `[x]` | `is_empty()` check |

## Takeaways

1. Rust's `dedup()` is a one-liner for the imperative approach — stdlib is batteries-included
2. OCaml's pattern matching on cons cells is more elegant for list algorithms
3. The `windows(2)` approach has no OCaml equivalent — it leverages contiguous memory layout
4. Mutation in Rust is always a conscious choice (`&mut`), never accidental
5. Both languages handle edge cases (empty, single-element) but Rust uses conditionals where OCaml uses pattern arms
