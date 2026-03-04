# 027: Group By Size

**Difficulty:** 3  **Level:** Foundations

Partition a list into disjoint groups of specified sizes — generating all possible ways to make that partition.

## The Problem This Solves

You need to divide a team of 9 people into groups: a pair, a trio, and a quartet — and you want to enumerate *all* possible ways to do that. This is the "group problem" from combinatorics, used in tournament scheduling, work assignment, experimental design, and partition-based algorithms.

Given `['Alice', 'Bob', 'Carol', 'Dave']` and sizes `[2, 1, 1]`, the function returns all 12 ways to divide those 4 people into a group of 2, a group of 1, and another group of 1. Order within groups doesn't matter; order between groups does (group 1 is different from group 2 even if they're the same size).

This is harder than plain combinations because it's a *layered* problem: first choose the first group, then from the remainder choose the second group, and so on — recursively.

## The Intuition

**Think of it as nested combinations.** To form groups of sizes `[k1, k2, k3, ...]`:

1. Choose `k1` elements from the full list for group 1 (C(n, k1) ways)
2. For each such choice, choose `k2` elements from what's left for group 2
3. Continue until all sizes are exhausted

This is exactly the recursive structure in the code. The helper `combinations` function returns `(chosen, remainder)` pairs — "here's your group, here's what's left over" — so each recursive call can take the remainder as its new input.

The total count for 9 people into `[2, 3, 4]` is C(9,2) × C(7,3) × C(4,4) = 36 × 35 × 1 = 1260.

## How It Works in Rust

```rust
// Returns (chosen_group, leftover) pairs — threads the remainder through
fn combinations<T: Clone>(k: usize, lst: &[T]) -> Vec<(Vec<T>, Vec<T>)> {
    if k == 0 {
        return vec![(vec![], lst.to_vec())];  // empty group, everything left over
    }
    let mut result = Vec::new();
    for i in 0..lst.len() {
        let chosen = lst[i].clone();
        // rest = everything except position i
        let rest: Vec<T> = lst[..i].iter().chain(lst[i+1..].iter()).cloned().collect();
        for (mut combo, remainder) in combinations(k - 1, &rest) {
            combo.insert(0, chosen.clone());
            result.push((combo, remainder));
        }
    }
    result
}

fn group<T: Clone>(lst: &[T], sizes: &[usize]) -> Vec<Vec<Vec<T>>> {
    if sizes.is_empty() {
        return vec![vec![]];  // no more groups needed: one empty partition
    }
    let k = sizes[0];
    let mut result = Vec::new();
    for (chosen, remainder) in combinations(k, lst) {
        for mut sub_groups in group(&remainder, &sizes[1..]) {
            sub_groups.insert(0, chosen.clone());  // prepend this group
            result.push(sub_groups);
        }
    }
    result
}
```

**Key design:** `combinations` returns the *remainder* alongside each chosen group. This avoids needing to compute `lst - chosen` at the `group` level — the threading is done inside `combinations` itself.

**Reading the type:** `Vec<Vec<Vec<T>>>` — a list of partitions, where each partition is a list of groups, where each group is a list of elements.

## What This Unlocks

- **Team assignment** — enumerate all ways to divide N people into predefined team sizes.
- **Tournament scheduling** — generate round-robin brackets with groups of fixed size.
- **Experimental design** — partition subjects into treatment and control groups in all possible ways.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Threading remainder | `combinations` returns `(chosen, rest)` | Same pattern: `(Vec<T>, Vec<T>)` tuple |
| List difference | `List.filter (fun x -> x <> chosen)` | Slice with index exclusion: `[..i] ++ [i+1..]` |
| Result type | `'a list list list` | `Vec<Vec<Vec<T>>>` |
| Recursive base | `sizes = [] → [[]]` | `sizes.is_empty() → vec![vec![]]` |
| Group prepend | `chosen :: sub_groups` | `sub_groups.insert(0, chosen.clone())` |
