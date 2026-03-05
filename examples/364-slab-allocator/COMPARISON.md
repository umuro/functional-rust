# OCaml vs Rust: Slab Allocator

## Side-by-Side Comparison

### Type Definition

**OCaml:**
```ocaml
type 'a slab = {
  mutable data: 'a option array;
  mutable free: int list;
  mutable next_id: int;
}

let make cap = { data=Array.make cap None; free=[]; next_id=0 }
```

**Rust:**
```rust
struct Slab<T> {
    entries: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> Slab<T> {
    fn new() -> Self {
        Self { entries: Vec::new(), free: Vec::new() }
    }
}
```

### Insert Operation

**OCaml:**
```ocaml
let insert s v =
  match s.free with
  | i::rest -> s.data.(i) <- Some v; s.free <- rest; i
  | [] ->
    let i = s.next_id in
    s.data.(i) <- Some v; s.next_id <- i+1; i
```

**Rust:**
```rust
fn insert(&mut self, val: T) -> usize {
    if let Some(key) = self.free.pop() {
        self.entries[key] = Some(val);
        key
    } else {
        let key = self.entries.len();
        self.entries.push(Some(val));
        key
    }
}
```

### Remove Operation

**OCaml:**
```ocaml
let remove s i =
  s.data.(i) <- None;
  s.free <- i :: s.free
```

**Rust:**
```rust
fn remove(&mut self, key: usize) -> Option<T> {
    let slot = self.entries.get_mut(key)?;
    let val = slot.take()?;
    self.free.push(key);
    Some(val)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Backing store | Fixed-size array | Dynamic Vec |
| Free list | `int list` (immutable prepend) | `Vec<usize>` (push/pop) |
| Capacity | Fixed at creation | Grows dynamically |
| Bounds checking | Runtime exception | `Option` return |
| Return removed value | No | Yes (`Option<T>`) |

## Memory Model

**OCaml:** Uses a fixed-size array. Must pre-allocate capacity. Free slots tracked in a list.

**Rust:** Uses a dynamic `Vec`. Grows as needed. Free slots tracked in a `Vec` for O(1) push/pop.

## Slot Reuse

Both implementations maintain a free list for O(1) slot reuse:

```
Insert sequence: [A, B, C] → keys [0, 1, 2]
Remove key 1:    [A, _, C] → free list [1]
Insert D:        [A, D, C] → reuses key 1
```

## Use Cases

| Use Case | OCaml | Rust |
|----------|-------|------|
| Entity systems | Array + free list | `slab::Slab<T>` |
| Connection pools | Manual management | `slab` crate |
| Graph nodes | Integer IDs | Stable keys |
