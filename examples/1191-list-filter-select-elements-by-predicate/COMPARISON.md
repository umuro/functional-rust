# OCaml vs Rust: List.filter — Select Elements by Predicate

## Side-by-Side Code

### OCaml

```ocaml
(* Idiomatic: built-in List.filter *)
let evens = List.filter (fun x -> x mod 2 = 0) [1; 2; 3; 4; 5; 6; 7; 8]
let odds  = List.filter (fun x -> x mod 2 <> 0) [1; 2; 3; 4; 5; 6; 7; 8]

(* Recursive: pattern match on list spine *)
let rec filter_rec pred = function
  | [] -> []
  | x :: rest ->
    if pred x then x :: filter_rec pred rest
    else filter_rec pred rest
```

### Rust (idiomatic)

```rust
pub fn filter<T: Clone, F>(items: &[T], pred: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| pred(x)).cloned().collect()
}

pub fn filter_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 == 0).copied().collect()
}
```

### Rust (functional/recursive)

```rust
pub fn filter_recursive<T: Clone, F>(items: &[T], pred: &F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => vec![],
        [head, rest @ ..] => {
            let mut tail = filter_recursive(rest, pred);
            if pred(head) {
                tail.insert(0, head.clone());
            }
            tail
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| filter | `val filter : ('a -> bool) -> 'a list -> 'a list` | `fn filter<T: Clone, F: Fn(&T) -> bool>(items: &[T], pred: F) -> Vec<T>` |
| predicate type | `'a -> bool` | `Fn(&T) -> bool` |
| input | `'a list` | `&[T]` (borrowed slice) |
| output | `'a list` | `Vec<T>` (owned) |

## Key Insights

1. **Direct structural equivalent:** `List.filter` in OCaml and `Iterator::filter` in Rust implement the same operation — keep elements satisfying a predicate in a single pass. The mental model is identical; only the syntax differs.
2. **Clone requirement arises from ownership:** OCaml's GC-managed lists share structure freely; the original list is never touched. Rust's `&[T]` slice is borrowed, so producing an owned `Vec<T>` requires cloning each element with `.cloned()` or `.copied()` for `Copy` types.
3. **Predicate receives a reference:** Because `items.iter()` yields `&T`, the filter closure receives `&&T`. The double-deref is handled by writing `|&&x|` (for `Copy` types) or `|x| pred(x)` in a wrapper. OCaml predicates receive the value directly.
4. **Lazy evaluation in Rust:** Rust's iterator chain is lazy — `filter` does not execute until a consuming adapter like `.collect()` is called. This means you can chain `.filter().map().take()` without intermediate allocations; OCaml's `List.filter` allocates immediately.
5. **Recursive pattern parity:** OCaml's `x :: rest` list destructuring maps cleanly to Rust's `[head, rest @ ..]` slice pattern. The logic of the recursive version is identical in both languages.

## When to Use Each Style

**Use idiomatic Rust when:** Processing a slice or iterator and collecting the results — `.iter().filter(pred).cloned().collect()` is the idiomatic one-liner and compiles to a tight loop.
**Use recursive Rust when:** Demonstrating the OCaml parallel explicitly, or when you need to transform the structure during filtering (e.g., wrapping in a tree node) rather than just selecting elements.
