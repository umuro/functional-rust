# OCaml vs Rust: List.filter — Select Elements by Predicate

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds = List.filter (fun x -> x mod 2 <> 0) numbers

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
| filter | `('a -> bool) -> 'a list -> 'a list` | `fn filter<T: Clone, F>(items: &[T], pred: F) -> Vec<T>` |
| predicate type | `'a -> bool` | `F: Fn(&T) -> bool` |
| list type | `'a list` | `&[T]` (input slice), `Vec<T>` (output) |
| closure syntax | `fun x -> x mod 2 = 0` | `\|x: &i32\| x % 2 == 0` |

## Key Insights

1. **Clone requirement**: OCaml lists share structure (GC handles memory); Rust must `clone()` elements when moving them into a new `Vec` because ownership is explicit.
2. **Borrowed input, owned output**: Rust takes `&[T]` (borrowed slice) and returns `Vec<T>` (owned collection). OCaml's `'a list -> 'a list` uses GC to share nodes.
3. **Iterator chaining**: Rust's `.filter().cloned().collect()` pipeline is lazy until `.collect()` — no intermediate allocations. OCaml allocates a new list node on every match.
4. **Pattern matching**: OCaml's `| x :: rest ->` is idiomatic list decomposition; Rust mirrors it with `[head, rest @ ..]` slice patterns (available since Rust 1.42).
5. **Recursive tail position**: OCaml's `filter_rec` is not tail-recursive (it builds the result on the way back up); the same applies to the Rust recursive version, risking stack overflow on very long slices.

## When to Use Each Style

**Use iterator filter when:** working with slices or iterators in production code — it's idiomatic Rust and avoids manual recursion.
**Use recursive filter when:** learning the OCaml↔Rust translation or when processing custom recursive data structures (trees, linked lists) where pattern matching on the structure is natural.
