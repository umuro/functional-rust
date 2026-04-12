**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[count-min-sketch on hightechmind.io](https://hightechmind.io/posts/functional-rust/count-min-sketch)

---

## Problem Statement

Implement a Count-Min Sketch — a probabilistic frequency estimation structure that uses O(depth × width) space independent of the number of distinct keys. Insert items and query frequency in O(depth) time. The frequency estimate equals the minimum counter across all rows, providing an upper bound with tunable error probability.

## Learning Outcomes

- Implement a `depth × width` counter table: `Vec<Vec<u64>>`
- Define `depth` independent hash functions by parameterizing with different seeds
- Implement `update(key, delta)` that increments one column per row
- Implement `query(key) -> u64` that returns `min` over all row counters — always >= true frequency
- Understand the error bound: with width `w = ceil(e/ε)` and depth `d = ceil(ln(1/δ))`, the estimate exceeds true count by more than `ε * N` with probability at most `δ`

## Rust Application

```rust
fn hash(seed: u64, s: &str) -> u64 {
    s.bytes().fold(seed, |h, b| {
        h.wrapping_mul(seed).wrapping_add(b as u64) ^ b as u64
    })
}

pub struct CountMinSketch {
    table: Vec<Vec<u64>>,
    seeds: Vec<u64>,
    width: usize,
    depth: usize,
}

impl CountMinSketch {
    pub fn new(width: usize, depth: usize) -> Self {
        let seeds = vec![31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        CountMinSketch {
            table: vec![vec![0u64; width]; depth],
            seeds: (0..depth).map(|i| seeds[i % seeds.len()]).collect(),
            width, depth,
        }
    }

    fn column(&self, row: usize, key: &str) -> usize {
        (hash(self.seeds[row], key) as usize) % self.width
    }

    pub fn update(&mut self, key: &str, delta: u64) {
        for i in 0..self.depth {
            let col = self.column(i, key);
            self.table[i][col] += delta;
        }
    }

    pub fn query(&self, key: &str) -> u64 {
        (0..self.depth)
            .map(|i| self.table[i][self.column(i, key)])
            .min()
            .unwrap_or(0)
    }
}
```

Each row uses a different hash seed, creating independent hash functions. `update` increments one cell per row; `query` reads one cell per row and returns the minimum. The minimum is chosen because hash collisions can only inflate estimates, never deflate them.

With `depth = 5` rows and `width = 2000` columns, the sketch uses 80KB (10,000 × 8 bytes) and provides frequency estimates with error at most `N/2000` with probability 1 - (1/2)^5 = 97%.

## OCaml Approach

```ocaml
let hash seed s =
  String.fold_left (fun h b ->
    Int64.(to_int (logxor
      (add (mul (of_int h) (of_int seed)) (of_int (Char.code b)))
      (of_int (Char.code b))))
  ) (Int64.to_int seed) s

type t = {
  table: int array array;
  seeds: int array;
  width: int;
  depth: int;
}

let create width depth =
  let seeds = [|31;37;41;43;47;53;59;61;67;71|] in
  { table = Array.init depth (fun _ -> Array.make width 0);
    seeds = Array.init depth (fun i -> seeds.(i mod 10));
    width; depth }

let update cms key delta =
  Array.iteri (fun i seed ->
    let col = (abs (hash seed key)) mod cms.width in
    cms.table.(i).(col) <- cms.table.(i).(col) + delta
  ) cms.seeds

let query cms key =
  Array.fold_left (fun acc (i, seed) ->
    min acc cms.table.(i).((abs (hash seed key)) mod cms.width)
  ) max_int (Array.mapi (fun i s -> (i, s)) cms.seeds)
```

OCaml's `Int64` arithmetic for hashing avoids the GC tag issue with overflow. The sketch structure is identical; only the hash implementation differs in verbosity.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Hash overflow | `wrapping_mul/add` — defined behavior | `Int64` wrapping or `land max_int` |
| Counter type | `u64` — no overflow for most counts | `int` — 63-bit; risk for very large counts |
| `min` over rows | `.min().unwrap_or(0)` | `Array.fold_left min max_int ...` |
| Table allocation | `vec![vec![0u64; width]; depth]` | `Array.init depth (fun _ -> Array.make ...)` |

Count-Min Sketches are used in network traffic analysis, database query optimization, and streaming analytics. They trade exactness for bounded space — ideal when the key space is too large for a full frequency map.

## Exercises

1. Implement `merge(other: &CountMinSketch)` that adds two sketches of equal dimensions (element-wise sum).
2. Add `decay(&mut self, factor: f64)` that multiplies all counters by `factor` for time-decayed frequency estimation.
3. Verify the error bound experimentally: insert 10,000 items, query all of them, measure maximum overcount.
4. Implement a heavy hitter detector: items whose estimated frequency exceeds `N/k` are "heavy hitters".
5. Compare memory usage of `CountMinSketch` vs `HashMap<String, u64>` for 100,000 distinct keys.
