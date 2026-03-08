# OCaml ↔ Rust: List Map Comparison

## Side-by-Side Code

### Basic Implementation

#### OCaml: List.map (Idiomatic)
```ocaml
let numbers = [1; 2; 3; 4; 5]
let doubled = List.map (fun x -> x * 2) numbers
let () = List.iter (fun x -> Printf.printf "%d " x) doubled
(* Output: 2 4 6 8 10 *)
```

#### Rust: Iterator (Idiomatic)
```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled = map_iter(&numbers, |x| x * 2);
println!("{:?}", doubled);
// Output: [2, 4, 6, 8, 10]
```

**Observations:**
- OCaml syntax is more compact (fewer brackets and type hints)
- Rust requires explicit `&` borrowing to avoid consuming `numbers`
- Both are one-liners for the mapping operation

---

### Recursive Implementation

#### OCaml: Naive Recursion
```ocaml
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs
```

**Pros:**
- Very concise (3 lines)
- Directly follows list structure
- Pattern matching is natural

**Cons:**
- Not tail-recursive (can stack overflow on large lists)
- Creates intermediate list nodes

---

#### OCaml: Tail-Recursive (Optimized)
```ocaml
let map_tail f xs =
  let rec go acc = function
    | [] -> List.rev acc
    | x :: xs -> go (f x :: acc) xs
  in
  go [] xs
```

**Pros:**
- Tail-call optimized (O(1) stack space)
- Linear time complexity
- No risk of stack overflow

**Cons:**
- More complex code
- Requires `List.rev` at the end

---

#### Rust: Recursive (Functional Style)
```rust
pub fn map_recursive<T, U, F>(xs: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    fn go<T, U, F>(mut xs: Vec<T>, f: &F, mut acc: Vec<U>) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        if xs.is_empty() {
            acc
        } else {
            let head = xs.remove(0);
            acc.push(f(head));
            go(xs, f, acc)
        }
    }

    go(xs, &f, Vec::new())
}
```

**Pros:**
- Demonstrates functional style in Rust
- Conceptually similar to OCaml

