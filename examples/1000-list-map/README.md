# Example 1000: List Map

**Topic:** Apply a function to each element of a list  
**Category:** stdlib-list  
**Difficulty:** Beginner  

## Overview

This example demonstrates how to apply a function to every element of a collection using both idiomatic Rust patterns and functional-style recursion. We compare it with OCaml's approach to highlight key language differences.

## OCaml Approach

OCaml provides `List.map` as a standard library function that applies a function to each element and returns a new list:

```ocaml
let numbers = [1; 2; 3; 4; 5]
let doubled = List.map (fun x -> x * 2) numbers
let () = List.iter (fun x -> Printf.printf "%d " x) doubled
(* Output: 2 4 6 8 10 *)
```

The OCaml approach uses:
- **Immutable lists**: Lists are persistent data structures
- **First-class functions**: Anonymous functions (`fun x -> ...`) are treated as values
- **Pattern matching**: Functions can be defined recursively using pattern matching on lists

```ocaml
(* Naive recursive implementation *)
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs

(* Tail-recursive (more efficient) *)
let map_tail f xs =
  let rec go acc = function
    | [] -> List.rev acc
    | x :: xs -> go (f x :: acc) xs
  in
  go [] xs
```

## Rust Approach

Rust offers two main approaches:

### 1. **Idiomatic Rust: Iterators** (Preferred)

```rust
pub fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    xs.iter().map(f).collect()
}

// Usage
let numbers = vec![1, 2, 3, 4, 5];
let doubled = map_iter(&numbers, |x| x * 2);
// doubled = vec![2, 4, 6, 8, 10]
```

**Why iterators are idiomatic:**
- **Zero-cost abstraction**: Compiles to the same assembly as manual loops
- **Composable**: Chain multiple operations (map, filter, fold)
- **Lazy evaluation**: Operations aren't executed until consumed
- **Optimized by the compiler**: Can be auto-vectorized

### 2. **Functional Rust: Tail Recursion**

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

This mirrors OCaml's approach more closely but is **not idiomatic Rust** because:
- Iterators are more efficient
- Recursive patterns are less common in Rust
- No tail-call optimization guarantee (unlike OCaml)

## Four Key Differences

### 1. **Memory Model: Ownership vs. Garbage Collection**

| OCaml | Rust |
|-------|------|
| Garbage collected | Ownership-based memory management |
| Lists are automatically cleaned up | Caller must explicitly manage ownership |
| No lifetime concerns | Lifetimes must be explicitly tracked |

```ocaml
(* OCaml: automatic cleanup *)
let process_list xs = List.map (fun x -> x * 2) xs
(* xs is automatically cleaned when no longer referenced *)
```

```rust
// Rust: explicit ownership
pub fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
// ^- Must borrow to avoid consuming input
```

### 2. **Iteration Model: Pattern Matching vs. Iterators**

| OCaml | Rust |
|-------|------|
| Pattern match on list structure (`[] \| x :: xs`) | Iterator trait with `.iter()`, `.map()` |
| Recursive naturally follows list structure | Lazy iterator chain composition |
| No lazy evaluation by default | Built-in lazy evaluation |

```ocaml
(* OCaml pattern matching *)
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs
```

```rust
// Rust iterator chain
xs.iter().map(f).collect()
```

### 3. **Type Safety: Inference vs. Explicit Generics**

| OCaml | Rust |
|-------|------|
| Strong type inference | Explicit generic parameters |
| Function type inferred from usage | Must specify `where F: Fn(T) -> U` |
| Simpler syntax | More verbose but prevents surprises |

```ocaml
(* Type inferred: val map : ('a -> 'b) -> 'a list -> 'b list *)
let map f xs = ...
```

```rust
// Type explicitly specified
pub fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
```

### 4. **Performance Model: Lazy vs. Eager (Both Optimized)**

| OCaml | Rust |
|-------|------|
| Eager evaluation (immediate) | Lazy iterators (deferred until `.collect()`) |
| Intermediate lists are created | No intermediate allocations if possible |
| Tail recursion can be optimized | Iterator chains can auto-vectorize |

```ocaml
(* Creates intermediate lists *)
let doubled = List.map (fun x -> x * 2) numbers  (* list allocated *)
let tripled = List.map (fun x -> x * 3) doubled (* list allocated *)
(* Two allocations total *)
```

```rust
// No intermediate allocations (when optimized)
let result: Vec<i32> = xs.iter()
    .map(|x| x * 2)
    .map(|x| x * 3)
    .collect();  // Only one allocation here
```

## Learning Outcomes

After completing this example, you should understand:

1. **Two approaches to mapping in Rust**: iterators (idiomatic) and recursion (functional)
2. **Why iterators are preferred**: zero-cost abstractions with composability
3. **How ownership affects design**: Rust's `&[T]` vs. OCaml's implicit list ownership
4. **Lazy vs. eager evaluation**: Rust's iterator chains vs. OCaml's eager evaluation
5. **Pattern matching vs. iterator traits**: Different paths to the same goal
6. **Generic function design**: How to write reusable map functions with trait bounds

## Files

- `src/lib.rs` - Library with idiomatic and recursive implementations, 15 tests
- `example.rs` - Standalone example with `main()` demonstrating all approaches
- `example.ml` - OCaml reference implementation with 11 test assertions
- `COMPARISON.md` - Detailed side-by-side code comparison

## Running

### Rust Tests
```bash
cargo test -p example-1000-list-map
```

### Rust Example
```bash
rustc example.rs -o example-1000 && ./example-1000
```

### OCaml (if ocaml is installed)
```bash
ocaml example.ml
```

## Related Concepts

- **List folds**: `reduce`, `fold_left`, `fold_right`
- **Filter**: Apply a predicate to select elements
- **Lazy evaluation**: `std::iter::Chain`, `std::iter::Map`
- **Trait bounds**: `Fn`, `FnMut`, `FnOnce`
- **Higher-order functions**: Functions that take/return functions
