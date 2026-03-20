# OCaml vs Rust: List.partition — Divide List by Predicate

## Side-by-Side Code

### OCaml
```ocaml
(* Idiomatic — one function call *)
let (small, big) = List.partition (fun x -> x <= 5) [1;2;3;4;5;6;7;8;9;10]

(* Recursive — explicit structural recursion *)
let rec partition_rec pred = function
  | [] -> ([], [])
  | x :: rest ->
    let (yes, no) = partition_rec pred rest in
    if pred x then (x :: yes, no)
    else (yes, x :: no)
```

### Rust (idiomatic)
```rust
pub fn partition_idiomatic<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().partition(|x| pred(x))
}
```

### Rust (functional fold)
```rust
pub fn partition_fold<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().fold((vec![], vec![]), |(mut yes, mut no), x| {
        if pred(x) { yes.push(x); } else { no.push(x); }
        (yes, no)
    })
}
```

### Rust (recursive)
```rust
pub fn partition_recursive<'a, T, F>(items: &'a [T], pred: &F) -> (Vec<&'a T>, Vec<&'a T>)
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => (vec![], vec![]),
        [head, rest @ ..] => {
            let (mut yes, mut no) = partition_recursive(rest, pred);
            if pred(head) { yes.insert(0, head); } else { no.insert(0, head); }
            (yes, no)
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val partition : ('a -> bool) -> 'a list -> 'a list * 'a list` | `fn partition_idiomatic<T, F>(items: &[T], pred: F) -> (Vec<&T>, Vec<&T>)` |
| List type | `'a list` | `&[T]` (borrowed slice) |
| Predicate | `'a -> bool` | `Fn(&T) -> bool` |
| Return type | `'a list * 'a list` | `(Vec<&T>, Vec<&T>)` |
| Tuple result | `let (small, big) = ...` | `let (small, big) = ...` (identical syntax) |

## Key Insights

1. **Direct API parity:** `Iterator::partition` and `List.partition` are conceptually identical — both take a predicate and return a pair of collections. The Rust API feels like a direct port.
2. **Borrowing avoids copying:** The Rust implementation returns `Vec<&T>` — references into the original slice. OCaml always produces new lists (cons cells) because lists are immutable and linked. Rust's borrow checker makes zero-copy partitioning safe and natural.
3. **Predicate takes `&T` not `T`:** In Rust, iterating over `&[T]` yields `&T`. The predicate must accept a reference. This is why the closure is `|x| *x <= 5` (or `|x: &i32| *x <= 5`) rather than `|x| x <= 5`.
4. **Lifetime threading in recursive form:** The recursive Rust version requires an explicit lifetime `'a` to connect the input slice lifetime to the output references. OCaml's GC handles this automatically — there is no concept of lifetimes.
5. **Fold mirrors accumulator recursion:** The fold version closely mirrors how you would manually implement `List.partition` in OCaml without pattern matching: carry two accumulators and decide which to extend at each step.

## When to Use Each Style

**Use idiomatic Rust (`Iterator::partition`) when:** you have a slice or any iterator and want the simplest, most readable, most performant partition — this is the right default.

**Use fold when:** you want to be explicit about the accumulator pattern, or you are building a more complex partition (e.g., partitioning into more than two groups by accumulating differently).

**Use recursive Rust when:** you are teaching the OCaml parallel or need to process the slice head-first with early-termination logic not expressible with `partition`.
