# OCaml vs Rust: Flattening with flat_map()

## Side-by-Side Code

### OCaml
```ocaml
(* List.concat_map : ('a -> 'b list) -> 'a list -> 'b list *)

(* Split sentences into words *)
let words = List.concat_map String.split_on_char ' ' ["hello world"; "foo bar"]

(* Expand each n into range 0..n-1 *)
let expanded = List.concat_map (fun n -> List.init n Fun.id) [1; 2; 3]
(* → [0; 0; 1; 0; 1; 2] *)

(* Parse, silently drop failures *)
let parsed = List.concat_map (fun s ->
  match int_of_string_opt s with
  | Some n -> [n * 2]
  | None -> []
) ["1"; "two"; "3"]
(* → [2; 6] *)
```

### Rust (idiomatic — iterator adapter)
```rust
// flat_map is lazy: no intermediate Vec allocated
let words: Vec<&str> = ["hello world", "foo bar"]
    .iter()
    .flat_map(|s| s.split_whitespace())
    .collect();
// → ["hello", "world", "foo", "bar"]

let expanded: Vec<i32> = [1i32, 2, 3]
    .iter()
    .flat_map(|&n| 0..n)
    .collect();
// → [0, 0, 1, 0, 1, 2]

// Result/Option implements IntoIterator — failures yield zero elements
let parsed: Vec<i32> = ["1", "two", "3"]
    .iter()
    .flat_map(|s| s.parse::<i32>().map(|n| n * 2))
    .collect();
// → [2, 6]
```

### Rust (explicit — map + flatten)
```rust
// flat_map(f) is exactly map(f).flatten()
let words: Vec<&str> = ["hello world", "foo bar"]
    .iter()
    .map(|s| s.split_whitespace())
    .flatten()
    .collect();
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function | `List.concat_map : ('a -> 'b list) -> 'a list -> 'b list` | `Iterator::flat_map<B, F>(self, f: F) -> FlatMap<…>` |
| Input element | `'a` | `Self::Item` |
| Output per element | `'b list` (eager `list`) | `impl IntoIterator<Item = B>` (any iterable, lazy) |
| Collect | implicit — returns a list | explicit `.collect::<Vec<_>>()` |
| Drop failures | `match … \| None -> []` | `Result`/`Option` implement `IntoIterator` — zero items on failure |

## Key Insights

1. **Laziness:** OCaml's `List.concat_map` builds intermediate lists eagerly; Rust's `flat_map` is a lazy adapter — nothing is allocated until `.collect()` is called, enabling efficient pipeline composition.

2. **Zero-output via type system:** In OCaml you explicitly return `[]` to emit nothing. In Rust, `Result` and `Option` implement `IntoIterator` with zero items on failure, so `flat_map(|s| s.parse::<i32>())` is both a filter and a transform — no explicit `match` needed.

3. **map + flatten identity:** `flat_map(f)` ≡ `map(f).flatten()` in both languages. Rust exposes this decomposition directly as two chainable adapters, making the monad bind law transparent.

4. **Any iterable output:** OCaml's `concat_map` requires a `list` return. Rust's `flat_map` accepts *any* `IntoIterator` — ranges (`0..n`), slices, `String::bytes()`, `str::split_whitespace()` — without wrapping in a `Vec` first.

5. **Monad bind:** `flat_map` is the `>>=` (bind) of the iterator/list monad. Both OCaml and Rust use it to sequence "one-to-many" transformations, but Rust's lazy evaluation makes it composable with the rest of the iterator ecosystem at zero allocation cost.

## When to Use Each Style

**Use `flat_map` (idiomatic)** when the mapping function naturally returns something iterable — a range, a split string, a parsed result — and you want a single flat output without intermediate allocations.

**Use `map` + `flatten` explicitly** when you already have a `Vec<Vec<T>>` or `Vec<Option<T>>` and simply need to flatten it; or when the two steps are clearer separated for readability.
