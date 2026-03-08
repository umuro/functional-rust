# Detailed Comparison: OCaml vs Rust List Filtering

This document provides a side-by-side analysis of list filtering in OCaml and Rust, including code, type signatures, and key insights.

## Side-by-Side Code Comparison

### The Simplest Case: Built-in Filter

**OCaml (using `List.filter`):**
```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
```

**Rust (using `.filter().collect()`):**
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
let evens: Vec<_> = numbers.iter().filter(|x| x % 2 == 0).cloned().collect();
```

**Observations:**
- OCaml: One line, built-in function
- Rust: More explicit (iterator chain), but lazy until `.collect()`
- OCaml works with immutable linked lists; Rust works with contiguous arrays

---

### Recursive Implementation

**OCaml:**
```ocaml
let rec filter_recursive pred lst =
  match lst with
  | [] -> []
  | head :: tail ->
      if pred head then
        head :: filter_recursive pred tail
      else
        filter_recursive pred tail
```

**Rust:**
```rust
pub fn filter_recursive<T: Clone>(items: &[T], pred: impl Fn(&T) -> bool + Copy) -> Vec<T> {
    match items {
        [] => Vec::new(),
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, pred);
            if pred(head) {
                let mut result = vec![head.clone()];
                result.append(&mut rest);
                result
            } else {
                rest
            }
        }
    }
}
```

**Key Differences:**
1. **Pattern matching syntax:**
   - OCaml: `[head :: tail]` for cons pattern
   - Rust: `[head, tail @ ..]` for slice pattern (more explicit about reference)

2. **List construction:**
   - OCaml: `head :: rest` (cheap, constant time cons)
   - Rust: Manual `vec![head.clone()]` then `.append(&mut rest)` (involves allocation)

3. **Generic bounds:**
   - OCaml: Type inference handles `'a`; closure type implicit
   - Rust: Explicit `<T: Clone>`, `impl Fn(&T) -> bool + Copy`

---

## Type Signatures Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Filter function** | `('a -> bool) -> 'a list -> 'a list` | `<T: Clone>(pred: impl Fn(&T) -> bool) -> Vec<T>` |
| **Closure type** | `'a -> bool` (implicit) | `Fn(&T) -> bool` (trait bound) |
| **Input list** | `'a list` (linked) | `&[T]` (slice/array) |
| **Output** | `'a list` (linked) | `Vec<T>` (owned heap array) |
| **Predicate application** | `pred x` (value) | `pred(x)` (reference) |
| **Cons operation** | `h :: t` (O(1)) | `vec![h].append(&mut t)` (O(n)) |
| **Type inference** | Full polymorphism `'a` | Generic bounds `<T>` |

---

## Execution Model Differences

### OCaml: Evaluation Strategy
```
List.filter pred [1; 2; 3]
→ matches 2 with pred?
→ if yes: 2 :: List.filter pred [3]
→ constructs linked-list node at each step
→ returns full list (eager)
```

**Characteristics:**
- Eager evaluation (entire list constructed before return)
- Cheap cons (`::`) at each step
- Garbage collector cleans up intermediate nodes
- Natural recursion (list structure drives control flow)

### Rust: Iterator Chain
```rust
numbers.iter()          // Iterator<&T>
  .filter(|x| ...)      // FilterIter wrapper
  .cloned()             // ClonedIter wrapper
  .collect()            // consumes iterator, allocates Vec
```

**Characteristics:**
- Lazy evaluation (iterator wrappers created, not evaluated until `.collect()`)
- Zero intermediate allocations during chaining
- Borrowing prevents mutation during iteration
- Composable (`.filter().map().take()` = one pass)

---

## Closure Semantics

### OCaml
```ocaml
let x = 5
let pred = fun y -> y > x  (* x captured by closure *)
List.filter pred [1; 2; 3; 4; 5; 6]  (* [6] *)
```

**Closure rules:**
- Captures variables from enclosing scope (immutable by default)
- No explicit capture syntax; implicit via free variables

### Rust
```rust
let x = 5;
let pred = |y: &i32| y > &x;  // x captured by shared borrow
vec![1, 2, 3, 4, 5, 6]
    .iter()
    .filter(pred)
    .cloned()
    .collect::<Vec<_>>()  // [6]
```

**Closure rules:**
- Captures by reference (shared borrow by default)
- Move semantics available with `move` keyword
- Borrow checker ensures closure doesn't outlive captured variables

---

## Performance Characteristics

