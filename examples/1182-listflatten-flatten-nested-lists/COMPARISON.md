# OCaml vs Rust: List.flatten — Flatten Nested Lists

## Side-by-Side Code

### OCaml

```ocaml
(* Idiomatic: built-in List.flatten *)
let flat = List.flatten [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]

(* Recursive: append head to flattened tail *)
let rec flatten_rec = function
  | [] -> []
  | head :: rest -> head @ flatten_rec rest

(* concat_map: map then flatten in one pass *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]
```

### Rust (idiomatic)

```rust
pub fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flatten().cloned().collect()
}

pub fn concat_map<T, U, F>(list: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> Vec<U>,
{
    list.iter().flat_map(f).collect()
}
```

### Rust (functional/recursive)

```rust
pub fn flatten_recursive<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    match nested {
        [] => vec![],
        [head, rest @ ..] => {
            let mut result = head.clone();
            result.extend(flatten_recursive(rest));
            result
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| flatten | `val flatten : 'a list list -> 'a list` | `fn flatten<T: Clone>(nested: &[Vec<T>]) -> Vec<T>` |
| concat_map | `val concat_map : ('a -> 'b list) -> 'a list -> 'b list` | `fn concat_map<T, U, F: Fn(&T) -> Vec<U>>(list: &[T], f: F) -> Vec<U>` |
| List type | `'a list list` | `&[Vec<T>]` (slice of Vecs) |
| Flat result | `'a list` | `Vec<T>` |

## Key Insights

1. **Iterator::flatten is direct:** Rust's `Iterator::flatten` is the exact structural equivalent of OCaml's `List.flatten` — both collapse one layer of nesting. No manual recursion or accumulator needed.
2. **flat_map vs concat_map:** OCaml's `List.concat_map f xs` and Rust's `.flat_map(f)` are the same abstraction: map to collections, flatten the results. Rust's iterator lazy evaluation means no intermediate allocation.
3. **Clone boundary:** OCaml lists are persistent/immutable and sharing is free. Rust's `&[Vec<T>]` holds references, so producing an owned `Vec<T>` requires `.cloned()`. If `T: Copy`, you'd use `.copied()` instead.
4. **Slice patterns mirror list patterns:** OCaml's `head :: rest` destructuring maps cleanly to Rust's `[head, rest @ ..]` slice pattern, making the recursive version very readable.
5. **Performance:** The idiomatic Rust version is O(n) in total elements and allocates exactly once. The recursive version allocates at each level (like OCaml's `@`), making it O(n²) in the worst case.

## When to Use Each Style

**Use idiomatic Rust when:** You have a flat collection to produce and want maximum efficiency — `.iter().flatten().cloned().collect()` is the canonical one-liner.
**Use recursive Rust when:** Teaching the OCaml parallel explicitly, or when processing nested structure that also needs transformation at each level before flattening.
