📖 **[View on hightechmind.io →](https://hightechmind.io/rust/027-group-by-size)**

---

# 027 — Group the Elements of a Set into Disjoint Subsets
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Partitioning a set into groups of specified sizes (OCaml 99 Problems #27) — for example, dividing 9 people into groups of 2, 3, and 4 — generalizes the combinations problem. This is multinomial selection: choose the first group, then choose the second group from the remainder, and so on. The number of ways to partition n elements into groups of sizes k1, k2, ..., km is the multinomial coefficient n! / (k1! · k2! · ... · km!).

This problem appears in sports scheduling (dividing teams into pools), committee selection, machine learning data splitting (train/validation/test), and parallel task distribution. The recursive structure — select one group, recurse on the rest — is a generalization of backtracking.

## Learning Outcomes

- Compose the combinations operation recursively to form disjoint groups
- Understand multinomial coefficients as iterated binomial coefficients
- Handle groups of different sizes specified as a list
- Recognize partitioning as a key operation in algorithm design
- Use the combinations result from example 026 as a building block

- Build group-by-size by iterating through sizes and using `combinations` to select each group from the remaining elements
- The total number of groupings is the multinomial coefficient: n! / (k1! * k2! * ... * km!)

## Rust Application

The implementation selects the first group of size `sizes[0]` from the full list using combinations, then for each such selection it recursively groups the remaining elements according to `sizes[1..]`. The base case is when `sizes` is empty: return one empty partition. Collecting all results produces all valid partitions. The key is tracking which elements remain after each selection.

## OCaml Approach

OCaml's version: `let group lst sizes = let rec aux lst sizes = match sizes with | [] -> [[]] | k :: rest -> List.concat_map (fun combo -> let remaining = List.filter (fun x -> not (List.mem x combo)) lst in List.map (fun groups -> combo :: groups) (aux remaining rest)) (combinations k lst) in aux lst sizes`. This selects a combination of size k, removes those elements from the pool, then recursively groups the remainder.

OCaml's group operation: `let group list sizes = match sizes with | [] -> [[]] | k :: ks -> List.concat_map (fun c -> List.map (fun g -> c :: g) (group (List.filter (fun x -> not (List.mem x c)) list) ks)) (combinations k list)`. For each combination of size k, recursively group the remaining elements. The multinomial coefficient n! / (k1! · k2! · ... · km!) counts the results.

## Key Differences

1. **List.mem vs HashSet**: OCaml's `List.filter ... not (List.mem x combo)` is O(n·k) per combination. Rust should use a `HashSet` for O(1) membership testing when removing selected elements.
2. **Index-based removal**: Rust can track which elements remain by their indices rather than values, avoiding the equality check for non-`Eq` types.
3. **Clone depth**: Each partition result contains clones of the original elements. Rust's clone cost scales with the partition depth. OCaml shares structure via GC.
4. **Explosion in output size**: For 9 elements in groups of 2, 3, 4, there are C(9,2)·C(7,3)·C(4,4) = 36·35·1 = 1260 partitions. Large inputs explode quickly — always check the expected output size.

1. **Recursive composition:** Both implementations select a group of size k using `combinations`, then recurse on the remainder. The structure is identical — composition of previous solutions.
2. **Element removal:** Removing selected elements from the pool uses different idioms: OCaml uses `List.filter (fun x -> not (List.mem x c))` (O(k·n)); Rust uses index-based exclusion or `HashSet`.
3. **Result type:** `Vec<Vec<Vec<T>>>` in Rust (list of groupings, each grouping is a list of groups). Deeply nested types benefit from type aliases for readability.

## Exercises

1. **Equal groups**: Specialize to `equal_groups(list: &[i32], k: usize) -> Vec<Vec<Vec<i32>>>` that divides the list into groups all of size k. Handle the case where `list.len() % k != 0`.
2. **Count partitions**: Write `count_groups(n: u64, sizes: &[u64]) -> u64` that computes the number of partitions using the multinomial formula without generating them.
3. **Round-robin tournament**: Given 8 teams, generate all possible round-robin schedules by partitioning them into 4 pairs each round, for 7 rounds. Use the group function.

4. **All permutations of groups**: After generating all possible groupings, generate all distinct orderings of those groups (the multinomial coefficient counts unordered groups; ordered groups multiply by k1! * k2! * ...).
5. **Balanced partition**: Implement `balanced_split<T: Clone>(list: &[T]) -> Vec<(Vec<T>, Vec<T>)>` that generates all ways to split a list into two roughly equal halves.
