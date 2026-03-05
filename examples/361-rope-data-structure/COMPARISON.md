# OCaml vs Rust: Rope Data Structure

## Side-by-Side Comparison

### Type Definition

**OCaml:**
```ocaml
type rope =
  | Leaf of string
  | Node of rope * rope * int  (* left, right, total_len *)
```

**Rust:**
```rust
enum Rope {
    Leaf(String),
    Node {
        left: Box<Rope>,
        right: Box<Rope>,
        length: usize,
    },
}
```

### Length Calculation

**OCaml:**
```ocaml
let length = function
  | Leaf s -> String.length s
  | Node (_,_,n) -> n
```

**Rust:**
```rust
fn length(&self) -> usize {
    match self {
        Rope::Leaf(s) => s.len(),
        Rope::Node { length, .. } => *length,
    }
}
```

### Concatenation

**OCaml:**
```ocaml
let concat a b = match a, b with
  | Leaf "", _ -> b
  | _, Leaf "" -> a
  | _ -> make_node a b
```

**Rust:**
```rust
fn concat(left: Rope, right: Rope) -> Rope {
    if left.length() == 0 { return right; }
    if right.length() == 0 { return left; }
    let length = left.length() + right.length();
    Rope::Node {
        left: Box::new(left),
        right: Box::new(right),
        length,
    }
}
```

### Index Access

**OCaml:**
```ocaml
let rec index_at rope i =
  match rope with
  | Leaf s -> s.[i]
  | Node (l,_,_) when i < length l -> index_at l i
  | Node (l,r,_) -> index_at r (i - length l)
```

**Rust:**
```rust
fn byte_at(&self, idx: usize) -> Option<u8> {
    match self {
        Rope::Leaf(s) => s.as_bytes().get(idx).copied(),
        Rope::Node { left, right, .. } => {
            let ll = left.length();
            if idx < ll { left.byte_at(idx) }
            else { right.byte_at(idx - ll) }
        }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Heap allocation | Automatic GC | Explicit `Box<T>` |
| Pattern matching | Built-in ADT | `match` on enum |
| String concat | `^` operator | `+` with `&str` |
| Memory safety | GC-managed | Ownership + borrowing |
| Index bounds | Runtime exception | `Option<T>` return |
| String type | Immutable `string` | Owned `String` |

## Memory Model

**OCaml:** Values are GC-managed. When you create a `Node(left, right, len)`, the GC tracks all references. Structural sharing happens naturally.

**Rust:** Each `Box<Rope>` is a heap allocation with unique ownership. Structural sharing requires `Rc<Rope>` or `Arc<Rope>` for reference counting.

## Performance Considerations

- OCaml's GC adds latency spikes during collection
- Rust's `Box` has deterministic deallocation
- Both achieve O(1) concatenation
- Both achieve O(log n) index access with balanced trees
