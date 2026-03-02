# Comparison: Last Two Elements

## OCaml

```ocaml
let rec last_two = function
  | [] | [_] -> None
  | [x; y] -> Some (x, y)
  | _ :: t -> last_two t
```

## Rust — Idiomatic (direct indexing)

```rust
pub fn last_two<T>(slice: &[T]) -> Option<(&T, &T)> {
    let len = slice.len();
    if len < 2 { None }
    else { Some((&slice[len - 2], &slice[len - 1])) }
}
```

## Rust — Functional (recursive)

```rust
pub fn last_two_recursive<T>(slice: &[T]) -> Option<(&T, &T)> {
    match slice {
        [] | [_] => None,
        [x, y] => Some((x, y)),
        [_, rest @ ..] => last_two_recursive(rest),
    }
}
```

## Rust — Iterator (windows)

```rust
pub fn last_two_windows<T>(slice: &[T]) -> Option<(&T, &T)> {
    slice.windows(2).last().map(|w| (&w[0], &w[1]))
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Data structure | `'a list` (linked list) | `&[T]` (slice / contiguous) |
| Return type | `('a * 'a) option` | `Option<(&T, &T)>` |
| Ownership | Copies values into tuple | Borrows references |
| Pattern syntax | `[x; y]` | `[x, y]` |
| Complexity | O(n) always | O(1) idiomatic, O(n) recursive |

## Type Signatures

- **OCaml:** `val last_two : 'a list -> ('a * 'a) option`
- **Rust:** `fn last_two<T>(slice: &[T]) -> Option<(&T, &T)>`

## Takeaways

1. Rust's contiguous memory layout enables O(1) access to the last two elements — OCaml's linked list requires full traversal
2. Rust returns borrowed references (`&T`) instead of copies, avoiding allocation for large types
3. Slice pattern matching in Rust (`[x, y]`, `[_, rest @ ..]`) closely mirrors OCaml's list patterns
4. The `windows()` iterator is uniquely Rust — it exploits contiguous memory for sliding-window views
5. Both languages use `Option`/`option` to safely handle the "not enough elements" case without exceptions
