# OCaml vs Rust: Knapsack Problem — Dynamic Programming (Functional)

## Side-by-Side Code

### OCaml

```ocaml
let knapsack items capacity =
  let n = List.length items in
  let items = Array.of_list items in
  let cache = Hashtbl.create 256 in
  let rec solve i cap =
    if i >= n || cap <= 0 then 0
    else match Hashtbl.find_opt cache (i, cap) with
    | Some v -> v
    | None ->
      let (w, v) = items.(i) in
      let without = solve (i+1) cap in
      let with_item = if w <= cap then v + solve (i+1) (cap - w) else 0 in
      let best = max without with_item in
      Hashtbl.add cache (i, cap) best;
      best
  in solve 0 capacity
```

### Rust (idiomatic — memoized top-down, mirrors OCaml)

```rust
pub fn knapsack(items: &[(usize, usize)], capacity: usize) -> usize {
    let mut cache = HashMap::new();
    solve(items, 0, capacity, &mut cache)
}

fn solve(
    items: &[(usize, usize)],
    i: usize,
    cap: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if i >= items.len() || cap == 0 {
        return 0;
    }
    if let Some(&v) = cache.get(&(i, cap)) {
        return v;
    }
    let (w, v) = items[i];
    let without = solve(items, i + 1, cap, cache);
    let with_item = if w <= cap {
        v + solve(items, i + 1, cap - w, cache)
    } else {
        0
    };
    let best = without.max(with_item);
    cache.insert((i, cap), best);
    best
}
```

### Rust (bottom-up DP table)

```rust
pub fn knapsack_bottom_up(items: &[(usize, usize)], capacity: usize) -> usize {
    let n = items.len();
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];
    for i in 1..=n {
        let (w, v) = items[i - 1];
        for c in 0..=capacity {
            dp[i][c] = if w <= c {
                dp[i - 1][c].max(v + dp[i - 1][c - w])
            } else {
                dp[i - 1][c]
            };
        }
    }
    dp[n][capacity]
}
```

### Rust (space-optimised rolling array)

```rust
pub fn knapsack_rolling(items: &[(usize, usize)], capacity: usize) -> usize {
    let mut dp = vec![0usize; capacity + 1];
    for &(w, v) in items {
        for c in (w..=capacity).rev() {
            dp[c] = dp[c].max(v + dp[c - w]);
        }
    }
    dp[capacity]
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val knapsack : (int * int) list -> int -> int` | `fn knapsack(items: &[(usize, usize)], capacity: usize) -> usize` |
| Item list | `(int * int) list` | `&[(usize, usize)]` (borrowed slice of tuples) |
| Memoisation cache | `(int * int, int) Hashtbl.t` | `HashMap<(usize, usize), usize>` |
| DP table | `int array array` (if bottom-up) | `Vec<Vec<usize>>` |

## Key Insights

1. **Closure vs free function for mutable cache:** In OCaml, `let rec solve` freely closes
   over the mutable `Hashtbl`. Rust's borrow checker prevents a recursive closure from
   holding `&mut HashMap` at the same time as borrowing `items`; the fix is to lift the
   helper into a free `fn` that receives both as parameters.

2. **Tuple destructuring is identical:** Both `let (w, v) = items.(i)` (OCaml) and
   `let (w, v) = items[i]` (Rust) destructure a tuple element from an array/slice —
   the surface syntax is nearly the same.

3. **`max` as method vs free function:** OCaml uses `max without with_item` (polymorphic
   built-in); Rust uses `without.max(with_item)`, a method on the concrete type `usize`.
   Both are zero-cost.

4. **Bottom-up avoids stack depth:** For large inputs (thousands of items, large capacity)
   the recursive top-down approach may overflow the stack. The bottom-up table iterates
   instead, which is safer and cache-friendlier.

5. **Rolling array and `rev()`:** The space-optimised variant folds the 2-D table into a
   single row by processing capacity in reverse (`(w..=capacity).rev()`). This ensures
   that `dp[c - w]` still reflects the *previous* item's row — the same invariant the
   2-D table maintains explicitly. OCaml would express the same idea with `Array.blit` or
   a functional fold over a vector.

## When to Use Each Style

**Use top-down memoisation when:** the problem has sparse subproblem structure (not all
`(i, cap)` pairs are reachable) or you want to stay close to the recursive mathematical
definition for readability.

**Use bottom-up table when:** you need predictable stack usage and every subproblem will
be computed regardless — the tight nested loop is also easier to parallelise.

**Use rolling array when:** memory is the bottleneck and you only need the final answer
(not the full DP table for path reconstruction).
