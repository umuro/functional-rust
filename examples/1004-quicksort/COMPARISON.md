# Side-by-Side: OCaml vs. Rust Quicksort

## Code Comparison

### OCaml Version

```ocaml
(* Recursive quicksort with higher-order comparator *)
let rec quicksort gt = function
  | [] -> []
  | x::xs ->
      let ys, zs = List.partition (gt x) xs in
      (quicksort gt ys) @ (x :: (quicksort gt zs))

(* Usage *)
let _ = quicksort (>) [4; 65; 2; -31; 0; 99; 83; 782; 1]
(* Result: [-31; 0; 1; 2; 4; 65; 83; 99; 782] *)
```

**Line count:** 5 lines (excluding comments)  
**Syntax highlights:**
- Pattern matching on list structure: `| [] -> []` and `| x::xs ->`
- Built-in `List.partition` for filtering
- Infix `@` operator for list concatenation
- Higher-order function `gt` passed implicitly

---

### Rust Version (Functional Style)

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

// Usage
let sorted = quicksort(|a, b| a > b, vec![4, 65, 2, -31, 0, 99, 83, 782, 1]);
// Result: vec![-31, 0, 1, 2, 4, 65, 83, 99, 782]
```

**Line count:** 18 lines (vs. 5 in OCaml)  
**Syntax highlights:**
- Explicit type parameters: `<T: Clone + Ord, F: Fn(...) + Copy>`
- Iterator-based `.partition()` instead of pattern matching
- Mutable references in `.append(&mut result)`
- Explicit `|x| gt(&pivot, x)` closure syntax

---

### Rust Version (Idiomatic)

```rust
pub fn quicksort_idiomatic<T: Ord>(xs: Vec<T>) -> Vec<T> {
    let mut result = xs;
    result.sort();
    result
}

// Usage
let sorted = quicksort_idiomatic(vec![4, 65, 2, -31, 0, 99, 83, 782, 1]);
// Result: vec![-31, 0, 1, 2, 4, 65, 83, 99, 782]
```

**Line count:** 5 lines (matches OCaml in brevity, but uses stdlib)  
**Syntax highlights:**
- Single trait bound: `T: Ord`
- Delegates to proven stdlib implementation
- No recursive structure visible

---

## Type Signature Comparison

| Aspect | OCaml | Rust (Functional) | Rust (Idiomatic) |
|--------|-------|------------------|------------------|
| **Function name** | `quicksort` | `quicksort` | `quicksort_idiomatic` |
| **Type params** | `'a` (implicit) | `T: Clone + Ord, F: Fn(...) + Copy` | `T: Ord` |
| **Comparator** | `gt` (first param) | `F: Fn(&T, &T) -> bool + Copy` | Not needed (uses `Ord`) |
| **Input** | `'a list` | `Vec<T>` | `Vec<T>` |
| **Output** | `'a list` | `Vec<T>` | `Vec<T>` |
| **Complexity** | O(n²) worst, O(n log n) avg | O(n²) worst, O(n log n) avg | O(n log n) worst (introsort) |
| **Memory model** | Immutable, GC-managed | Owned, Clone-able elements | Owned, `Ord` elements |
| **Generic** | Polymorphic over any `'a` | Requires `Clone` trait | Requires `Ord` trait only |

---

## Execution Flow Comparison

### OCaml Execution (Immutable)

```
quicksort (>) [4, 65, 2, -31, 0, 99, 83, 782, 1]
├─ pivot = 4
├─ ys = [...]  (elements > 4, from List.partition)
├─ zs = [...]  (elements ≤ 4)
├─ left = quicksort (>) ys
├─ right = quicksort (>) zs
└─ left @ [4] @ right = [-31, 0, 1, 2, 4, 65, 83, 99, 782]
```

**Memory behavior:**
- New list created at each `@` operation
- Garbage collector reclaims old list nodes
- Immutability ensures no aliasing issues

---

### Rust Execution (Functional Style)

```
quicksort(|a, b| a > b, vec![4, 65, 2, -31, 0, 99, 83, 782, 1])
├─ pivot = xs.remove(0)  // moves 4 out
├─ xs.into_iter().partition(...)  // creates (ys, zs)
├─ left = quicksort(..., ys)  // recursive call
├─ result = quicksort(..., zs)  // recursive call
├─ left.push(pivot)  // mutate in-place
└─ left.append(&mut result)  // mutate in-place
```

**Memory behavior:**
- `.remove(0)` shifts all elements (O(n) operation)
- `.partition()` iterates once, creates two new vecs
- `.push()` and `.append()` mutate existing vecs
- No garbage collection; RAII cleanup on scope exit

---

### Rust Execution (Idiomatic)

```
quicksort_idiomatic(vec![4, 65, 2, -31, 0, 99, 83, 782, 1])
└─ result.sort()  // introsort algorithm
   ├─ Phase 1: quicksort (depth = log n)
   ├─ Phase 2: heapsort (if too deep)
   ├─ Phase 3: insertion sort (small arrays < 16)
   └─ In-place mutation of result
```

**Memory behavior:**
- Single allocation; no recursive copies
- All work done in-place
- Automatic switch to heapsort prevents O(n²) worst-case

---

## 5 Key Insights

### 1. **OCaml's Elegance vs. Rust's Explicitness**

