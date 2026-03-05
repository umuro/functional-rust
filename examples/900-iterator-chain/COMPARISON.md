# OCaml vs Rust: Chaining Iterators with chain()

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: @ operator (List.append) — eager, allocates a new list *)
let first = [1; 2; 3]
let second = [4; 5; 6]
let chained = first @ second          (* new list allocated here *)

(* Lazy alternative: Seq.append *)
let seq1 = List.to_seq [1; 2; 3]
let seq2 = List.to_seq [4; 5; 6]
let lazy_chain = Seq.append seq1 seq2 (* no allocation until iterated *)
```

### Rust (idiomatic)
```rust
// Rust: .chain() — lazy, zero extra allocation
let first = [1, 2, 3];
let second = [4, 5, 6];
let chained: Vec<i32> = first.iter().chain(second.iter()).copied().collect();
```

### Rust (functional/generic)
```rust
// Works over any two iterators of matching type
fn chain_iters<I, J, T>(a: I, b: J) -> Vec<T>
where
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    a.chain(b).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Eager concat | `val (@) : 'a list -> 'a list -> 'a list` | `[a, b].concat()` / `Vec::extend` |
| Lazy chain | `val Seq.append : 'a Seq.t -> 'a Seq.t -> 'a Seq.t` | `fn chain<U>(self, other: U) -> Chain<Self, U>` |
| Element copy | implicit (boxed OCaml values) | `.copied()` (explicit, `T: Copy`) |

## Key Insights

1. **Allocation model:** OCaml's `@` operator always allocates a new list immediately. Rust's `.chain()` defers everything — nothing is materialized until you call `.collect()` or iterate.

2. **Lazy by default:** Rust iterators are state machines describing *what to do*, not results. `.chain()` returns a `Chain<A, B>` struct that holds both iterators; it advances `A` until exhausted, then advances `B`.

3. **OCaml Seq is the real parallel:** `Seq.append` in OCaml is genuinely lazy, matching Rust's `.chain()` in spirit. The common `@` operator is the *eager* path — the idiomatic OCaml shortcut that doesn't scale to large data.

4. **Zero-cost abstraction:** The `Chain` iterator compiles to the same machine code as a hand-written loop that processes `first` then `second`. There is no virtual dispatch, no heap allocation for the combinator itself.

5. **Composability:** `.chain()` can be stacked: `a.chain(b).chain(c)` builds a `Chain<Chain<A, B>, C>` — all resolved at compile time, all zero cost. OCaml's `Seq.append` composes similarly but through closures rather than monomorphized types.

## When to Use Each Style

**Use `.chain()` on iterators when:** you want to process two sequences in order without allocating a combined collection — e.g., streaming log entries from multiple sources, combining filtered results, or building pipelines over large datasets.

**Use `.collect()` after `.chain()` when:** you need a concrete `Vec` for random access, repeated iteration, or passing to an API that requires owned data. The allocation happens exactly once, at the point you call `.collect()`.
