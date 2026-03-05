# OCaml vs Rust: Rc\<T\> — Shared Ownership

## Side-by-Side Code

### OCaml
```ocaml
(* Sharing is implicit — no annotation needed *)
type tree = Leaf | Node of tree * int * tree

let shared = Node (Leaf, 42, Leaf)
let tree1  = Node (shared, 1, Leaf)   (* GC keeps shared alive *)
let tree2  = Node (Leaf, 2, shared)   (* GC keeps shared alive *)

(* Shared-tail list — tail is never copied *)
let tail   = [3; 2; 1]
let list_a = 10 :: tail
let list_b = 20 :: tail
```

### Rust (idiomatic — Rc for shared ownership)
```rust
use std::rc::Rc;

let shared = Rc::new(Tree::Node(Rc::new(Tree::Leaf), 42, Rc::new(Tree::Leaf)));
// Rc::clone bumps the reference count — O(1), no heap allocation
let tree1 = Tree::Node(Rc::clone(&shared), 1, Rc::new(Tree::Leaf));
let tree2 = Tree::Node(Rc::new(Tree::Leaf), 2, Rc::clone(&shared));
// shared strong_count == 3 here; freed when all three drop
```

### Rust (functional — shared-tail cons list)
```rust
let tail   = List::cons(3, List::cons(2, List::cons(1, List::nil())));
let list_a = List::cons(10, Rc::clone(&tail));  // shares tail
let list_b = List::cons(20, Rc::clone(&tail));  // shares tail
// tail strong_count == 3; inner nodes never copied
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared pointer | `'a` (implicit GC) | `Rc<T>` |
| Clone a handle | `let b = a` (implicit) | `Rc::clone(&a)` |
| Reference count | hidden | `Rc::strong_count(&a) : usize` |
| Drop | GC-determined | deterministic on last drop |
| Thread safety | GC-managed | `Rc` is `!Send`; use `Arc` for threads |

## Key Insights

1. **Opt-in sharing:** Rust's `Rc<T>` makes shared ownership visible in the type. Every `Rc::clone` call is a deliberate decision — no hidden aliases.
2. **Zero-cost clone:** `Rc::clone` only increments an integer counter; the data on the heap is not copied. This matches OCaml's pointer-copy semantics.
3. **Deterministic drop:** Unlike OCaml's GC, `Rc` frees memory the moment the last handle is dropped — useful for resources like file handles or locks inside an `Rc`.
4. **No cycles:** `Rc` cannot break reference cycles on its own; use `Weak<T>` for back-pointers to avoid leaking memory — a trade-off OCaml's GC avoids.
5. **Single-threaded only:** `Rc<T>` is not `Send`; the compiler prevents accidental sharing across threads. `Arc<T>` uses atomic operations for the same pattern in multi-threaded code.

## When to Use Each Style

**Use `Rc<T>` when:** you genuinely need multiple owners in single-threaded code — shared tree nodes, immutable cons lists, reference-counted caches, or parent/child GUI widgets.

**Use plain references (`&T`) when:** you only need temporary access and the lifetime is clear — prefer borrows over `Rc` for zero-overhead sharing.
