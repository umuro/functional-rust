# OCaml vs Rust: List Map From Scratch

## Side-by-Side Code

### OCaml

```ocaml
(* Recursive abstraction: extract the pattern of applying f to each element *)
let rec map f = function
  | []     -> []
  | h :: t ->
    let h' = f h in
    h' :: map f t

(* Partial application: bind map to a specific function *)
let add1      = map (fun x -> x + 1)
let to_string = map string_of_int
let double    = map (fun x -> x * 2)

(* Usage: apply the specialized transformers *)
let nums = [1; 2; 3; 4; 5]
let () =
  List.iter (Printf.printf "%d ") (add1 nums);   (* [2; 3; 4; 5; 6] *)
  List.iter (Printf.printf "%s ") (to_string nums); (* ["1"; "2"; "3"; "4"; "5"] *)
  List.iter (Printf.printf "%d ") (double nums)  (* [2; 4; 6; 8; 10] *)
```

### Rust (idiomatic)

```rust
// Idiomatic Rust: use iterator chains with the built-in map
pub fn map_idiomatic<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    items.iter().map(|&x| f(x)).collect()
}

// Partial application in Rust: wrap map with a closure that captures the function
pub fn add_one(items: &[i32]) -> Vec<i32> {
    map(|x| x + 1, items)
}

pub fn to_string_int(items: &[i32]) -> Vec<String> {
    map(|x| x.to_string(), items)
}

pub fn double(items: &[i32]) -> Vec<i32> {
    map(|x| x * 2, items)
}

// Usage
fn main() {
    let nums = &[1, 2, 3, 4, 5];
    println!("{:?}", add_one(nums));      // [2, 3, 4, 5, 6]
    println!("{:?}", to_string_int(nums)); // ["1", "2", "3", "4", "5"]
    println!("{:?}", double(nums));       // [2, 4, 6, 8, 10]
}
```

### Rust (functional/recursive)

```rust
// Recursive Rust: explicit recursion with slice pattern matching
pub fn map_recursive<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    match items {
        [] => Vec::new(),
        [head, rest @ ..] => {
            let mut result = vec![f(*head)];
            result.extend(map_recursive(f, rest));
            result
        }
    }
}

// Demonstrates the same abstraction: extract and compose the per-element operation
// with the recursive structure of the list
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function type signature | `('a -> 'b) -> 'a list -> 'b list` | `fn map<T, U, F>(f: F, items: &[T]) -> Vec<U> where F: Fn(T) -> U, T: Copy` |
| Partial application (add1) | `int list -> int list` | `fn add_one(items: &[i32]) -> Vec<i32>` |
| List representation | `'a list` (linked list) | `&[T]` (slice) or `Vec<T>` (heap-allocated) |
| Function parameter | `'a -> 'b` (implicit currying) | `F where F: Fn(T) -> U` (trait bound) |
| Optional value | N/A (returns list) | `Vec<U>` (always returns a vector) |

## Key Insights

1. **The Abstraction Principle:** Both languages extract the common pattern (apply f to each element) into a reusable function. OCaml's implicit currying makes partial application effortless; Rust requires explicit wrapper functions or closures, but the underlying abstraction is identical.

2. **Data Structure Idioms:** OCaml uses immutable linked lists; Rust uses slices for borrowed data. Rust's approach is memory-efficient but requires `Copy` bounds when elements are borrowed. The `Vec` return type allocates on the heap, mirroring OCaml's list construction.

3. **Recursion vs Iteration:** OCaml's recursive style is natural and idiomatic. Rust offers recursion, but iterator chains (`.iter().map().collect()`) are preferred for clarity and safety. Both express the same transformation; iterators avoid stack overhead for large lists.

4. **Closure Trait Bounds:** Rust's `Fn` trait bounds replace OCaml's implicit function types. The bound `F: Fn(T) -> U` means "F is a callable that takes T and returns U"—equivalent to OCaml's `'a -> 'b`, but explicit in the code.

5. **Memory Safety Trade-off:** Rust's slice-based `map` requires `T: Copy` to borrow elements efficiently. This constraint is absent in OCaml because all values are boxed. The `Copy` bound ensures we don't move data from the borrowed slice; without it, we'd need to clone or take ownership, changing the semantics.

## When to Use Each Style

**Use idiomatic Rust when:** Writing production code that transforms lists, ranges, or iterables. Iterator chains are readable, efficient, and compose naturally: `nums.iter().map(f).filter(g).collect()`.

**Use recursive Rust when:** Teaching functional abstraction, demonstrating the structural recursion pattern, or implementing custom list types where recursion is the only option. Recursive implementations are slower (stack overhead) but reveal the underlying algorithm.

**Use OCaml when:** Exploring rapid prototyping of algorithms, mathematical transformations, or functional patterns where currying and pattern matching shine. OCaml's implicit partial application makes higher-order functions feel lightweight.
