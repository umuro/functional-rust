# Example 1002: List Fold Left

**Topic:** Reduce a list to a single value from left to right  
**Category:** stdlib-list  
**Difficulty:** Beginner

## Learning Outcomes

After studying this example, you will understand:

1. **Fold operations** - How to reduce a collection to a single value using accumulation
2. **Iterator adapters in Rust** - Using `fold()` with closures for elegant functional transformations
3. **Recursive patterns** - Implementing fold_left recursively in a tail-call-optimizable way
4. **Closures and generics** - Writing flexible functions that work with different types and operations
5. **Generic type parameters** - Understanding how Rust's type system handles polymorphic fold operations

## The OCaml Approach

OCaml's `List.fold_left` is a standard library function that processes lists from left to right:

```ocaml
let sum = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers
```

The pattern is:
- **Binary operation** - Function taking (accumulator, element) → accumulator
- **Initial value** - Starting accumulator (0 for sum, 1 for product, min_int for max)
- **List** - Collection to fold over

OCaml also allows custom recursive implementations that match this pattern internally.

## The Rust Approach

Rust provides two equally powerful approaches:

### 1. Iterator-Based (Idiomatic Rust)

```rust
pub fn fold_left_iter<T, U, F>(init: U, items: &[T], f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}
```

**Why this is idiomatic:**
- Uses iterator chains (`iter().fold()`)
- Type system is fully generic (works with any types)
- Compiles to efficient machine code
- Leverages Rust's ownership system elegantly

### 2. Recursive (Functional Style)

```rust
pub fn fold_left_recursive<T, U, F>(init: U, items: &[T], f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => init,
        [head, tail @ ..] => fold_left_recursive(f(init, head), tail, f),
    }
}
```

**Why this matters:**
- Direct translation of OCaml's recursive model
- Demonstrates pattern matching on slices
- Tail-recursive, so Rust may optimize it to a loop
- Shows functional programming patterns in Rust

## Four Key Differences: OCaml vs Rust

### 1. **Type Signatures**

**OCaml:**
```ocaml
List.fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a
```
- Implicit type parameters (uppercase letters)
- Curried function signature
- ML-style generics without explicit bounds

**Rust:**
```rust
fn fold_left_iter<T, U, F>(init: U, items: &[T], f: F) -> U
where F: Fn(U, &T) -> U
```
- Explicit type parameters with trait bounds
- Takes function by value (not curried)
- `where` clause for complex trait constraints

### 2. **Memory Management**

**OCaml:**
- Garbage collected
- No borrowing considerations
- Lists are immutable by design
- Pattern matching consumes/traverses naturally

**Rust:**
- Manual memory ownership
- Borrowing prevents aliasing bugs
- Slices allow zero-copy iteration
- Must explicitly reference elements with `&`

### 3. **Function Syntax**

**OCaml:**
```ocaml
List.fold_left ( + ) 0 numbers  (* Operator as function *)
List.fold_left max min_int numbers  (* Function reference *)
```

**Rust:**
```rust
fold_left_iter(0, &numbers, |acc, x| acc + x)  (* Closure syntax *)
fold_left_iter(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc })
```

### 4. **Convenience vs Explicitness**

**OCaml:**
- `List.fold_left ( + ) 0 xs` is concise
- Operator overloading built-in
- Less need for helper functions

**Rust:**
- Closures are explicit `|acc, x| ...`
- Each operation needs clear syntax
- But provides flexibility: closures can capture context
- Helper functions (`sum()`, `product()`) provide convenient APIs

## Usage

### Running the Example

```bash
# Build and run
cargo run --bin example --release

# Run all tests
cargo test -p example-1002-list-fold-left

# Run a specific test
cargo test -p example-1002-list-fold-left test_sum -- --nocapture

# Format code
cargo fmt -p example-1002-list-fold-left

# Check for warnings
cargo clippy -p example-1002-list-fold-left -- -D warnings
```

### Using in Your Own Code

```rust
use list_fold_left::{fold_left_iter, sum, product, max_value};

let numbers = vec![1, 2, 3, 4, 5];

// Using the convenience functions
let total = sum(&numbers);
let prod = product(&numbers);
let max_num = max_value(&numbers);

// Using fold_left directly
let result = fold_left_iter(0, &numbers, |acc, x| {
    println!("Current: {}, Accumulator: {}", x, acc);
    acc + x
});
```

## Tests Included

The library includes 16 comprehensive tests covering:

- **Empty lists** - Verify initial value is returned
- **Single elements** - Test with minimal data
- **Multiple elements** - Full accumulation flow
- **Negative numbers** - Edge cases with sign changes
- **String concatenation** - Generic example with non-numeric type
- **Both implementations** - Verify iterator and recursive produce same results

All tests are in `src/lib.rs` under the `#[cfg(test)]` module.

## Further Learning

To deepen your understanding:

1. **Compare performance** - Try benchmarking `fold_left_iter` vs `fold_left_recursive` with large lists
2. **Implement variants** - Try `fold_right` (right-associative folding)
3. **Explore related functions** - Study `map()`, `filter()`, and how they compose with `fold()`
4. **Type system** - Understand why the generic `<T, U, F>` bounds are necessary
5. **Closure captures** - Try capturing variables from the enclosing scope in your fold operations