**OCaml:**
```ocaml
let rec quicksort gt = function
  | [] -> []
  | x::xs -> let ys, zs = List.partition (gt x) xs in
             (quicksort gt ys) @ (x :: (quicksort gt zs))
```

**Rust:**
```rust
pub fn quicksort<T: Clone + Ord, F: Fn(&T, &T) -> bool + Copy>(gt: F, mut xs: Vec<T>) -> Vec<T> {
    if xs.is_empty() { return xs; }
    let pivot = xs.remove(0);
    let (ys, zs): (Vec<T>, Vec<T>) = xs.into_iter().partition(|x| gt(&pivot, x));
    let mut left = quicksort(gt, ys);
    let mut result = quicksort(gt, zs);
    left.push(pivot);
    left.append(&mut result);
    left
}
```

**Key difference:** OCaml achieves 5 lines with pattern matching and implicit polymorphism. Rust requires 18 lines because ownership, trait bounds, and explicit type parameters must be declared. However, Rust's explicitness catches more errors at compile-time.

**Insight:** *Elegance doesn't scale. Rust trades brevity for correctness.*

---

### 2. **The Comparator Pattern: Functions as Data**

| Language | Pattern | Flexibility |
|----------|---------|-------------|
| OCaml | `let quicksort gt = ...` where `gt` is a function | High: supports any binary predicate |
| Rust (Functional) | `<F: Fn(...) + Copy>` generic parameter | High but constrained: must implement `Copy` |
| Rust (Idiomatic) | Uses `Ord` trait | Medium: only ascending order by default, but `sort_by()` available |

**Insight:** *Rust's trait system enforces semantics. The `Copy` requirement on `F` isn't a limitation—it's a guarantee that the comparator is thread-safe and copyable.*

---

### 3. **Allocation Patterns: Stack vs. Heap**

**OCaml:**
- Creates intermediate list structures at each recursion level
- Garbage collector manages cleanup
- Multiple pointer indirections (linked list)

**Rust (Functional):**
- Creates two new `Vec`s at each recursion level (allocations on heap)
- Automatic RAII cleanup on scope exit
- Vectors are contiguous (better cache locality than linked lists)

**Rust (Idiomatic):**
- Single allocation; all work in-place
- No intermediate vectors
- Minimal memory overhead

**Insight:** *Even Rust's "functional" implementation is more imperative than OCaml due to vector allocations. The idiomatic version eliminates this overhead entirely.*

---

### 4. **Composability: Chaining vs. Pipeline**

**OCaml:** Easy composition with other list operations
```ocaml
quicksort (>) input
|> List.filter (fun x -> x > 0)
|> List.map (fun x -> x * 2)
```

**Rust (Functional):** Possible but requires iterator adapters
```rust
quicksort(|a, b| a > b, input)
    .into_iter()
    .filter(|x| x > &0)
    .map(|x| x * 2)
    .collect()
```

**Rust (Idiomatic):** Can be chained before sorting
```rust
let sorted: Vec<_> = input
    .into_iter()
    .filter(|x| x > &0)
    .map(|x| x * 2)
    .collect::<Vec<_>>();
sorted.sort();
```

**Insight:** *Rust's iterator paradigm is different but equally composable once you understand it. Immediate actions (like sorting) can't be lazily chained like in OCaml.*

---

### 5. **When to Break the Pattern: Pragmatism Over Purity**

**The Problem:**
- OCaml's `@` (list concatenation) is O(n) on the left list
- Rust's `.remove(0)` (removing head) is O(n) on the vector
- Both are inefficient for quicksort!

**The Solution (Rust only):**
- **Don't use functional quicksort in production**
- Use `std::sort` (introsort) which is O(n log n) guaranteed

**OCaml's Option:**
- Could use a mutable array-based quicksort (less idiomatic)
- Or accept the functional overhead for algorithmic clarity

**Rust's Advantage:**
```rust
// One line. Battle-tested. O(n log n) worst-case. Done.
vec.sort();
```

**Insight:** *Rust's ecosystem provides optimized primitives. Use them. The functional style is educational but idiomatic Rust chooses performance through library design, not through clever algorithms.*

---

## Summary Table

| Property | OCaml | Rust (Functional) | Rust (Idiomatic) |
|----------|-------|------------------|------------------|
| Lines of code | 5 | 18 | 5 |
| Type safety | Compile-time (ML type system) | Compile-time (Rust type system) | Compile-time + runtime traits |
| Memory model | GC, immutable | Owned, mutable | Owned, mutable |
| Worst-case perf | O(n²) | O(n²) | O(n log n) |
| Typical use | Algorithm teaching | Algorithm teaching | **Production code** |
| Composability | Excellent (pipeline) | Good (iterators) | Excellent (iterators) |
| Learning curve | Medium | High (ownership + traits) | Medium (stdlib knowledge) |
| Recommended for | Learning, teaching | Porting OCaml | **All Rust code** |

---

## Lessons for Rust Developers

1. **Know your standard library.** `std::sort` is better than any manual implementation.
2. **Trait bounds enable generic programming.** `F: Fn(...) + Copy` is powerful but requires understanding.
3. **Ownership isn't a limitation; it's a feature.** Rust prevents entire classes of bugs through ownership semantics.
4. **Mutability is intentional.** The `mut` keyword signals where mutation happens, making code review easier.
5. **Performance and safety are not trade-offs in Rust.** They're unified through the type system.
