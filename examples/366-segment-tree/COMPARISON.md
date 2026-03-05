# OCaml vs Rust: Segment Tree

## Side-by-Side Comparison

### Build

**OCaml:**
```ocaml
let rec build_ v l r =
  if l=r then t.data.(v) <- arr.(l)
  else begin
    let m = (l+r)/2 in
    build_ (2*v) l m;
    build_ (2*v+1) (m+1) r;
    t.data.(v) <- t.data.(2*v) + t.data.(2*v+1)
  end
```

**Rust:**
```rust
fn build(&mut self, arr: &[i64], v: usize, l: usize, r: usize) {
    if l == r {
        self.data[v] = arr[l];
        return;
    }
    let m = (l + r) / 2;
    self.build(arr, 2 * v, l, m);
    self.build(arr, 2 * v + 1, m + 1, r);
    self.data[v] = self.data[2 * v] + self.data[2 * v + 1];
}
```

### Query

**OCaml:**
```ocaml
let rec query t v l r ql qr =
  if qr < l || r < ql then 0
  else if ql <= l && r <= qr then t.data.(v)
  else
    let m = (l+r)/2 in
    query t (2*v) l m ql qr + query t (2*v+1) (m+1) r ql qr
```

**Rust:**
```rust
fn query_internal(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
    if qr < l || r < ql { return 0; }
    if ql <= l && r <= qr { return self.data[v]; }
    let m = (l + r) / 2;
    self.query_internal(2*v, l, m, ql, qr) + self.query_internal(2*v+1, m+1, r, ql, qr)
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Index type | `int` | `usize` |
| Array access | `arr.(i)` | `arr[i]` |
| Recursion | Natural | Same |
| Mutability | Mutable record | `&mut self` |
