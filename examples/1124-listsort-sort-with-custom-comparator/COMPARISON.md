# OCaml vs Rust: List.sort — Sort with Custom Comparator

## Side-by-Side Code

### OCaml
```ocaml
let words = ["banana"; "apple"; "cherry"; "date"]

(* Idiomatic: sort lexicographically using String.compare *)
let sorted = List.sort String.compare words

(* Sort by string length, ties broken lexicographically *)
let by_length = List.sort (fun a b ->
  compare (String.length a) (String.length b)) words

(* Sort in descending order by reversing the comparator *)
let descending = List.sort (fun a b -> String.compare b a) words
```

### Rust (idiomatic)
```rust
pub fn sort_strings<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort();
    result
}
```

### Rust (functional — sort_by with custom comparator)
```rust
pub fn sort_by_length<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    result
}

pub fn sort_descending<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| b.cmp(a));
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Basic sort | `val sort : ('a -> 'a -> int) -> 'a list -> 'a list` | `fn sort_strings<'a>(words: &[&'a str]) -> Vec<&'a str>` |
| Comparator type | `'a -> 'a -> int` (negative/zero/positive) | `FnMut(&T, &T) -> Ordering` |
| Return type | New `'a list` (persistent) | `Vec<T>` (new owned vector) |
| Custom comparator | `List.sort (fun a b -> ...) xs` | `slice.sort_by(\|a, b\| ...)` |
| Stable sort | `List.stable_sort cmp xs` | `slice.sort_by(...)` (stable by default) |

## Key Insights

1. **Comparator return type:** OCaml comparators return `int` (negative/zero/positive); Rust comparators return `std::cmp::Ordering` (`Less`/`Equal`/`Greater`). The `Ordering` type is more explicit and eliminates off-by-one bugs from integer comparisons.
2. **Mutation vs persistence:** OCaml's `List.sort` returns a new list (lists are immutable linked structures). Rust's `sort`/`sort_by` mutates a `Vec` in place — you clone first if you need the original.
3. **Stability:** Rust's standard `sort`/`sort_by` is guaranteed stable (merge sort). OCaml's `List.sort` is also stable; `List.fast_sort` may not be.
4. **Chained comparisons:** Rust's `Ordering::then` / `then_with` chains multiple sort keys cleanly — equivalent to OCaml's nested `compare` calls but reads left-to-right without nesting.
5. **Borrowing:** The Rust functions take `&[&str]` (a borrowed slice of borrowed strings) and return `Vec<&str>` (owned vector of the same borrowed strings). No string data is copied — only the pointers.

## When to Use Each Style

**Use `sort()` when:** sorting types that implement `Ord` naturally — strings, integers, tuples. Zero boilerplate, same as OCaml's `List.sort String.compare`.

**Use `sort_by(\|a, b\| ...)` when:** you need a custom comparator — sorting by a field, reversing order, or chaining multiple criteria. Equivalent to OCaml's `List.sort (fun a b -> ...)`.

**Use `sort_by_key(\|x\| ...)` when:** sorting by a single extracted key — cleaner than `sort_by` for simple cases like `sort_by_key(\|s\| s.len())`.
