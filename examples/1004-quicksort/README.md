# Example 1004: Quicksort with Higher-Order Comparators

## Learning Outcomes

After studying this example, you should understand:

1. **Functional vs. Imperative Sorting**: How the OCaml version uses immutable data structures and recursive composition, versus Rust's preference for in-place mutations and ownership-based design.

2. **Higher-Order Functions in Rust**: How to pass comparator functions across recursive calls using trait bounds (`F: Fn(...) + Copy`) and why they must implement `Copy` in recursive contexts.

3. **Ownership and Borrowing**: Why Rust requires explicit handling of function closures in recursive scenarios, and how this differs from OCaml's garbage collection model.

4. **Performance Trade-offs**: Why idiomatic Rust uses `std::sort` (introsort: quicksort → heapsort) over manual quicksort, even though the algorithm is the same.

5. **Type Safety**: How Rust's type system enforces correctness at compile-time, preventing entire classes of bugs that OCaml might catch at runtime.

---

## OCaml Approach

### Key Characteristics

- **Immutable by default**: Lists are immutable; partitioning creates new lists
- **Pattern matching**: Elegant deconstruction of list structures
- **Lazy evaluation potential**: Can be composed with other lazy operations
- **Garbage collection**: No explicit memory management

### OCaml Code

```ocaml
let rec quicksort gt = function
  | [] -> []
  | x::xs ->
      let ys, zs = List.partition (gt x) xs in
      (quicksort gt ys) @ (x :: (quicksort gt zs))

let _ = quicksort (>) [4; 65; 2; -31; 0; 99; 83; 782; 1]
```

### Algorithm

1. **Base case**: Empty list returns empty
2. **Recursive case**:
   - Take head `x` as pivot
   - Partition tail `xs` into `ys` (elements > pivot) and `zs` (elements ≤ pivot)
   - Recursively sort both partitions
   - Concatenate: `left @ [pivot] @ right`

**Time Complexity**: O(n²) worst-case (unbalanced pivot), O(n log n) average-case  
**Space Complexity**: O(n) due to list copying and recursion stack

---

## Rust Approach

### Functional Quicksort (Rust)

This implementation mirrors the OCaml version as closely as Rust semantics allow:

```rust
pub fn quicksort<T: Clone + Ord, F: Fn(&T, &T) -> bool + Copy>(
    gt: F,
    mut xs: Vec<T>,
) -> Vec<T> {
    if xs.is_empty() {
        return xs;
    }

    let pivot = xs.remove(0);
    let (ys, zs): (Vec<T>, Vec<T>) = xs.into_iter().partition(|x| gt(&pivot, x));

    let mut left = quicksort(gt, ys);
    let mut result = quicksort(gt, zs);
    left.push(pivot);
    left.append(&mut result);
    left
}
```

**Differences from OCaml:**
- Requires `Clone` for elements and `Copy` for the comparator function
- Uses `Vec` instead of linked lists (better cache locality)
- Mutates in-place where beneficial (`.push()`, `.append()`)

### Idiomatic Rust Quicksort

The **preferred production approach** uses `std::sort`:

```rust
pub fn quicksort_idiomatic<T: Ord>(xs: Vec<T>) -> Vec<T> {
    let mut result = xs;
    result.sort();
    result
}
```

**Why?**
- Uses **introsort**: hybrid of quicksort + heapsort + insertion sort
- Proven, optimized implementation in the standard library
- Better worst-case behavior (O(n log n) guaranteed)
- Handles cache efficiency automatically
- Battle-tested across decades of Rust code

---

## 4 Key Differences Between OCaml and Rust

### 1. **Function Trait Bounds vs. First-Class Functions**

| OCaml | Rust |
|-------|------|
| Functions are first-class; no special syntax needed | Functions must be constrained with trait bounds: `F: Fn(...) + Copy` |
| Closures are created implicitly | Closures are explicit `\|x\| { ... }` syntax |
| Can be passed freely across recursive calls | Must implement `Copy` in recursive contexts (or use references) |

