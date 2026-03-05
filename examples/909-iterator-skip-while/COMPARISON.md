# OCaml vs Rust: Conditional Skipping with skip_while()

## Side-by-Side Code

### OCaml
```ocaml
let rec skip_while pred = function
  | [] -> []
  | x :: xs as lst ->
    if pred x then skip_while pred xs else lst
```

### Rust (idiomatic — iterator adapter)
```rust
pub fn skip_less_than(slice: &[i32], threshold: i32) -> Vec<i32> {
    slice
        .iter()
        .copied()
        .skip_while(|&x| x < threshold)
        .collect()
}
```

### Rust (functional/recursive — mirrors OCaml)
```rust
pub fn skip_while_recursive<T, F>(slice: &[T], pred: F) -> &[T]
where
    F: Fn(&T) -> bool,
{
    match slice {
        [] => &[],
        [head, rest @ ..] if pred(head) => skip_while_recursive(rest, pred),
        _ => slice,
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val skip_while : ('a -> bool) -> 'a list -> 'a list` | `fn skip_while_recursive<T, F>(slice: &[T], pred: F) -> &[T]` |
| List/sequence type | `'a list` | `&[T]` (slice) |
| Predicate type | `'a -> bool` | `F: Fn(&T) -> bool` |
| Return type | `'a list` (new list) | `&[T]` (sub-slice, zero-copy) |

## Key Insights

1. **Zero-copy slice returns**: The recursive Rust version returns `&[T]` — a sub-slice pointing into the original allocation — whereas OCaml returns a shared reference to the existing list tail. Both avoid copying, but Rust makes the ownership explicit in the type.

2. **Iterator adapter vs recursion**: Idiomatic Rust delegates to `Iterator::skip_while`, which is lazy and composes freely with other adapters. OCaml's idiomatic form is direct recursion on a linked list; there is no lazy iterator in the stdlib equivalent.

3. **Slice patterns mirror list patterns**: `[head, rest @ ..]` in Rust is the direct equivalent of `x :: xs` in OCaml, making the recursive translation nearly mechanical. The `@ ..` rest-binding is Rust's spread pattern.

4. **"Once stopped, never restarts" semantic**: Both languages share this fundamental contract — the predicate is consulted only during the initial prefix scan. Elements after the first failure are yielded unconditionally, unlike `filter()` which tests every element.

5. **Generics vs polymorphism**: OCaml's `'a` type parameter is inferred and implicit; Rust requires an explicit `<T>` and a trait bound `F: Fn(&T) -> bool` for the predicate. The result is the same expressiveness with explicit contracts.

## When to Use Each Style

**Use idiomatic Rust (`skip_while` adapter) when:** composing with other iterator adapters (`.chain()`, `.map()`, `.collect()`), processing large streams lazily, or stripping structured prefixes like CSV comment lines or log headers.

**Use recursive Rust when:** demonstrating the OCaml translation clearly, working with recursive data structures where the iterator model doesn't map naturally, or when you need to return a sub-slice reference without allocating.
