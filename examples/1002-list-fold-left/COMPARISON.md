# Comparison: OCaml vs Rust - Fold Left

## Side-by-Side Code Comparison

### The Original OCaml Example

```ocaml
(* Data and basic operations *)
let numbers = [1; 2; 3; 4; 5]
let sum = List.fold_left ( + ) 0 numbers
let product = List.fold_left ( * ) 1 numbers
let max_val = List.fold_left max min_int numbers

(* Output *)
let () = Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum product max_val
```

### The Rust Iterator Equivalent

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Iterator-based fold_left (idiomatic)
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    let max_val = numbers.iter().fold(i32::MIN, |acc, x| if x > &acc { *x } else { acc });
    
    println!("Sum: {}, Product: {}, Max: {}", sum, product, max_val);
}
```

### The Rust Recursive Equivalent

```rust
fn fold_left_recursive<T, U, F>(init: U, items: &[T], f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => init,
        [head, tail @ ..] => fold_left_recursive(f(init, head), tail, f),
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Recursive fold_left (functional style)
    let sum = fold_left_recursive(0, &numbers, |acc, x| acc + x);
    let product = fold_left_recursive(1, &numbers, |acc, x| acc * x);
    let max_val = fold_left_recursive(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc });
    
    println!("Sum: {}, Product: {}, Max: {}", sum, product, max_val);
}
```

## Type Signatures Table

| Operation | OCaml | Rust (Iterator) | Rust (Recursive) |
|-----------|-------|---|---|
| Sum | `int list -> int` | `fn(Vec<i32>) -> i32` | `fn(U, &[T], Fn(U, &T)->U) -> U` |
| Generic Fold | `('a -> 'b -> 'a) -> 'a -> 'b list -> 'a` | `fn<T,U,F>(U, &[T], F)->U where F:Fn(U,&T)->U` | Same as iterator |
| Max | `(int -> int -> int) -> int -> int list -> int` | `fn(i32::MIN, &[i32], \|acc, x\| ...) -> i32` | Same as iterator |

## Key Insights

### 1. **Currying vs Direct Application**

**OCaml embraces currying:**
```ocaml
List.fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a
```
Each arrow represents a curried parameter. You can partially apply:
```ocaml
let fold_add = List.fold_left ( + )
let sum = fold_add 0 numbers
```

**Rust uses direct function application:**
```rust
fn fold_left_iter<T, U, F>(init: U, items: &[T], f: F) -> U
```
All parameters at once. Closures capture context naturally:
```rust
let init = 0;
let result = fold_left_iter(init, &numbers, |acc, x| acc + x);
```

**Winner:** OCaml is more elegant for composition, but Rust's approach is more explicit and easier to read for newcomers.

### 2. **Memory Model Impact**

**OCaml:**
- Lists are immutable linked lists
- Each recursive call traverses one cons cell
- Garbage collection handles cleanup
- No lifetime annotations needed

**Rust:**
```rust
fn fold_left_recursive<T, U, F>(init: U, items: &[T], f: F) -> U {
    match items {
        [] => init,
        [head, tail @ ..] => fold_left_recursive(f(init, head), tail, f),
    }
}
```
- Slices provide zero-copy views into arrays
- Stack grows with recursion depth (no TCO guarantee!)
- Lifetimes ensure `F` and `T` don't outlive data
- No garbage collection needed

**Winner:** Rust is more efficient for large arrays thanks to slices. OCaml is safer from stack overflow due to list structure.

### 3. **Operator Overloading vs Explicit Syntax**

**OCaml:**
```ocaml
List.fold_left ( + ) 0 numbers   (* Treat + as a function *)
List.fold_left max min_int numbers  (* Use max builtin directly *)
```

**Rust:**
```rust
fold_left_iter(0, &numbers, |acc, x| acc + x)  (* Must write closure *)
fold_left_iter(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc })
```

**Winner:** OCaml is concise. Rust is explicit (which some find clearer, especially beginners).

### 4. **Generic Type Constraints**

**OCaml:**
```ocaml
let fold_left f init xs = List.fold_left f init xs
(* Type checker infers: 'a -> 'b -> 'a *)
(* Works with ANY types *)
```

**Rust:**
```rust
pub fn fold_left_iter<T, U, F>(init: U, items: &[T], f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}
```
- `T` = element type
- `U` = accumulator type (can differ from T)
- `F` = closure type with trait bound `Fn(U, &T) -> U`
- All constraints explicit in the signature

**Winner:** Rust wins on clarity—you see exactly what types are involved and what the function expects.

### 5. **Tail Call Optimization (TCO)**

**OCaml:**
```ocaml
let rec fold_left f acc = function
  | [] -> acc
  | head :: tail -> fold_left f (f acc head) tail
```
This is **guaranteed tail-recursive**. The recursive call is in tail position, so OCaml (and most Lisp/Scheme implementations) optimize it to a loop.

**Rust:**
```rust
match items {
    [] => init,
    [head, tail @ ..] => fold_left_recursive(f(init, head), tail, f),
}
```
This is **tail-recursive in structure** but Rust makes NO GUARANTEE of TCO. The compiler *may* optimize it, but:
- For large lists, this could overflow the stack
- `items.iter().fold()` is always loop-compiled and safe

**Winner:** OCaml wins on predictability. Rust wins on practical safety (use iterators!).

## Performance Implications

### Iterator-Based (Rust)
- **Pros:** Zero-copy, compiled to loop, cache-friendly, no stack overflow risk
- **Cons:** Slightly more verbose syntax
- **Best for:** Large lists, performance-critical code

### Recursive (Both Languages)
- **OCaml Pros:** Guaranteed TCO, elegant pattern matching
- **OCaml Cons:** Linked list traversal is cache-unfriendly
- **Rust Pros:** Shows functional style, clear semantics
- **Rust Cons:** Stack risk, no TCO guarantee
- **Best for:** Learning, small lists, composition patterns

## Summary: When to Use Each

| Scenario | Language | Approach | Reason |
|---|---|---|---|
| Large arrays (>10K) | Rust | Iterator | Stack-safe, cache-friendly |
| Beautiful code | OCaml | fold_left | Concise, elegant |
| Learning recursion | Rust | Recursive | Explicit pattern matching |
| Production system | Rust | Iterator | Predictable performance |
| Functional composition | OCaml | Standard lib | Currying + pipelines |
| Type-safe pipelines | Rust | Closures + chains | Iterator adapters |

Both languages excel at expressing fold operations. OCaml is more concise; Rust is more explicit and safer for large-scale data processing.
