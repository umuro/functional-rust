# Map and Fold on Trees: OCaml vs Rust

## The Core Insight
Once you can fold a tree, you can express almost any tree computation as a one-liner. This example shows how the catamorphism pattern — replacing constructors with functions — works identically in both languages, but Rust's ownership model adds friction around accumulator cloning and closure references.

## OCaml Approach
OCaml's `fold_tree` takes a function `f : 'a -> 'b -> 'b -> 'b` and a base value, recursing over the tree structure. Thanks to currying, `size = fold_tree (fun _ l r -> 1 + l + r) 0` reads cleanly. The GC handles all intermediate lists created by preorder/inorder — `[v] @ l @ r` allocates freely. Pattern matching with `function` keyword keeps the code terse.

## Rust Approach
Rust's `fold_tree` needs `A: Clone` because the accumulator must be passed to both subtrees — ownership can't be in two places at once. Closures are passed as `&impl Fn(...)` references to avoid ownership issues. The `vec!` macro and `extend` method replace OCaml's `@` list append. The code is slightly more verbose but makes every allocation explicit.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Fold signature | `('a -> 'b -> 'b -> 'b) -> 'b -> 'a tree -> 'b` | `(&Tree<T>, A, &impl Fn(&T, A, A) -> A) -> A` |
| Accumulator | Passed freely (GC) | Requires `Clone` bound |
| List append | `@` operator | `extend()` method |
| Closure passing | Implicit currying | `&impl Fn(...)` reference |
| Derived operations | One-liners via fold | One-liners via fold |
| Memory | GC handles intermediates | Explicit Vec allocation |

## What Rust Learners Should Notice
- The `Clone` bound on the accumulator is the price of ownership: both subtrees need their own copy of the base case
- `&impl Fn(...)` avoids taking ownership of the closure, so `fold_tree` can call it multiple times
- Rust's `vec![]` + `extend` is the idiomatic way to build up collections, replacing OCaml's `@` list concatenation
- The catamorphism pattern is universal — once you define fold for any data type, you unlock compositional programming
- Intermediate Vecs in preorder/inorder are allocated on the heap; in performance-critical code, you'd use a mutable accumulator instead

## Further Reading
- [The Rust Book — Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [OCaml Beyond Lists](https://cs3110.github.io/textbook/chapters/hop/beyond_lists.html)
