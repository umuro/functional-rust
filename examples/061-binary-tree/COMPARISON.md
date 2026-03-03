# Binary Tree — Size, Membership, Traversal: OCaml vs Rust

## The Core Insight
Binary trees are the canonical recursive data structure. Both languages use algebraic types (OCaml's `type` / Rust's `enum`), but Rust's ownership model forces you to think about where tree nodes live in memory — a distinction OCaml's garbage collector hides entirely.

## OCaml Approach
OCaml's `type 'a tree = Leaf | Node of 'a * 'a tree * 'a tree` is beautifully concise. The GC handles all allocation and deallocation, so recursive types "just work" without any indirection markers. Pattern matching with `function` keyword makes structural recursion read almost like a mathematical definition. Polymorphism comes free via `'a` type parameters, and structural equality (`=`) works on any type without extra annotations.

## Rust Approach
Rust's `enum Tree<T> { Leaf, Node(T, Box<Tree<T>>, Box<Tree<T>>) }` requires `Box` for heap allocation of recursive children — without it, the compiler can't compute the enum's size. This is the key tradeoff: explicit memory layout in exchange for zero-cost abstractions and no GC pauses. Generic functions need trait bounds (`PartialEq` for comparison, `Clone` for copying values out of borrowed trees). The `&Tree<T>` borrow pattern lets traversals share data without cloning.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Memory | GC-managed, implicit indirection | `Box<T>` for explicit heap allocation |
| Recursive types | Direct, no annotation needed | Requires `Box` to break infinite size |
| Equality | Structural `=` on any type | Requires `PartialEq` trait bound |
| Polymorphism | `'a` type parameter | `<T>` generic with trait bounds |
| Traversal | Returns new list, GC cleans up | Borrows with `&T` or clones with `Clone` |
| Pattern matching | `function` keyword sugar | `match` expression |

## What Rust Learners Should Notice
- `Box<T>` is Rust's way of saying "this value lives on the heap" — it's a single-owner smart pointer, not a shared reference
- You must choose: return `Vec<&T>` (borrowing) or `Vec<T>` (cloning/moving). OCaml doesn't force this choice because GC manages lifetimes
- Helper constructors like `Tree::node()` reduce `Box::new()` noise — a common Rust pattern for recursive types
- The `#[derive(Debug, Clone, PartialEq)]` line is Rust's way of opting into capabilities that OCaml provides by default
- Iterative traversal with an explicit stack avoids deep recursion stack overflow — important in Rust where stack size is bounded

## Further Reading
- [The Rust Book — Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The Rust Book — Box<T>](https://doc.rust-lang.org/book/ch15-01-box.html)
- [OCaml Trees](https://cs3110.github.io/textbook/chapters/data/trees.html)
