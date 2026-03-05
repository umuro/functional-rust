# OCaml vs Rust: Union-Find (Disjoint Set)

## Side-by-Side Comparison

### Data Structure

**OCaml:**
```ocaml
let parent = Array.init 10 (fun i -> i)
let rank   = Array.make 10 0
```

**Rust:**
```rust
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u32>,
    size: Vec<usize>,
    components: usize,
}
```

### Find with Path Compression

**OCaml:**
```ocaml
let rec find x =
  if parent.(x) = x then x
  else begin
    parent.(x) <- find parent.(x);  (* path compression *)
    parent.(x)
  end
```

**Rust:**
```rust
fn find(&mut self, x: usize) -> usize {
    if self.parent[x] != x {
        self.parent[x] = self.find(self.parent[x]); // path compression
    }
    self.parent[x]
}
```

### Union by Rank

**OCaml:**
```ocaml
let union x y =
  let rx = find x and ry = find y in
  if rx = ry then ()
  else if rank.(rx) < rank.(ry) then parent.(rx) <- ry
  else if rank.(rx) > rank.(ry) then parent.(ry) <- rx
  else begin parent.(ry) <- rx; rank.(rx) <- rank.(rx)+1 end
```

**Rust:**
```rust
fn union(&mut self, x: usize, y: usize) -> bool {
    let rx = self.find(x);
    let ry = self.find(y);
    if rx == ry { return false; }
    
    if self.rank[rx] < self.rank[ry] {
        self.parent[rx] = ry;
    } else if self.rank[rx] > self.rank[ry] {
        self.parent[ry] = rx;
    } else {
        self.parent[ry] = rx;
        self.rank[rx] += 1;
    }
    self.components -= 1;
    true
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Return value | `unit` | `bool` (was different?) |
| Component tracking | Manual | Built into struct |
| Size tracking | Not included | Included |
| Mutability | Global arrays | `&mut self` |
| Encapsulation | Module-level | Struct methods |

## Complexity

Both achieve the same optimal complexity:

| Operation | Complexity |
|-----------|------------|
| Find | O(α(n)) amortized |
| Union | O(α(n)) amortized |
| Connected | O(α(n)) amortized |

Where α(n) is the inverse Ackermann function, effectively ≤ 4 for all practical n.

## Optimizations

**Path Compression:** After finding a root, update all nodes on the path to point directly to the root.

**Union by Rank:** Always attach the shorter tree under the root of the taller tree.

Together, these make operations essentially O(1) in practice.

## Use Cases

| Use Case | Description |
|----------|-------------|
| Kruskal's MST | Check if edge creates cycle |
| Network connectivity | Online "are A and B connected?" |
| Cycle detection | `union()` returns false = cycle |
| Image segmentation | Group connected pixels |
