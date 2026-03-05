# OCaml vs Rust: List.flatten — Flatten Nested Lists

## Side-by-Side Code

### OCaml
```ocaml
(* Idiomatic OCaml — stdlib *)
let nested = [[1; 2]; [3; 4; 5]; [6]; [7; 8; 9; 10]]
let flat = List.flatten nested
(* -> [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] *)

(* concat_map: map then flatten in one pass *)
let pairs = List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]
(* -> [1; 10; 2; 20; 3; 30] *)
```

### Rust (idiomatic)
```rust
pub fn flatten_idiomatic<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flatten().cloned().collect()
}

pub fn concat_map<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> Vec<U>,
{
    items.iter().flat_map(f).collect()
}
```

### Rust (functional/recursive)
```rust
pub fn flatten_recursive<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    match nested {
        [] => vec![],
        [head, rest @ ..] => {
            let mut result = head.clone();
            result.extend(flatten_recursive(rest));
            result
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| flatten signature | `val flatten : 'a list list -> 'a list` | `fn flatten_idiomatic<T: Clone>(nested: &[Vec<T>]) -> Vec<T>` |
| concat_map signature | `val concat_map : ('a -> 'b list) -> 'a list -> 'b list` | `fn concat_map<T, U, F>(items: &[T], f: F) -> Vec<U>` |
| Nested list type | `'a list list` | `&[Vec<T>]` (slice of owned vecs) |
| Element constraint | None (GC handles all) | `T: Clone` (must clone out of borrow) |
| Iterator adapter | `List.flatten` | `.flatten()` + `.cloned()` |
| Map+flatten adapter | `List.concat_map` | `.flat_map(f)` |

## Key Insights

1. **Direct stdlib mapping:** OCaml's `List.flatten` maps exactly to Rust's `.flatten()` iterator adapter — the concepts are identical, only the syntax differs.
2. **Ownership cost:** The Rust idiomatic version needs `.cloned()` because it borrows the nested structure; OCaml's GC makes this transparent.
3. **concat_map is flat_map:** OCaml's `List.concat_map f lst` is precisely Rust's `lst.iter().flat_map(f).collect()` — both apply a function and concatenate results in a single pass.
4. **Slice patterns enable recursion:** Rust's `[head, rest @ ..]` pattern mirrors OCaml's `head :: rest` decomposition, making the recursive solution read almost identically.
5. **Performance:** The idiomatic iterator-based solution avoids intermediate allocations that the recursive version creates at each step.

## When to Use Each Style

**Use idiomatic Rust when:** You want efficient, allocation-minimal code in production — `.flatten()` and `.flat_map()` are zero-cost abstractions that compile to tight loops.

**Use recursive Rust when:** You are teaching the structural recursion pattern or translating OCaml code directly for comparison purposes — it makes the list decomposition explicit.
