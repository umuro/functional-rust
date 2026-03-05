# OCaml vs Rust: Persistent Data Structures

## Side-by-Side Comparison

### Persistent List

**OCaml:**
```ocaml
let lst1 = [1;2;3;4;5]
let lst2 = 0 :: lst1  (* shares tail with lst1 *)
let lst3 = List.tl lst1  (* shares almost all of lst1 *)
```

**Rust:**
```rust
enum PList<T> {
    Nil,
    Cons(T, Rc<PList<T>>),
}

let l1 = PList::cons(1, PList::nil());
let l2 = PList::cons(0, Rc::clone(&l1)); // shares l1
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Default behavior | Persistent (immutable) | Owned (mutable) |
| Sharing mechanism | Automatic (GC) | Explicit `Rc<T>` |
| List cons | `::` operator | `Cons(h, Rc::clone(&t))` |
| Memory management | GC | Reference counting |

## Memory Model

**OCaml:** The GC tracks all references. When you write `h :: t`, you create a new cons cell pointing to the same tail. The GC ensures neither version is freed while references exist.

**Rust:** `Rc<T>` provides reference counting. When all `Rc` clones go out of scope, the data is freed. No GC pauses, but cycles would leak (use `Weak` if needed).

## Performance

| Operation | Persistent List | Persistent Vec |
|-----------|----------------|----------------|
| Prepend | O(1) | O(n) |
| Append | O(n) | O(n) |
| Update | O(n) | O(n) |
| Access | O(n) | O(1) |

Note: For better persistent vector performance, use a tree-based structure like HAMT (O(log₃₂ n) updates).
