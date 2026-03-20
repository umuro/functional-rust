**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

[skip-list on hightechmind.io](https://hightechmind.io/posts/functional-rust/skip-list)

---

## Problem Statement

Implement a skip list — a probabilistic sorted data structure providing O(log n) average search, insert, and delete. Use an arena-based approach: nodes are stored in a `Vec` and referenced by index instead of raw pointers. Each node has a `forward: Vec<usize>` of index references to nodes at each level. Use a deterministic xorshift PRNG for reproducible tests.

## Learning Outcomes

- Implement an arena allocator pattern: `Vec<SkipListNode>` where node 0 is the header sentinel
- Implement `random_level()` using a geometric distribution: keep incrementing level while PRNG output < P=0.5, up to MAX_LEVEL
- Implement `search(value) -> bool` by descending from max level and advancing `forward[level]` while value is too small
- Implement `insert(value)` using the same top-down search with an `update` array tracking the predecessor at each level
- Understand why index-based arenas avoid raw pointer unsafety in Rust while still achieving O(log n) operations

## Rust Application

```rust
const MAX_LEVEL: usize = 8;
const P: f64 = 0.5;

struct SkipListNode {
    value: i64,
    forward: Vec<usize>,  // 0 = null sentinel (header index)
}

pub struct SkipList {
    nodes: Vec<SkipListNode>,
    level: usize,
    rng: Rng,
}

impl SkipList {
    pub fn new(seed: u64) -> Self {
        let header = SkipListNode {
            value: i64::MIN,
            forward: vec![0; MAX_LEVEL],  // all forward pointers = 0 (null)
        };
        SkipList { nodes: vec![header], level: 1, rng: Rng::new(seed) }
    }

    pub fn search(&self, value: i64) -> bool {
        let mut current = 0;  // start at header
        for lvl in (0..self.level).rev() {
            while self.nodes[self.nodes[current].forward[lvl]].value < value {
                current = self.nodes[current].forward[lvl];
            }
        }
        let next = self.nodes[current].forward[0];
        next != 0 && self.nodes[next].value == value
    }

    pub fn insert(&mut self, value: i64) {
        let mut update = vec![0usize; MAX_LEVEL];
        let mut current = 0;
        for lvl in (0..self.level).rev() {
            while self.nodes[self.nodes[current].forward[lvl]].value < value {
                current = self.nodes[current].forward[lvl];
            }
            update[lvl] = current;
        }
        let new_level = self.rng.random_level();
        // ... allocate node and update forward pointers
    }
}
```

The arena pattern (index 0 = header sentinel) avoids `Option<Box<Node>>` chains and raw pointer indirection. Forward pointers are array indices — safe in Rust and cache-friendlier than heap-allocated node pointers.

The header node has value `i64::MIN` so the `while value > forward.value` loop always terminates without checking for null explicitly.

## OCaml Approach

```ocaml
type node = {
  value: int;
  mutable forward: int array;  (* indices into node array *)
}

type t = {
  nodes: node DynArray.t;  (* resizable array *)
  mutable level: int;
  rng: unit -> float;
}

let search sl v =
  let current = ref 0 in
  for lvl = sl.level - 1 downto 0 do
    while (DynArray.get sl.nodes (DynArray.get sl.nodes !current).forward.(lvl)).value < v do
      current := (DynArray.get sl.nodes !current).forward.(lvl)
    done
  done;
  let next = (DynArray.get sl.nodes !current).forward.(0) in
  next <> 0 && (DynArray.get sl.nodes next).value = v
```

OCaml's approach is structurally identical — arena-based with mutable `forward` arrays. The key difference is that OCaml uses `DynArray` (Batteries) or `Array.t ref` for the arena, while Rust uses a `Vec`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Arena | `Vec<SkipListNode>` | `DynArray.t` or `Array.t ref` |
| Null pointer | Index `0` (header sentinel) | Same convention |
| Raw pointers | Not needed — indices only | Not needed with arena |
| PRNG | Inline xorshift (deterministic) | `Random.float` (stdlib) |
| Level randomization | Geometric distribution, P=0.5 | Same |

Skip lists provide similar O(log n) average performance to balanced BSTs but are simpler to implement. They are used in Redis (sorted sets), LevelDB, and various in-memory databases.

## Exercises

1. Implement `delete(value) -> bool` using the same `update` array as `insert`.
2. Implement `range_query(l, r) -> Vec<i64>` that returns all values in `[l, r]` in sorted order.
3. Implement `rank(value) -> Option<usize>` that returns the position of `value` in sorted order.
4. Make the skip list generic: `SkipList<T: Ord>` rather than `SkipList` with `i64`.
5. Benchmark skip list vs `BTreeSet` for 100,000 random insertions and lookups.
