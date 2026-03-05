# OCaml vs Rust: DoubleEndedIterator

## Side-by-Side Code

### OCaml
```ocaml
(* Palindrome via array index arithmetic — O(n) allocation to convert list *)
let palindrome_check lst =
  let arr = Array.of_list lst in
  let n = Array.length arr in
  let rec aux i j =
    if i >= j then true
    else arr.(i) = arr.(j) && aux (i + 1) (j - 1)
  in
  aux 0 (n - 1)

(* Back access requires List.rev — copies the entire list *)
let last = function
  | [] -> None
  | lst -> Some (List.nth lst (List.length lst - 1))
```

### Rust (idiomatic — DoubleEndedIterator)
```rust
pub fn palindrome_check<T: PartialEq>(data: &[T]) -> bool {
    let mut iter = data.iter();
    loop {
        match (iter.next(), iter.next_back()) {
            (Some(a), Some(b)) => if a != b { return false; }
            _ => return true,
        }
    }
}

pub fn last_element<T>(data: &[T]) -> Option<&T> {
    data.iter().next_back()
}
```

### Rust (functional/recursive — mirrors OCaml index approach)
```rust
pub fn palindrome_check_recursive<T: PartialEq>(data: &[T]) -> bool {
    match data {
        [] | [_] => true,
        [first, rest @ .., last] => first == last && palindrome_check_recursive(rest),
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Palindrome | `val palindrome_check : 'a list -> bool` | `fn palindrome_check<T: PartialEq>(data: &[T]) -> bool` |
| Last element | `val last : 'a list -> 'a option` | `fn last_element<T>(data: &[T]) -> Option<&T>` |
| Back iterator | N/A (no trait) | `iter.next_back()` via `DoubleEndedIterator` |
| Reversed iteration | `List.rev lst \|> List.iter` | `data.iter().rev()` — zero cost, no allocation |

## Key Insights

1. **Back access cost:** OCaml lists are singly-linked — `List.nth lst (len-1)` is O(n) and `List.rev` allocates a copy. Rust slices implement `DoubleEndedIterator` natively, so `.next_back()` is O(1) and zero-allocation.

2. **`.rev()` is free in Rust:** `Iterator::rev()` is a zero-cost adaptor that merely swaps which end `.next()` reads from. It does not create a reversed copy of the data — unlike OCaml's `List.rev`.

3. **Simultaneous traversal:** Rust's `DoubleEndedIterator` allows `.next()` and `.next_back()` on the *same* iterator, so front and back converge toward the middle. OCaml has no equivalent trait; you need explicit index variables or a converted array.

4. **Slice pattern matching:** The recursive Rust version uses `[first, rest @ .., last]` — a slice pattern that deconstructs both ends in one `match` arm, directly mirroring OCaml's structural recursion without allocating an array.

5. **Ownership safety:** Rust's iterator protocol ensures the front and back pointers never cross — the iterator becomes exhausted safely. In OCaml, the index-based `aux i j` loop relies on the programmer maintaining `i <= j`; Rust enforces this invariant in the type system via the iterator's internal state.

## When to Use Each Style

**Use idiomatic Rust (DoubleEndedIterator) when:** you want zero-allocation bidirectional traversal on slices, strings, or any collection that implements the trait — palindrome checks, symmetric reductions, trimming from both ends.

**Use recursive Rust (slice patterns) when:** the algorithm is naturally expressed as structural recursion and you want a direct correspondence with functional OCaml code for clarity or teaching purposes.