**Why?** Rust's ownership system requires knowing at compile-time whether a function can be moved or must be copied. OCaml's garbage collection handles this implicitly.

---

### 2. **Immutability by Default vs. Ownership-Based Mutation**

| OCaml | Rust |
|-------|------|
| Lists are immutable; `@` concatenation creates new lists | Vecs can be mutated; `.push()` and `.append()` modify in-place |
| Memory management is automatic | Ownership model enforces single owner; must explicitly `move` or `clone` |
| Pattern matching on immutable structures is natural | Rust can mutate when semantically necessary (e.g., `.remove(0)`) |

**Example:**
- OCaml: `(quicksort gt ys) @ (x :: (quicksort gt zs))` — creates new list structure
- Rust: `left.append(&mut result)` — mutates `left` in-place

---

### 3. **List Structure vs. Vector Storage**

| OCaml | Rust |
|-------|------|
| Linked lists (`::`) — O(1) head removal, O(n) traversal | Vectors — O(n) removal, O(1) indexed access |
| Pattern matching on list structure is idiomatic | `.partition()` is idiomatic; destructuring is less common |
| Cache locality is poor; pointer-chasing is slow | Cache-friendly; elements are contiguous |

**Trade-off:** OCaml linked lists are elegant for recursion but slow in practice. Rust vectors require `.remove(0)` which is O(n), but the overall performance is better due to cache behavior.

---

### 4. **Algorithm Selection: Quicksort vs. Introsort**

| Approach | OCaml | Rust |
|----------|-------|------|
| **Manual Quicksort** | ✅ Idiomatic, works well with immutability | ✓ Possible, but not recommended |
| **Library Sorting** | ❌ Not idiomatic (rarely used) | ✅ **Strongly preferred** (`std::sort`) |
| **Worst-Case Complexity** | O(n²) without safeguards | O(n log n) guaranteed (introsort) |
| **Production Code** | Manual quicksort acceptable | Always use `std::sort` |

**Why the difference?**

- **OCaml** emphasizes algorithmic clarity. Manual quicksort is educational and matches functional programming patterns.
- **Rust** emphasizes reliability and performance. Introsort (the stdlib implementation) is battle-tested, handles edge cases, and has better cache behavior.

In Rust, there's a saying: *"Don't implement sorting yourself."* The standard library has already solved all the hard problems.

---

## Complexity Analysis

| Variant | Time (avg) | Time (worst) | Space |
|---------|-----------|-------------|-------|
| Functional Quicksort (OCaml/Rust) | O(n log n) | O(n²) | O(n) + O(log n) stack |
| Idiomatic Rust (`std::sort`) | O(n log n) | O(n log n) | O(n) + O(log n) stack |
| Insertion Sort (small arrays) | O(n²) | O(n²) | O(1) |

**Introsort strategy:**
1. Start with quicksort
2. If recursion depth exceeds 2 log n, switch to heapsort
3. Use insertion sort for arrays < 16 elements

---

## Testing

All implementations are tested with:

- ✅ Empty lists
- ✅ Single elements
- ✅ Multiple elements in various orders
- ✅ Already sorted (best/worst case for quicksort)
- ✅ Reverse sorted (worst case for naive quicksort)
- ✅ Duplicates
- ✅ Custom comparators (ascending/descending)
- ✅ Negative numbers

**Total: 11 tests, all passing**

```bash
$ cargo test --lib
running 11 tests
test result: ok. 11 passed
```

---

## Further Reading

- [OCaml Pattern Matching](https://ocaml.org/docs/pattern-matching)
- [Rust Higher-Order Functions and Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Introsort Algorithm](https://en.wikipedia.org/wiki/Introsort)
- [Rust std::sort Implementation](https://github.com/rust-lang/rust/blob/master/library/alloc/src/slice.rs)
