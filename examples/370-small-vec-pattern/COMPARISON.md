# OCaml vs Rust: SmallVec Pattern

## Side-by-Side Comparison

### OCaml Approach

**OCaml:**
```ocaml
(* OCaml arrays of floats/ints are already unboxed *)
let small_array = [|1;2;3;4|]  (* stack-like, unboxed *)

(* For dynamic small collections, use list or array *)
let push_small arr x = Array.append arr [|x|]
```

### Rust Approach

**Rust:**
```rust
enum SmallVec<T, const N: usize> {
    Inline { data: [Option<T>; N], len: usize },
    Heap(Vec<T>),
}

impl<T, const N: usize> SmallVec<T, N> {
    fn push(&mut self, val: T) {
        match self {
            Inline { data, len } if *len < N => {
                data[*len] = Some(val);
                *len += 1;
            }
            Inline { data, len } => {
                // Spill to heap
                let mut v = /* collect inline items */;
                v.push(val);
                *self = Heap(v);
            }
            Heap(v) => v.push(val),
        }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Small optimization | Lists are cons cells | Explicit SmallVec |
| Array allocation | Always heap | Inline for N items |
| Const generics | N/A | `const N: usize` |
| Memory layout | GC-managed | Explicit enum |

## Memory Layout

**OCaml:** Arrays are heap-allocated but contain unboxed primitives. Lists are cons cells (always heap).

**Rust SmallVec:**
- Inline mode: `[Option<T>; N]` on stack + len
- Heap mode: Vec pointer + capacity + len

## Performance Characteristics

| Operation | SmallVec (inline) | SmallVec (heap) | Vec |
|-----------|------------------|-----------------|-----|
| Push | O(1) | O(1) amortized | O(1) amortized |
| Access | O(1) | O(1) | O(1) |
| Memory | Stack | Heap | Heap |
| Allocation | None | Once on spill | On first push |

## When to Use

- Collections usually have ≤N elements
- Many short-lived small collections
- Performance-critical inner loops
- Avoiding allocator pressure
