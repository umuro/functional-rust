📖 **[View on hightechmind.io →](https://hightechmind.io/rust/223-zygomorphism)**

---

# Zygomorphism — Two Mutually Dependent Folds

## Problem Statement

Sometimes two fold operations are interdependent: "compute the average" requires both the sum and the count simultaneously. Running two separate catamorphisms would traverse the structure twice. A zygomorphism runs two algebras in a single pass: a "helper" algebra computes auxiliary data, and the "main" algebra uses that auxiliary data alongside the recursive results. Named after the Greek for "yoke," it couples two folds together.

## Learning Outcomes

- Understand zygomorphisms as paired catamorphisms sharing one traversal
- Learn how the helper algebra provides auxiliary data to the main algebra
- See "average" as a canonical example: sum and count computed together
- Understand the performance benefit: one traversal instead of two

## Rust Application

`zygo<A, B>(helper: impl Fn(ExprF<A>) -> A, main: impl Fn(ExprF<(A, B)>) -> B) -> impl Fn(FixExpr) -> B`. The `helper` algebra computes auxiliary data (node count, depth, size). The `main` algebra receives `ExprF<(A, B)>` — pairs of helper result and recursive main result. Average: `helper` computes `(sum, count)` pairs; `main` divides sum by count at the top. The traversal runs once; both algebras execute at each node.

## OCaml Approach

OCaml's zygomorphism:
```ocaml
let rec zygo helper main (Fix ef) =
  let pairs = map_expr_f (fun child -> (helper_result child, zygo helper main child)) ef in
  main pairs
```
The recursion threads both results together. OCaml's `let rec` handles the mutual recursion. In practice, OCaml code often uses `para` or plain `let rec` instead of named recursion schemes, but the `zygo` pattern is useful for demonstrating the concept.

## Key Differences

1. **Two passes vs. one**: `zygo` combines two passes into one — equivalent to computing `cata alg1 . cata alg2` but more efficient (one traversal, not two).
2. **Dependency direction**: The helper algebra's results feed into the main algebra, not vice versa — one-directional coupling.
3. **Specialization**: `zygo` specializes to `cata` when the main algebra ignores the helper (no dependency) and to `para` when the helper is the original structure (identity algebra).
4. **Practical use**: Statistics computation (mean + variance in one pass), tree balancing checks, and "labeled fold" patterns use zygomorphisms.

## Exercises

1. Implement a `zygo` that computes both the sum and the product of a list in one traversal.
2. Write a tree zygomorphism that computes the average leaf value: helper computes `(sum, count)`, main divides.
3. Implement "is balanced?" using a zygomorphism: helper computes height, main checks balance condition per node.
