# OCaml vs Rust: 0/1 Knapsack — Memoized Recursion

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

### Rust (idiomatic — memoized recursion)
```rust
pub fn knapsack(items: &[Item], capacity: usize) -> u64 {
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
    solve(items, 0, capacity, &mut memo)
}

fn solve(items: &[Item], i: usize, cap: usize,
         memo: &mut HashMap<(usize, usize), u64>) -> u64 {
    if i >= items.len() || cap == 0 { return 0; }
    if let Some(&cached) = memo.get(&(i, cap)) { return cached; }
    let item = items[i];
    let without = solve(items, i + 1, cap, memo);
    let with_item = if item.weight <= cap {
        item.value + solve(items, i + 1, cap - item.weight, memo)
    } else { 0 };
    let best = without.max(with_item);
    memo.insert((i, cap), best);
    best
}
```

### Rust (bottom-up tabulation)
```rust
pub fn knapsack_tabulation(items: &[Item], capacity: usize) -> u64 {
    let n = items.len();
    let mut dp = vec![vec![0u64; capacity + 1]; n + 1];
    for i in 1..=n {
        let item = items[i - 1];
        for w in 0..=capacity {
            dp[i][w] = dp[i - 1][w];
            if item.weight <= w {
                let take = item.value + dp[i - 1][w - item.weight];
                if take > dp[i][w] { dp[i][w] = take; }
            }
        }
    }
    dp[n][capacity]
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Memo table | `Hashtbl.t` (captured by closure) | `HashMap<(usize, usize), u64>` (passed as `&mut`) |
| Item type | `(int * int)` tuple | `struct Item { weight: usize, value: u64 }` |
| Cache lookup | `Hashtbl.find_opt cache (i, cap)` | `memo.get(&(i, cap))` |
| Cache insert | `Hashtbl.add cache (i, cap) best` | `memo.insert((i, cap), best)` |
| Result type | `int` | `u64` |

## Key Insights

1. **Mutable state threading**: Rust requires the memo table to be passed as `&mut HashMap` through every recursive call, making data flow explicit. OCaml closures capture `Hashtbl` by reference implicitly — less boilerplate but implicit mutation.
2. **Cache key**: Both use `(item_index, capacity)` as the cache key. Rust's `HashMap` requires `Hash + Eq` on keys; `(usize, usize)` derives both automatically.
3. **Closure vs. function**: OCaml's `let rec solve i cap = ...` is a closure capturing `cache`, `items`, and `n` from the outer scope. Rust uses a standalone function with explicit parameters — the borrow checker ensures `memo` is not aliased unsafely.
4. **Top-down vs. bottom-up**: Memoized recursion (top-down) only computes subproblems actually reached; bottom-up tabulation fills the entire DP table. The memoized version can be faster when many subproblems are unreachable.
5. **Stack depth**: Both approaches risk stack overflow for very deep recursion (large n and capacity). The tabulation version avoids recursion entirely.

## When to Use Each Style

**Use memoized recursion when:** the recursion structure maps naturally to the problem, many subproblems are unreachable, or you want a direct correspondence with the mathematical definition.
**Use bottom-up tabulation when:** all subproblems will be needed, you want predictable O(n × W) time and space with no recursion overhead, or stack depth is a concern.