**Cons:**
- Not idiomatic Rust
- `xs.remove(0)` is O(n) per element → O(n²) total
- No tail-call optimization (Rust doesn't guarantee it)
- More verbose type annotations

---

#### Rust: Idiomatic (Iterator)
```rust
pub fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    xs.iter().map(f).collect()
}
```

**Pros:**
- Concise and idiomatic
- Zero-cost abstraction
- Lazy evaluation
- Composable with other iterator methods
- Compiler can optimize aggressively

**Cons:**
- Less obviously "functional" to beginners
- Closure captures by reference

---

## Type Signatures Comparison

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Signature** | `('a -> 'b) -> 'a list -> 'b list` | `<T, U, F>(xs: &[T], f: F) -> Vec<U> where F: Fn(&T) -> U` |
| **Type Variables** | `'a`, `'b` (implicit) | `T`, `U`, `F` (explicit) |
| **Function Type** | `'a -> 'b` | `Fn(&T) -> U` (trait) |
| **Collection Type** | `list` (persistent) | `Vec<T>` (mutable heap) / `&[T]` (slice) |
| **Closure** | `fun x -> ...` | `\|x\| ...` |
| **Reference** | Implicit | Explicit (`&T`) |
| **Lifetime** | None | `'_` (implicit) |

**OCaml Type Example:**
```ocaml
List.map : ('a -> 'b) -> 'a list -> 'b list
```
- Universally quantified polymorphism
- Implicit reference handling
- Function is a first-class value type

**Rust Type Example:**
```rust
fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
where F: Fn(&T) -> U
```
- Generic with trait bounds
- Explicit borrowing (`&[T]`)
- Function must satisfy `Fn` trait

---

## Implementation Characteristics

### Memory Allocation

#### OCaml
```ocaml
(* Creates intermediate list *)
let doubled = List.map (fun x -> x * 2) numbers
```
- **Cost:** O(n) heap allocations (one per list node)
- **Space:** O(n) for result + O(n) for stack frames
- **Cleanup:** Automatic garbage collection

#### Rust (Iterator)
```rust
let doubled = map_iter(&numbers, |x| x * 2);
```
- **Cost:** One allocation for final `Vec::collect()`
- **Space:** O(n) for result
- **Cleanup:** RAII (automatic on drop)

#### Rust (Recursive)
```rust
let doubled = map_recursive(numbers, |x| x * 2);
```
- **Cost:** One allocation (the accumulator Vec) + n remove calls
- **Space:** O(n) for result + O(n) for recursion stack
- **Cleanup:** RAII (automatic on drop)

---

## Five Key Insights

### 1. **Iterators > Recursion in Rust**

Rust's iterator abstraction is fundamentally different from OCaml's pattern matching. While OCaml naturally expresses recursion through pattern matching on list structure, Rust's iterators are:
- More efficient (no intermediate vectors)
- More composable (`.filter().map().fold()`)
- More idiomatic (what Rust developers expect)
- Better optimized (compiler can vectorize)

**Lesson:** Don't write OCaml-style recursive functions in Rust. Use iterators.

### 2. **Ownership Prevents Hidden Costs**

```ocaml
(* This creates TWO lists silently *)
let doubled = List.map (fun x -> x * 2) numbers
let tripled = List.map (fun x -> x * 3) doubled
```

```rust
// This creates only ONE list (chained iterators)
let result: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .map(|x| x * 3)
    .collect();  // Only one allocation here!
```

Rust's ownership model makes allocation explicit. You can't accidentally create intermediate structures.

### 3. **Reference Semantics Matter**

OCaml's implicit reference handling means:
- `List.map` always returns a new list
- Original list is never modified
- References are transparent

Rust's explicit borrowing requires thought:
- `map_iter(&xs, f)` borrows but doesn't consume
- `map_recursive(xs, f)` consumes ownership
- Reference type affects performance

**Lesson:** Borrows are cheaper than moves for map operations.

### 4. **Lazy vs. Eager Evaluation**

| OCaml | Rust |
|-------|------|
| `List.map` eagerly evaluates | `iter().map()` lazily evaluates |
| All transformations happen immediately | Transformations happen on `.collect()` |
| Can't express infinite sequences | Can express infinite lazily-evaluated sequences |
| Simpler mental model | More efficient for chains |

```ocaml
(* Both transformations executed immediately *)
List.map (fun x -> x * 2) 
  (List.map (fun x -> x + 1) numbers)
```

```rust
// Only executed when `.collect()` is called
numbers.iter()
    .map(|x| x + 1)
    .map(|x| x * 2)
    .collect::<Vec<_>>()
```

### 5. **Closure Capture Differences**

OCaml closures automatically capture variables:
```ocaml
let multiplier = 3
let triple = List.map (fun x -> x * multiplier) numbers
(* multiplier is captured *)
```

Rust closures can capture by reference or move:
```rust
let multiplier = 3;
let result = map_iter(&numbers, |x| x * multiplier);  // Captures &multiplier
```

This affects:
- Memory lifetime
- Mutable vs. immutable captures
- Thread safety

---

## Performance Comparison

### Allocation Count (for 5-element list)

| Approach | OCaml | Rust |
|----------|-------|------|
| `List.map` | 5 node allocations | 1 Vec allocation |
| Single `.map()` | 5 allocations | 1 allocation |
| Two chained `.map()` | 10 allocations | 1 allocation |
| Recursive naive | 5 allocations | N/A (not recommended) |
| Recursive tail | 5 allocations | 1 allocation (but with removes) |

**Winner:** Rust iterators by far (O(1) allocations for chains)

### Time Complexity (for n-element list)

| Approach | OCaml | Rust |
|----------|-------|------|
| `List.map` | O(n) | O(n) |
| Single `.map()` | O(n) | O(n) |
| Recursive naive | O(n) stack frames | Stack overflow risk |
| Recursive tail | O(n) optimized | O(n²) with remove |
| Chained iterators | O(n) × chain length | O(n) (fused) |

**Winner:** Rust iterators (fused operations), OCaml tail-recursive as backup

---

## Test Coverage

### OCaml Tests (11 assertions)
- Empty list
- Single element
- Multiple elements
- Type conversion (int → string)
- Negative numbers (abs)
- Tail-recursive equivalence
- Chained transformations

### Rust Tests (15 assertions)
- Empty vector
- Single element
- Multiple elements
- Type conversion (i32 → String)
- Negative numbers
- Squaring (higher-order operations)
- Closure captures
- All three implementations tested

---

## Conversion Checklist: OCaml → Rust

When converting OCaml `List.map` patterns to Rust:

- [ ] Replace `List.map f xs` with `xs.iter().map(f).collect()`
- [ ] Change `[a; b; c]` to `vec![a, b, c]` or `&[a, b, c]`
- [ ] Replace pattern matching with iterator methods
- [ ] Add type annotations (Rust may not infer them)
- [ ] Consider lifetime implications for `&T` vs. owned `T`
- [ ] Use `.filter()`, `.map()`, `.fold()` for complex transformations
- [ ] Avoid recursion; use iterators instead
- [ ] Test with `.collect::<Vec<_>>()` to force evaluation

---

## Summary

| Dimension | OCaml | Rust |
|-----------|-------|------|
| **Syntax** | Concise, inferred | Explicit, verbose |
| **Performance** | Eager, allocates intermediates | Lazy, fused operations |
| **Idiomatic** | Recursion + pattern matching | Iterators + trait bounds |
| **Safety** | Garbage collected | Ownership + borrowing |
| **Type checking** | Implicit polymorphism | Explicit generics |
| **Parallel** | Requires threads/async | Built-in (fearless concurrency) |

Both are excellent functional languages. OCaml excels at pattern matching and quick prototyping. Rust excels at performance, memory safety, and large-scale systems. Choose the tool for the problem.
