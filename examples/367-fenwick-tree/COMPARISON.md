# OCaml vs Rust: Fenwick Tree

## Side-by-Side Comparison

### Update

**OCaml:**
```ocaml
let update i v =
  let i = ref i in
  while !i <= n do
    tree.(!i) <- tree.(!i) + v;
    i := !i + (!i land (- !i))
  done
```

**Rust:**
```rust
fn update(&mut self, mut i: usize, delta: i64) {
    while i <= self.n {
        self.tree[i] += delta;
        i += i & i.wrapping_neg();
    }
}
```

### Prefix Sum

**OCaml:**
```ocaml
let prefix_sum i =
  let i = ref i and s = ref 0 in
  while !i > 0 do
    s := !s + tree.(!i);
    i := !i - (!i land (- !i))
  done;
  !s
```

**Rust:**
```rust
fn prefix_sum(&self, mut i: usize) -> i64 {
    let mut sum = 0;
    while i > 0 {
        sum += self.tree[i];
        i -= i & i.wrapping_neg();
    }
    sum
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Lowbit | `i land (-i)` | `i & i.wrapping_neg()` |
| Iteration | `while` with ref | `while` with `mut` |
| Negation | `-i` (signed) | `wrapping_neg()` |
| Index base | 1-indexed | 1-indexed |

## Lowbit Explanation

The "lowbit" operation extracts the lowest set bit:
- `lowbit(6)` = `lowbit(110₂)` = `010₂` = 2
- `lowbit(8)` = `lowbit(1000₂)` = `1000₂` = 8

This is computed as `i & (-i)` using two's complement.
