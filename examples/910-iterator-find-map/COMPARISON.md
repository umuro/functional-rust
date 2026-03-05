# OCaml vs Rust: Transform-and-Find with find_map()

## Side-by-Side Code

### OCaml
```ocaml
let find_map f lst =
  let rec aux = function
    | [] -> None
    | x :: xs -> (match f x with Some _ as r -> r | None -> aux xs)
  in aux lst

let () =
  let strings = ["hello"; "42"; "world"; "17"; "foo"] in
  let first_int = find_map int_of_string_opt strings in
  Printf.printf "First int: %s\n"
    (match first_int with Some n -> string_of_int n | None -> "None")
```

### Rust (idiomatic)
```rust
pub fn first_int(strings: &[&str]) -> Option<i32> {
    strings.iter().find_map(|s| s.parse::<i32>().ok())
}
```

### Rust (functional/recursive — mirrors OCaml)
```rust
pub fn find_map_rec<T, B, F>(list: &[T], f: F) -> Option<B>
where
    F: Fn(&T) -> Option<B>,
{
    match list {
        [] => None,
        [head, tail @ ..] => f(head).or_else(|| find_map_rec(tail, f)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| find_map | `val find_map : ('a -> 'b option) -> 'a list -> 'b option` | `fn find_map<B, F: Fn(&T) -> Option<B>>(&[T], F) -> Option<B>` |
| List type | `'a list` | `&[T]` (slice) |
| Optional value | `'a option` | `Option<B>` |
| Parse int | `int_of_string_opt : string -> int option` | `str::parse::<i32>().ok()` |

## Key Insights

1. **Built-in vs library**: OCaml added `List.find_map` in 4.10; Rust's `Iterator::find_map` has been stable since 1.30 — it's the idiomatic standard.
2. **Closure returns `Option`**: The convention is identical — `None` means "skip", `Some(v)` means "stop and return `v`". Both languages encode early termination purely through the return type.
3. **Lazy evaluation**: Both implementations are short-circuit — once the first `Some` is found, remaining elements are never processed. Rust's iterator chain makes this explicit and zero-cost.
4. **`find_map` vs `filter_map().next()`**: In Rust, `iter.find_map(f)` is exactly `iter.filter_map(f).next()` but communicates intent more directly — you're searching, not building a collection.
5. **Recursive style**: The OCaml recursive pattern maps cleanly to Rust slice patterns (`[head, tail @ ..]`) with `.or_else()` replacing the `match` on the recursive call, keeping the functional structure intact.

## When to Use Each Style

**Use idiomatic Rust (`find_map`)** when scanning an iterator for the first successfully-transformed element — parsing, config lookup, file extension matching.
**Use recursive Rust** when teaching the OCaml parallel explicitly or when working with custom recursive data structures where the iterator API doesn't apply directly.
