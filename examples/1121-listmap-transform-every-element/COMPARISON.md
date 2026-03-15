# OCaml vs Rust: List.map — Transform Every Element

## Side-by-Side Code

### OCaml
```ocaml
let numbers = [1; 2; 3; 4; 5]
let doubled = List.map (fun x -> x * 2) numbers
let () = List.iter (fun x -> Printf.printf "%d " x) doubled
(* Output: 2 4 6 8 10 *)

(* More general: define a map function ourselves *)
let rec map f = function
  | [] -> []
  | x :: xs -> f x :: map f xs

let () =
  assert (map (fun x -> x * 2) [1;2;3;4;5] = [2;4;6;8;10])
```

### Rust (idiomatic)
```rust
pub fn map_idiomatic<F, T, U>(list: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    list.iter().map(f).collect()
}
```

### Rust (functional/recursive)
```rust
pub fn map_recursive<F, T, U>(list: &[T], f: &F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_recursive(tail, f));
            result
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val map : ('a -> 'b) -> 'a list -> 'b list` | `fn map_idiomatic<F, T, U>(list: &[T], f: F) -> Vec<U>` |
| List type | `'a list` (linked list) | `&[T]` (slice) / `Vec<T>` (owned vector) |
| Higher-order function | First-class functions, closures | Closures implementing `Fn`, `FnMut`, `FnOnce` |
| Return type | New list allocated | New `Vec<U>` allocated (owned) |

## Key Insights

1. **Ownership and Borrowing**: In Rust, the input list is borrowed as a slice (`&[T]`), which does not take ownership. The output is a new owned `Vec<U>`. This is similar to OCaml's functional style where the input list is immutable and a new list is returned.

2. **Type System**: Rust requires explicit generic bounds (`F: Fn(&T) -> U`) to ensure the closure can be called. OCaml's type inference automatically deduces the function type.

3. **Performance**: Rust's iterator-based `map` is zero-cost abstraction and can be highly optimized. The recursive version is less efficient due to repeated allocations and recursion depth limits, but mirrors OCaml's recursion pattern.

4. **Pattern Matching**: Both languages support pattern matching for recursive decomposition. Rust's slice patterns (`[head, tail @ ..]`) are similar to OCaml's `x :: xs`.

5. **Mutability**: The idiomatic Rust version uses immutable borrows and produces a new vector. The recursive version also avoids mutation inside the recursion (except for building the result vector via `extend`). OCaml's lists are immutable by default.

## When to Use Each Style

**Use idiomatic Rust when:** You are working with slices or iterators and want maximum performance and readability. The iterator chain `list.iter().map(f).collect()` is the standard Rust idiom and is immediately recognizable to Rust developers.

**Use recursive Rust when:** You are teaching functional programming concepts, comparing directly with OCaml, or need to mirror a recursive algorithm that doesn't fit well into iterators (e.g., tree traversal). Recursive Rust can be clearer for certain algorithms but may suffer from stack overflow for large inputs (unlike OCaml's tail‑call optimization).