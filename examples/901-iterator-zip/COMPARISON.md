# OCaml vs Rust: Pairing Elements with zip()

## Side-by-Side Code

### OCaml
```ocaml
let names = ["Alice"; "Bob"; "Carol"] in
let scores = [95; 87; 92] in
let paired = List.combine names scores in
(* paired: [("Alice", 95); ("Bob", 87); ("Carol", 92)] *)

(* List.combine raises Invalid_argument on length mismatch *)
let indexed = List.mapi (fun i x -> (i, x)) ["a"; "b"; "c"]
```

### Rust (idiomatic)
```rust
let names = ["Alice", "Bob", "Carol"];
let scores = [95u32, 87, 92];
let paired: Vec<_> = names.iter().zip(scores.iter()).collect();
// stops silently at shorter — no panic

let indexed: Vec<_> = ["a", "b", "c"].iter().enumerate().collect();
```

### Rust (functional — build HashMap via zip)
```rust
let map: HashMap<&str, u32> = names.iter()
    .zip(scores.iter())
    .map(|(&name, &score)| (name, score))
    .collect();
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Zip two lists | `List.combine : 'a list -> 'b list -> ('a * 'b) list` | `.zip() -> impl Iterator<Item=(A,B)>` |
| Enumerate | `List.mapi : (int -> 'a -> 'b) -> 'a list -> 'b list` | `.enumerate() -> impl Iterator<Item=(usize,T)>` |
| Unzip | `List.split : ('a * 'b) list -> 'a list * 'b list` | `.unzip() -> (Vec<A>, Vec<B>)` |
| Pair type | `'a * 'b` | `(A, B)` |

## Key Insights

1. **Error handling**: OCaml's `List.combine` raises `Invalid_argument` on length mismatch; Rust's `zip()` silently truncates at the shorter iterator — the safe, panic-free choice.
2. **Laziness**: Rust's `zip()` is lazy — it produces pairs on demand without allocating. OCaml's `List.combine` eagerly builds a new list. Add `.collect()` in Rust when you need a concrete `Vec`.
3. **Enumerate as zip**: OCaml uses `List.mapi` to pair indices with elements; Rust uses `.enumerate()`, which is `zip(0..)` in disguise — both express the same intent, but Rust's name is more discoverable.
4. **Unzip symmetry**: Both languages provide the inverse operation (`List.split` / `.unzip()`), making round-trips straightforward. Rust's `.unzip()` is a collector, so the types are inferred from context.
5. **Composability**: Because `zip()` returns an iterator, you can chain further adapters (`.map()`, `.filter()`, `.flat_map()`) before collecting — OCaml achieves the same with `List.map` applied to the combined list, but Rust avoids the intermediate allocation.

## When to Use Each Style

**Use idiomatic Rust (`.zip().collect()`)** when you need a `Vec` of pairs for later use or when feeding into `.collect::<HashMap<_,_>>()`.
**Use `.zip()` inline in a `for` loop or `.for_each()`** when you only need to process pairs once and don't want to allocate — the most common pattern and zero overhead.
