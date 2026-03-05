# OCaml vs Rust: Arena Allocation

## Side-by-Side Comparison

### Arena Type Definition

**OCaml:**
```ocaml
type 'a arena = {
  mutable items: 'a list;
  mutable count: int;
}

let make_arena () = { items=[]; count=0 }
```

**Rust:**
```rust
struct Arena {
    data: Vec<u8>,
    offset: Cell<usize>,
    allocations: Cell<usize>,
}

impl Arena {
    fn new(capacity: usize) -> Self {
        Self {
            data: vec![0u8; capacity],
            offset: Cell::new(0),
            allocations: Cell::new(0),
        }
    }
}
```

### Allocation

**OCaml:**
```ocaml
let arena_alloc a v =
  a.items <- v :: a.items;
  a.count <- a.count + 1;
  v
```

**Rust:**
```rust
fn alloc_bytes(&self, size: usize, align: usize) -> Option<usize> {
    let offset = self.offset.get();
    let aligned = (offset + align - 1) & !(align - 1);
    let new_offset = aligned + size;
    if new_offset > self.data.len() { return None; }
    self.offset.set(new_offset);
    self.allocations.set(self.allocations.get() + 1);
    Some(aligned)
}
```

### Reset / Free All

**OCaml:**
```ocaml
let arena_reset a =
  Printf.printf "Freeing %d items\n" a.count;
  a.items <- [];
  a.count <- 0
```

**Rust:**
```rust
fn reset(&self) {
    self.offset.set(0);
    self.allocations.set(0);
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Memory model | GC-managed list | Raw byte buffer |
| Allocation | Prepend to list | Bump pointer |
| Free all | Clear list, GC collects | Reset offset (O(1)) |
| Alignment | N/A (GC handles) | Manual alignment |
| Typed safety | Type parameter `'a` | Separate `TypedArena<T>` |
| Interior mutability | `mutable` fields | `Cell<usize>` |

## Memory Layout

**OCaml:** Items stored as linked list. Each cons cell has GC overhead. The arena doesn't actually control memory layout - GC does.

**Rust:** Contiguous byte buffer. Bump pointer advances linearly. True arena semantics with O(1) reset.

## Use Cases

| Use Case | OCaml Approach | Rust Approach |
|----------|---------------|---------------|
| Compiler AST | Minor heap (GC optimizes) | `typed_arena::Arena` |
| Game frame data | N/A | `bumpalo::Bump` |
| Per-request allocation | Minor heap | Arena with reset |
| Persistent structures | Natural (immutable) | Requires `Rc`/`Arc` |

## Performance

- **OCaml**: GC pause latency, but excellent for short-lived objects
- **Rust**: Deterministic O(1) allocation and O(1) reset, no GC pauses
