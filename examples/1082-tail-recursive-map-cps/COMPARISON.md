# OCaml vs Rust: Tail-Recursive Map with CPS

## Side-by-Side Code

### OCaml
```ocaml
(* Naive — not tail-recursive *)
let rec map_naive f = function
  | [] -> []
  | h :: t -> f h :: map_naive f t

(* Tail-recursive with accumulator + reverse *)
let map_tr f lst =
  let rec go acc = function
    | [] -> List.rev acc
    | h :: t -> go (f h :: acc) t
  in go [] lst

(* CPS — tail-recursive, preserves order *)
let map_cps f lst =
  let rec go k = function
    | [] -> k []
    | h :: t -> go (fun rest -> k (f h :: rest)) t
  in go Fun.id lst
```

### Rust (idiomatic)
```rust
pub fn map_idiomatic<T, U, F: Fn(&T) -> U>(f: F, list: &[T]) -> Vec<U> {
    list.iter().map(f).collect()
}
```

### Rust (accumulator loop — equivalent of OCaml's tail-recursive version)
```rust
pub fn map_acc<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    let mut acc = Vec::with_capacity(list.len());
    for item in list {
        acc.push(f(item));
    }
    acc
}
```

### Rust (CPS with boxed closures)
```rust
pub fn map_cps<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    let mut cont: Box<dyn FnOnce(Vec<U>) -> Vec<U>> = Box::new(|v| v);
    for item in list.iter().rev() {
        let mapped = f(item);
        let prev = cont;
        cont = Box::new(move |mut rest: Vec<U>| {
            rest.push(mapped);
            prev(rest)
        });
    }
    cont(Vec::new())
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Naive map | `val map_naive : ('a -> 'b) -> 'a list -> 'b list` | `fn map_naive<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U>` |
| Accumulator map | `val map_tr : ('a -> 'b) -> 'a list -> 'b list` | `fn map_acc<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U>` |
| CPS map | `val map_cps : ('a -> 'b) -> 'a list -> 'b list` | `fn map_cps<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U>` |
| Continuation type | `'b list -> 'b list` (implicit) | `Box<dyn FnOnce(Vec<U>) -> Vec<U>>` (explicit heap allocation) |
| List type | `'a list` (singly-linked, immutable) | `&[T]` input / `Vec<U>` output (contiguous, mutable) |

## Key Insights

1. **TCO is the fundamental difference.** OCaml guarantees tail-call optimization, making structural recursion with an accumulator genuinely stack-safe. Rust does not — the same recursive structure still grows the call stack. Rust's equivalent is an explicit loop.

2. **Cons vs push inverts accumulation order.** OCaml's `::` prepends to a linked list in O(1), naturally building in reverse. That's why `List.rev` is needed at the end. Rust's `Vec::push` appends in O(1) amortized, building in forward order — the reverse step disappears entirely.

3. **Continuations require explicit heap allocation in Rust.** OCaml closures are lightweight GC-managed values — building a chain of continuations is cheap. In Rust, each continuation must be `Box<dyn FnOnce(...)>`, requiring a heap allocation per list element. This makes CPS a poor fit for performance-sensitive Rust code.

4. **CPS in Rust is educational, not practical.** Applying the continuation chain still involves O(n) nested function calls (each continuation calls the previous one), so it can still overflow. The technique demonstrates the concept but doesn't solve the stack-safety problem the way it does in OCaml.

5. **Iterator chains are Rust's zero-cost abstraction for this pattern.** `iter().map(f).collect()` is the idiomatic Rust solution — it compiles to a tight loop with no recursion, no heap-allocated closures, and no intermediate allocations beyond the output `Vec`.

## When to Use Each Style

**Use the accumulator loop (`map_acc`) when:** you need an explicit loop body with complex logic that doesn't fit a single closure — e.g., conditional mapping, stateful transforms, or early termination.

**Use CPS (`map_cps`) when:** you're teaching or studying continuation-passing style. It's valuable for understanding how OCaml achieves tail recursion through continuations, but it's not the right tool for production Rust.

**Use idiomatic Rust (`map_idiomatic`) when:** this is the default choice. Iterator chains are the Rust way — zero-cost, composable, and optimized by the compiler.
