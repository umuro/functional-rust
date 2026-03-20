📖 **[View on hightechmind.io →](https://hightechmind.io/rust/841-backtracking-framework)**

---

# Backtracking Framework
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Many combinatorial optimization and enumeration problems — N-queens, Sudoku solving, permutation generation, subset enumeration, graph coloring, constraint satisfaction — require exploring a search space that grows exponentially with input size. Backtracking systematically explores candidates and abandons (backtracks from) any partial solution as soon as it determines it cannot lead to a valid complete solution. This pruning makes backtracking vastly more efficient than brute force for well-constrained problems, though still exponential in the worst case. Understanding backtracking as a reusable framework — not just specific puzzles — enables applying it to new combinatorial problems.

## Learning Outcomes

- Implement the backtracking template: `choose → explore → unchoose`
- Use a `used` flag array to avoid revisiting elements in permutation generation
- Recognize constraint-based pruning: skip choices that violate constraints early
- Apply to N-queens: test placement validity before recursing, backtrack on conflict
- Understand the difference between backtracking (exact, complete) and heuristic search (approximate)

## Rust Application

```rust
pub fn generate_permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut result = vec![];
    let mut current = vec![];
    let mut used = vec![false; items.len()];
    fn backtrack<T: Clone>(items: &[T], used: &mut Vec<bool>,
                            current: &mut Vec<T>, result: &mut Vec<Vec<T>>) {
        if current.len() == items.len() { result.push(current.clone()); return; }
        for i in 0..items.len() {
            if used[i] { continue; }
            used[i] = true; current.push(items[i].clone());
            backtrack(items, used, current, result);
            used[i] = false; current.pop(); // unchoose
        }
    }
    backtrack(items, &mut used, &mut current, &mut result);
    result
}
```

The nested `fn backtrack` captures the mutable references from the outer scope. The `used` flag array tracks which elements are in the current path, enabling O(n) per-choice overhead instead of O(n) vec contains check. The `current.push` / `current.pop` pattern is the canonical "choose/unchoose" backtracking structure. Generic `<T: Clone>` makes this work for any cloneable type. The `result.push(current.clone())` clones the current path at leaf nodes — the only allocation cost.

## OCaml Approach

OCaml backtracking uses a `used` array as `bool array` mutated in-place. The recursive function returns `unit` and accumulates results via an `acc ref`. OCaml's functional style can also express this with continuation-passing: `let rec backtrack cont current used = ...`. The `Array.make n false` initializes the used flags. OCaml's pattern matching for N-queens constraint checking reads naturally. The `List.rev current` at leaf nodes (if building the list from head prepending) recovers the correct order.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Used flags | `Vec<bool>` | `bool array` |
| Result accumulation | `&mut Vec<Vec<T>>` | `acc ref` or continuation |
| Choose/unchoose | `push`/`pop` on `Vec` | `Array.set` true/false |
| Generic type | `<T: Clone>` | Parametric `'a` |
| N-queens variant | Separate `fn queens` | Separate or same framework |
| Iterative variant | `Iterator` via generator | Sequence with `Seq.t` |

## Exercises

1. Implement N-queens backtracking using this framework: add a validity check before recursing.
2. Implement Sudoku solver using backtracking with forward checking (propagate constraints after each placement).
3. Add an `Iterator` interface to the backtracking framework so permutations can be consumed lazily.
4. Implement combination generation (choose k from n) and verify the count equals C(n, k).
5. Apply backtracking to graph coloring: find a k-coloring of a graph or report that none exists.
