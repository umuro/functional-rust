# OCaml vs Rust: Zip and Unzip

## Side-by-Side Code

### OCaml
```ocaml
(* Basic zip/unzip — built-in *)
let zip = List.combine        (* ('a list * 'b list) -> ('a * 'b) list *)
let unzip = List.split        (* ('a * 'b) list -> ('a list * 'b list) *)

(* zip_with / map2 *)
let dot_product xs ys =
  List.map2 ( * ) xs ys |> List.fold_left ( + ) 0

(* zip_with_index *)
let zip_with_index lst = List.mapi (fun i x -> (i, x)) lst

(* zip_longest — manual recursion *)
let rec zip_longest ~default_a ~default_b xs ys =
  match xs, ys with
  | [], [] -> []
  | x :: xs', y :: ys' -> (x, y) :: zip_longest ~default_a ~default_b xs' ys'
  | x :: xs', [] -> (x, default_b) :: zip_longest ~default_a ~default_b xs' []
  | [], y :: ys' -> (default_a, y) :: zip_longest ~default_a ~default_b [] ys'
```

### Rust (idiomatic — iterator adapters)
```rust
pub fn zip_vecs(a: &[i32], b: &[&str]) -> Vec<(i32, String)> {
    a.iter()
        .zip(b.iter())
        .map(|(&n, &s)| (n, s.to_string()))
        .collect()
}

pub fn unzip_vecs(pairs: &[(i32, &str)]) -> (Vec<i32>, Vec<String>) {
    pairs.iter().map(|&(n, s)| (n, s.to_string())).unzip()
}

pub fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
```

### Rust (functional/recursive — zip_longest)
```rust
pub fn zip_longest<T: Clone>(a: &[T], b: &[T], default_a: T, default_b: T) -> Vec<(T, T)> {
    let len = a.len().max(b.len());
    (0..len)
        .map(|i| {
            let x = a.get(i).cloned().unwrap_or_else(|| default_a.clone());
            let y = b.get(i).cloned().unwrap_or_else(|| default_b.clone());
            (x, y)
        })
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| zip | `val combine : 'a list -> 'b list -> ('a * 'b) list` | `.zip()` iterator adapter → `collect::<Vec<_>>()` |
| unzip | `val split : ('a * 'b) list -> 'a list * 'b list` | `.unzip()` on iterator of pairs |
| zip_with | `val map2 : ('a -> 'b -> 'c) -> 'a list -> 'b list -> 'c list` | `a.iter().zip(b).map(\|(x,y)| f(x,y)).collect()` |
| enumerate | `List.mapi (fun i x -> (i, x))` | `.iter().enumerate()` |
| zip_longest | custom recursion | index-based range loop + `.get().unwrap_or_else()` |

## Key Insights

1. **Truncation vs. error on mismatch:** OCaml's `List.combine` raises `Invalid_argument` if lists differ in length; Rust's `.zip()` silently stops at the shorter iterator — a quieter contract that may hide bugs.
2. **`.unzip()` is a collector:** Rust expresses unzip as a special `collect()` destination rather than a standalone function, making it composable with arbitrary iterator chains.
3. **`zip_with` → iterator chain:** OCaml's `List.map2 f xs ys` becomes `xs.iter().zip(ys).map(|(x,y)| f(x,y))` in Rust — more explicit but equally expressive.
4. **`zip_longest` needs manual work in both languages:** Neither OCaml stdlib nor Rust std provides a zip-longest; both require a custom implementation. Rust's index-based approach avoids explicit recursion.
5. **Ownership clarity:** `.zip()` in Rust works over references (`&T`), making it clear that neither input is consumed; `.unzip()` on an owned iterator transfers ownership into two output `Vec`s.

## When to Use Each Style

**Use idiomatic Rust (`.zip()` / `.unzip()`)** when combining or splitting parallel slices/iterators in a pipeline — it chains naturally with `.map()`, `.filter()`, and `.collect()`.

**Use `zip_longest`** when you cannot guarantee equal-length inputs and need to preserve all data rather than silently discard tails.
