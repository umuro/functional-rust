📖 **[View on hightechmind.io →](https://hightechmind.io/rust/372-skip-list-pattern)**

---

# 372: Skip List Pattern

## Problem Statement

Linked lists support O(1) insert/delete at a known position but O(n) search. Balanced BSTs (AVL, red-black) support O(log n) search with complex rotation logic. Skip lists (William Pugh, 1990) achieve O(log n) expected time for search, insert, and delete using a simpler probabilistic approach: multiple layers of linked lists where each layer skips over elements, creating "express lanes" for fast traversal. Redis's sorted sets, LevelDB's memtable, and HBase use skip lists as their core sorted data structure. This example demonstrates the concept with sorted vectors as simulated express lanes, illustrating the layered search principle.

## Learning Outcomes

- Understand the skip list concept: multiple sorted layers, each a subset of the layer below
- Navigate by starting at the highest layer (largest steps) and moving down
- Use binary search within each level for efficient express-lane searching
- Recognize that real skip lists use probabilistic node promotion (each level has ~50% of elements from below)
- See why skip lists are preferred over balanced BSTs in concurrent settings (easier to make lock-free)
- Implement insert maintaining the sorted invariant across all levels

## Rust Application

```rust
pub struct SkipList {
    level0: Vec<i32>, // all elements (sorted)
    level1: Vec<i32>, // every 2nd element
    level2: Vec<i32>, // every 4th element
}

impl SkipList {
    pub fn new() -> Self {
        Self { level0: Vec::new(), level1: Vec::new(), level2: Vec::new() }
    }

    fn rebuild_levels(&mut self) {
        self.level1 = self.level0.iter().step_by(2).copied().collect();
        self.level2 = self.level0.iter().step_by(4).copied().collect();
    }

    pub fn insert(&mut self, val: i32) {
        match self.level0.binary_search(&val) {
            Ok(_) => return, // already present
            Err(i) => self.level0.insert(i, val),
        }
        self.rebuild_levels();
    }

    pub fn search(&self, val: i32) -> bool {
        // Search top-down: highest level first (fewest elements)
        if self.level2.binary_search(&val).is_ok() { return true; }
        if self.level1.binary_search(&val).is_ok() { return true; }
        self.level0.binary_search(&val).is_ok()
    }
}
```

The simulated skip list uses `step_by(2^level)` to select express-lane elements. In a real skip list, each inserted node is promoted to higher levels with 50% probability (geometric distribution), and layers are linked lists rather than vectors — O(log n) expected height with high probability.

## OCaml Approach

OCaml's standard skip list equivalent is a balanced BST via `Map.Make`:

```ocaml
module IntSet = Set.Make(Int)

(* Real skip list in OCaml requires imperative refs for linked nodes *)
(* Functional approximation: sorted list with layer filtering *)
let level0 = ref []
let level1 () = List.filteri (fun i _ -> i mod 2 = 0) !level0
let level2 () = List.filteri (fun i _ -> i mod 4 = 0) !level0

let insert v =
  if not (List.mem v !level0) then
    level0 := List.sort compare (v :: !level0)

let search v = List.mem v (level2 ()) || List.mem v (level1 ()) || List.mem v !level0
```

In practice, OCaml uses `Set.Make` (AVL tree) for sorted sets — the skip list's main advantage (simpler concurrent implementations) doesn't apply in OCaml's GC-managed environment.

## Key Differences

| Aspect | Rust skip list simulation | OCaml `Set.Make` |
|--------|--------------------------|-----------------|
| Underlying structure | Multiple sorted vectors (simulation) | AVL tree (real O(log n)) |
| Concurrent safety | Real skip lists can be lock-free | `Set` operations require synchronization |
| Insert complexity | O(n) for this simulation (Vec insert) | O(log n) |
| Search complexity | O(log n) per level via binary search | O(log n) |
| Real implementation | `crossbeam-skiplist` crate | Not needed; `Set` suffices |

## Exercises

1. **Probabilistic promotion**: Implement a real skip list where inserted nodes are promoted to higher levels with 50% probability; use `Vec<(i32, usize)>` (value, height) for nodes linked via `Vec<usize>` index arrays.
2. **Range query**: Add `range(lo, hi) -> Vec<i32>` that returns all elements in `[lo, hi]`; use level2's binary search to find the start position, then linear scan through level0.
3. **Delete**: Implement `remove(&mut self, val: i32) -> bool` that removes the value from all levels where it appears, returning whether it was found.
