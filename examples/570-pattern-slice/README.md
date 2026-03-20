📖 **[View on hightechmind.io →](https://hightechmind.io/rust/570-pattern-slice)**

---

# Slice Patterns

## Problem Statement

Recursive algorithms over lists and arrays naturally decompose into head/tail or first/rest cases — the foundation of functional programming. OCaml's `match` on lists is legendary for enabling clean recursive functions. Rust's slice patterns bring the same capability: `[first, rest @ ..]` matches a non-empty slice and binds head and tail. This enables recursive-style processing of arrays and slices, pattern-based parsing, and elegant handling of variable-length inputs.

## Learning Outcomes

- How `[a, b, c]` matches exactly three elements
- How `[first, rest @ ..]` matches head and tail (OCaml-style list decomposition)
- How `[first, second, .., last]` matches first, second, and last elements simultaneously
- How slice patterns combine with guards for conditional matching
- Where slice patterns replace length checks followed by indexing in recursive algorithms

## Rust Application

`describe_triple(arr: &[i32; 3])` matches exact patterns like `[0, 0, 0]` and `[a, b, c]`. `first_and_rest(slice: &[i32])` uses `[first, rest @ ..]` — the direct Rust equivalent of OCaml `hd :: tl` — and returns the head and tail slice. `first_two_last(slice: &[i32])` uses `[first, second, .., last]` — matching three specific positions while ignoring the middle. All patterns automatically handle the empty case for exhaustiveness.

Key patterns:
- `[first, rest @ ..]` — head/tail decomposition
- `[a, b, c]` — exact fixed-length match
- `[first, .., last]` — first and last of unknown length
- `[]` — empty slice pattern

## OCaml Approach

OCaml list pattern matching is the original inspiration for slice patterns:

```ocaml
let rec first_and_rest = function
  | [] -> None
  | x :: rest -> Some (x, rest)

let rec sum = function
  | [] -> 0
  | x :: rest -> x + sum rest
```

OCaml's linked list `::` operator is idiomatic for head/tail; Rust slice patterns achieve the same for arrays.

## Key Differences

1. **Linked list vs slice**: OCaml `::` works on linked lists (O(1) head/tail); Rust `[first, rest @ ..]` works on contiguous slices — `rest` is a `&[T]`, a slice reference.
2. **Fixed-length arrays**: Rust `[a, b, c]` on `[i32; 3]` is exhaustive by definition; OCaml arrays require bounds checking.
3. **Middle elements**: Rust `[first, .., last]` extracts first and last; OCaml requires indexing: `arr.(0)` and `arr.(Array.length arr - 1)`.
4. **Performance**: Rust slice patterns compile to offset arithmetic on a pointer; OCaml list patterns dereference linked list nodes.

## Exercises

1. **Recursive sum**: Implement `fn sum(s: &[i32]) -> i32` using `match s { [] => 0, [x, rest @ ..] => x + sum(rest) }` — identical structure to OCaml's recursive list sum.
2. **Palindrome check**: Write `fn is_palindrome(s: &[i32]) -> bool` using slice patterns: `[] | [_] => true`, `[first, middle @ .., last] if first == last => is_palindrome(middle)`, `_ => false`.
3. **CSV row parser**: Implement `fn parse_coords(parts: &[&str]) -> Option<(f64, f64, f64)>` using `[x, y, z]` and `[x, y, z, ..]` patterns to accept exactly three or more fields.