| Operation | OCaml | Rust |
|-----------|-------|------|
| **Filter 1000 ints** | ~0.1ms (GC overhead) | ~0.01ms (zero-copy) |
| **Chain filter+map** | 2 passes (eager) | 1 pass (lazy) |
| **Memory** | Linked list pointers (8 bytes per node) | Contiguous Vec (8 bytes per int, dense) |
| **Cache locality** | Poor (pointer chasing) | Excellent (contiguous) |
| **Cons operation** | O(1) allocation | O(1)–O(n) depending on strategy |

**Winner:** Rust for large datasets due to cache locality and lazy iterator composition.

---

## 5 Key Insights

### 1. **Lists vs Vectors: Data Structure Dictates Idiom**

OCaml's singly-linked list makes recursion feel natural:
```ocaml
[1; 2; 3] ≡ 1 :: 2 :: 3 :: []
```

Rust's Vec makes iterators natural:
```rust
vec![1, 2, 3] ≡ [1, 2, 3] in contiguous memory
```

**Lesson:** The data structure shapes how you think about problems. Rust's iterators are an API layer over contiguous arrays; OCaml's recursion is the structural reality of linked lists.

### 2. **Lazy vs Eager: Trade Compile Efficiency for Runtime Efficiency**

OCaml's `List.filter` is eager—it builds the entire result immediately.

Rust's `.filter()` is lazy—it's a zero-cost wrapper that defers work until `.collect()`.

```rust
let iter = numbers.iter().filter(|x| x % 2 == 0);  // No filtering yet
let result = iter.collect::<Vec<_>>();  // Now it filters
```

**Lesson:** Laziness allows composition without intermediate allocations. Rust's type system makes this free.

### 3. **Ownership Eliminates One Dimension of Confusion**

In OCaml, garbage collection handles cleanup implicitly. You never ask, "Who owns this list?"

In Rust, ownership is explicit:
```rust
let numbers = vec![1, 2, 3];  // I own this
let evens = filter_iter(&numbers, |x| x % 2 == 0);  // I borrow it
println!("{:?}", numbers);  // Still usable (borrow is over)
```

**Lesson:** Explicit ownership prevents use-after-free bugs at compile time. It feels verbose initially but becomes a powerful invariant.

### 4. **Monomorphism vs Polymorphism: Specialization vs Generality**

OCaml is fully polymorphic. A function like `List.filter` works on any type:
```ocaml
List.filter (fun x -> x > 0) [1; 2; 3]         (* ints *)
List.filter (fun x -> x > 0.0) [1.0; 2.0]      (* floats *)
```

Rust uses monomorphization: each instantiation of a generic function gets a specialized version:
```rust
filter_iter(&[1, 2, 3], |x| x > &0)      // Generates code for Vec<i32>
filter_iter(&[1.0, 2.0], |x| x > &0.0)  // Generates code for Vec<f64>
```

**Lesson:** Monomorphization means no runtime dispatch overhead (the generic is as fast as hand-written code). The trade-off is compile time and binary size.

### 5. **Borrow Checker Prevents Entire Classes of Bugs**

OCaml doesn't prevent this:
```ocaml
let lst = [1; 2; 3]
let iter = List.iter (fun x -> ...) lst
(* Somewhere else, if you mutate lst, the iter may break *)
```

Rust prevents it:
```rust
let mut numbers = vec![1, 2, 3];
let evens: Vec<_> = numbers.iter().filter(|x| x % 2 == 0).collect();
// numbers.push(4);  // ERROR: can't mutate while borrowed
```

**Lesson:** The borrow checker is a feature, not a limitation. It catches data race bugs, use-after-free, and iterator invalidation at compile time.

---

## Summary Table: When to Use Each

| Use Case | OCaml | Rust | Why |
|----------|-------|------|-----|
| **List processing (small)** | ✅ Natural recursion | ⚠️ Iterators verbose | Linked structure natural for OCaml |
| **Performance-critical filtering** | ❌ GC overhead | ✅ Zero-copy iterators | Cache locality + laziness |
| **Polymorphic code** | ✅ Implicit `'a` | ⚠️ Explicit generics | OCaml's inference simpler |
| **Preventing mutations** | ❌ Immutable by default, but no guarantee | ✅ Borrow checker enforces | Rust's compile-time guarantee |
| **Teaching recursion** | ✅ Clear, idiomatic | ⚠️ Slice patterns less obvious | List structure matches algorithm |

---

**Conclusion:** OCaml and Rust solve the same problem with different idioms rooted in their data structures and memory models. Understanding *why* each language chooses its idiom teaches you more than the idiom itself.
