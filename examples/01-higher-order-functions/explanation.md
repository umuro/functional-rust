# Higher-Order Functions: OCaml → Rust

## What are Higher-Order Functions?

Functions that take other functions as arguments or return functions as results. The foundation of functional programming.

The three most important HOFs:
- **map**: Transform each element
- **filter**: Select elements matching a condition
- **fold** (reduce): Accumulate elements into a single value

---

## OCaml Implementation

```ocaml
(* Map: Apply function to each element *)
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs

(* Filter: Keep elements matching predicate *)
let rec filter pred = function
  | [] -> []
  | x :: xs ->
      if pred x then x :: filter pred xs
      else filter pred xs

(* Fold: Accumulate with a function *)
let rec fold_left f acc = function
  | [] -> acc
  | x :: xs -> fold_left f (f acc x) xs
```

**Key OCaml features:**
- Pattern matching on list structure (`[]` vs `x :: xs`)
- Implicit currying (functions automatically partially applicable)
- Tail recursion for `fold_left` (efficient)
- Clean syntax with `function` keyword

---

## Rust Translation

```rust
fn map<T, U, F>(f: F, list: &[T]) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let mut result = vec![f(x)];
            result.extend(map(f, xs));
            result
        }
    }
}

fn filter<T, F>(pred: F, list: &[T]) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let mut result = if pred(x) { vec![x.clone()] } else { vec![] };
            result.extend(filter(pred, xs));
            result
        }
    }
}

fn fold_left<T, U, F>(f: F, acc: U, list: &[T]) -> U
where
    F: Fn(U, &T) -> U + Copy,
{
    match list {
        [] => acc,
        [x, xs @ ..] => fold_left(f, f(acc, x), xs),
    }
}
```

**Key Rust adaptations:**
- Explicit generic parameters (`<T, U, F>`)
- Trait bounds (`F: Fn(&T) -> U`)
- Ownership considerations (borrowing `&[T]`, cloning when needed)
- Slice pattern matching (`[x, xs @ ..]`)
- `vec![]` for dynamic vectors

---

## Side-by-Side Comparison

### Map

| OCaml | Rust |
|-------|------|
| `let rec map f = function` | `fn map<T, U, F>(f: F, list: &[T]) -> Vec<U>` |
| `\| [] -> []` | `[] => vec![],` |
| `\| x :: xs -> f x :: map f xs` | `[x, xs @ ..] => { ... }` |
| Implicit generics | Explicit `<T, U, F>` with trait bounds |
| List cons (`::`) | Vector building + `extend` |

### Filter

| OCaml | Rust |
|-------|------|
| `if pred x then x :: ...` | `if pred(x) { vec![x.clone()] } else { vec![] }` |
| No ownership concerns | Explicit `Clone` trait bound |
| Pattern matching | Same pattern matching style |

### Fold

| OCaml | Rust |
|-------|------|
| Tail-recursive by default | Tail-recursive (Rust optimizes) |
| `fold_left f (f acc x) xs` | `fold_left(f, f(acc, x), xs)` |
| Implicit currying | Explicit arguments |

---

## Key Differences

### 1. Type System
- **OCaml**: Hindley-Milner type inference (you rarely write types)
- **Rust**: Type inference within function bodies, but signatures need annotations

### 2. Ownership
- **OCaml**: Garbage collected, no ownership concerns
- **Rust**: Ownership model requires explicit borrowing (`&[T]`) and sometimes cloning

### 3. Recursion
- **OCaml**: Tail-call optimization guaranteed
- **Rust**: LLVM usually optimizes tail calls, but not guaranteed

### 4. Currying
- **OCaml**: Functions are curried by default (`map f` returns a function)
- **Rust**: No automatic currying (use closures or explicit partial application)

### 5. Pattern Matching
- **OCaml**: List cons pattern (`x :: xs`)
- **Rust**: Slice patterns (`[x, xs @ ..]` requires nightly or stable with recent versions)

---

## Production vs. Learning

**Note:** These implementations are for learning! In production Rust:

```rust
// Use iterators (zero-cost abstractions)
let doubled: Vec<_> = [1, 2, 3, 4, 5]
    .iter()
    .map(|x| x * 2)
    .collect();

let evens: Vec<_> = [1, 2, 3, 4, 5, 6]
    .iter()
    .filter(|x| *x % 2 == 0)
    .collect();

let sum: i32 = [1, 2, 3, 4, 5]
    .iter()
    .fold(0, |acc, x| acc + x);
```

**Why iterators are better:**
- Zero-cost abstraction (compiled to loops)
- No intermediate allocations
- Composable without cloning
- Built-in optimizations

**Why we implement from scratch:**
- Understand the FP pattern
- See how recursion maps to Rust
- Learn ownership implications
- Appreciate iterator design

---

## Composition Example

### OCaml
```ocaml
let result =
  [1; 2; 3; 4; 5]
  |> map (fun x -> x * 2)
  |> filter (fun x -> x mod 2 = 0)
  |> fold_left (fun acc x -> acc + x) 0
(* Result: 30 *)
```

### Rust (idiomatic)
```rust
let result = [1, 2, 3, 4, 5]
    .iter()
    .map(|x| x * 2)
    .filter(|x| x % 2 == 0)
    .sum::<i32>();
// Result: 30
```

**Both achieve the same goal:**
- Transform data (double)
- Filter results (evens only)
- Reduce to single value (sum)

**Rust's iterator chain is equivalent to OCaml's pipe operator (`|>`).**

---

## Why This Matters

### 1. Foundation of FP
Map/filter/fold are the building blocks. Master these, and you understand:
- Transformations (map)
- Selections (filter)
- Aggregations (fold)

### 2. Composability
Chain operations without intermediate variables:
```rust
data.iter()
    .map(transform)
    .filter(predicate)
    .fold(initial, combiner)
```

### 3. Reusability
Write generic functions once, use them everywhere:
```rust
fn process<T>(data: &[T]) -> Vec<i32>
where
    T: Clone + Into<i32>,
{
    data.iter()
        .map(|x| x.clone().into())
        .filter(|x| *x > 0)
        .collect()
}
```

### 4. Immutability
Transformations return new data, original unchanged:
```rust
let original = vec![1, 2, 3];
let doubled = original.iter().map(|x| x * 2).collect::<Vec<_>>();
// original still [1, 2, 3]
```

---

## Further Reading

### OCaml
- [Real World OCaml - Higher-Order Functions](https://dev.realworldocaml.org/lists-and-patterns.html#higher-order-functions)
- [OCaml Manual - List Module](https://v2.ocaml.org/api/List.html)

### Rust
- [The Rust Book - Functional Language Features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Rust Iterator Trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Rust by Example - Higher Order Functions](https://doc.rust-lang.org/rust-by-example/fn/hof.html)

### Theory
- [Category Theory for Programmers](https://github.com/hmemcpy/milewski-ctfp-pdf) (Chapter 7: Functors)

---

## Next Example

**Coming next:** Pattern Matching - How OCaml's exhaustive matching maps to Rust's `match` expressions.

**Topics:** Variants, destructuring, wildcard patterns, guards, nested patterns.

---

*Part of the Functional Rust series - translating OCaml FP concepts to idiomatic Rust.*
