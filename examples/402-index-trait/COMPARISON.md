# OCaml vs Rust: Index Trait

## Side-by-Side Code

### OCaml — Function-based access
```ocaml
type 'a matrix = {
  rows: int;
  cols: int;
  data: 'a array;
}

let get m r c =
  if r >= m.rows || c >= m.cols then failwith "Out of bounds"
  else m.data.(r * m.cols + c)

let set m r c v =
  if r >= m.rows || c >= m.cols then failwith "Out of bounds"
  else m.data.(r * m.cols + c) <- v

let () =
  let m = { rows = 3; cols = 3; data = Array.make 9 0 } in
  set m 1 2 42;
  Printf.printf "m[1][2] = %d\n" (get m 1 2)
```

### Rust — Operator overloading via Index trait
```rust
use std::ops::{Index, IndexMut};

struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (r, c): (usize, usize)) -> &f64 {
        &self.data[r * self.cols + c]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut f64 {
        &mut self.data[r * self.cols + c]
    }
}

fn main() {
    let mut m = Matrix { rows: 3, cols: 3, data: vec![0.0; 9] };
    m[(1, 2)] = 42.0;  // IndexMut
    println!("m[1][2] = {}", m[(1, 2)]);  // Index
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Syntax | `get m r c` / `set m r c v` | `m[(r, c)]` / `m[(r, c)] = v` |
| Index type | Fixed by function signature | Any type via `Index<Idx>` trait |
| Return type | Value (copy) | Reference (`&T` / `&mut T`) |
| Mutability | Separate `set` function | `IndexMut` trait |
| Customization | Write new functions | Implement `Index` for your type |
| Bounds checking | Manual in function | Manual in `index()` impl |

---

## Index Types in Rust

Rust's `Index` trait is generic over the index type:

```rust
// Index by tuple
impl Index<(usize, usize)> for Matrix { ... }

// Index by string
impl Index<&str> for Config { ... }

// Index by custom type
struct NodeId(usize);
impl Index<NodeId> for Graph { ... }

// Index by range (slicing)
impl Index<Range<usize>> for String { ... }
```

OCaml uses different functions for each access pattern.

---

## The Reference Return

Rust's `Index` returns a **reference**, not a value:

```rust
impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, idx: (usize, usize)) -> &f64  // returns &f64, not f64
}
```

This enables:
1. Zero-copy access to large values
2. `IndexMut` for in-place modification
3. Chained indexing: `matrix[(0, 0)].method()`

OCaml's `get` returns a value, requiring `set` for modification.

---

## 5 Takeaways

1. **`[]` operator works with any index type in Rust.**
   Tuples, strings, custom IDs — whatever makes sense for your domain.

2. **OCaml uses functions; Rust uses traits.**
   Both achieve the goal; Rust's approach integrates with existing syntax.

3. **`Index` returns references, enabling zero-copy access.**
   Large structs inside containers aren't copied on each access.

4. **`IndexMut` enables write-through: `m[(i,j)] = v`.**
   The mutable reference returned allows direct assignment.

5. **Use `.get()` for `Option`-returning access.**
   `Index` panics on missing keys; `get()` returns `None`.
