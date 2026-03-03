# Mutual Recursion with `and`: OCaml vs Rust

## The Core Insight
Mutual recursion — functions that call each other — highlights a fundamental difference in how the two languages handle name resolution. OCaml processes definitions sequentially and needs explicit `and` to co-define functions; Rust resolves all items in a scope simultaneously, making mutual recursion invisible at the syntax level.

## OCaml Approach
OCaml's `let rec is_even = ... and is_odd = ...` explicitly tells the compiler that these functions are defined together and may reference each other. Without `and`, `is_odd` wouldn't be in scope when `is_even` is compiled. The same `and` keyword works for mutually recursive types and modules. This sequential processing is a deliberate design choice that makes code easier to reason about locally.

## Rust Approach
In Rust, all items (functions, types, traits) within a module are visible to each other regardless of definition order. `is_even` can call `is_odd` and vice versa with no special syntax. This is because Rust separates name resolution from compilation — it first collects all names, then type-checks. For the expression evaluator, `eval_expr` and `eval_mul` call each other naturally.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Mutual recursion | `let rec ... and ...` | No special syntax needed |
| Name resolution | Sequential (top-down) | All items visible simultaneously |
| Recursive types | `type ... and ...` | Enums reference each other freely |
| Forward reference | Not allowed without `and` | Always allowed within module |
| Stack safety | No guaranteed TCO | No guaranteed TCO (use iteration) |

## What Rust Learners Should Notice
- Rust's "all items visible" approach means you never need forward declarations for functions — a big ergonomic win
- However, `let` bindings within a function body ARE sequential (like OCaml's `let`) — only module-level items are mutually visible
- Neither language guarantees tail-call optimization, so deep mutual recursion (e.g., `is_even(1_000_000)`) can overflow the stack
- The expression evaluator pattern (enum + match + mutual functions) is extremely common in interpreters and compilers — both languages excel at this
- Rust's `Box::new` for recursive enum variants is the main additional cost compared to OCaml's GC-managed types

## Further Reading
- [The Rust Book — Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [OCaml Mutual Recursion](https://cs3110.github.io/textbook/chapters/modules/modules.html)
