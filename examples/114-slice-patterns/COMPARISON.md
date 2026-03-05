# OCaml vs Rust: Slice Patterns

## Side-by-Side Code

### OCaml
```ocaml
(* Head/tail destructuring on linked lists *)
let rec sum = function
  | [] -> 0
  | x :: rest -> x + sum rest

let describe = function
  | [] -> "empty"
  | [_] -> "singleton"
  | [_; _] -> "pair"
  | _ :: _ :: _ -> "many"
```

### Rust (idiomatic)
```rust
// Iterator-based — idiomatic for contiguous memory
pub fn sum_idiomatic(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

pub fn is_sorted_asc_idiomatic(slice: &[i32]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}
```

### Rust (functional/recursive — slice patterns)
```rust
// Structural match on slice shape — mirrors OCaml list patterns
pub fn sum(slice: &[i32]) -> i32 {
    match slice {
        [] => 0,
        [x, rest @ ..] => x + sum(rest),
    }
}

pub fn first_and_last(slice: &[i32]) -> Option<(i32, i32)> {
    match slice {
        [] => None,
        [only] => Some((*only, *only)),
        [head, .., tail] => Some((*head, *tail)),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive sum | `val sum : int list -> int` | `fn sum(slice: &[i32]) -> i32` |
| Head/tail pattern | `x :: rest` | `[x, rest @ ..]` |
| Empty pattern | `[]` | `[]` |
| Singleton pattern | `[x]` | `[x]` |
| First and last | `List.hd`, `List.rev \|> List.hd` | `[head, .., tail]` (one match arm) |
| List type | `'a list` (linked list) | `&[T]` (contiguous slice) |

## Key Insights

1. **Memory model**: OCaml's `::` patterns work on singly-linked lists with O(1) head access. Rust's slice patterns work on contiguous memory — a fundamentally different layout that enables cache-friendly traversal.

2. **The `rest @ ..` binding**: `rest @ ..` in Rust captures the remainder of the slice as a `&[T]` — the `@` binds a name to the matched segment. OCaml's `rest` in `x :: rest` is just a variable bound to the tail list. Both give you "everything after the head", but Rust's tail is a slice reference, not a new allocation.

3. **`[head, .., tail]` has no OCaml equivalent**: Matching on both the first and last element simultaneously requires an explicit list traversal in OCaml. Rust's `..` skip pattern does it in one arm with no extra allocation, exploiting random-access into the slice.

4. **Exhaustiveness at compile time**: Both OCaml and Rust compilers check that all cases are covered. Missing a `[]` arm is a compile error in both. In Rust this extends to numeric ranges and struct fields — the same principle, broader application.

5. **When to prefer each style**: The recursive slice-pattern style mirrors OCaml thinking and is readable for algorithms naturally expressed as "head + tail". The idiomatic iterator style (`.sum()`, `.windows(2)`) compiles to tighter loops and should be preferred when the standard library already has the operation.

## When to Use Each Style

**Use idiomatic Rust (iterators) when:** the operation maps cleanly to `.sum()`, `.take()`, `.windows()`, `.zip()` or other iterator adapters — these compile to tight loops with no recursion overhead.

**Use recursive slice patterns when:** the algorithm is naturally recursive (e.g., merge sort, parsing, tree-shaped data embedded in slices), or when you want to make the OCaml parallel explicit for teaching purposes.
