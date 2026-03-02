# Example 002: List Operations and Recursion

## Concept

Lists are the fundamental data structure in functional programming. OCaml uses linked lists (cons cells), while Rust uses slices (`&[T]`) and vectors (`Vec<T>`). This example shows how to translate recursive list operations between the two languages.

## Key Differences

### Data Structures

**OCaml:**
```ocaml
let lst = [1; 2; 3]  (* Linked list, O(1) prepend *)
let head :: tail = lst  (* Pattern matching cons cells *)
```

**Rust:**
```rust
let lst = vec![1, 2, 3];  // Vec<T>, contiguous memory
let [head, tail @ ..] = &lst[..];  // Slice pattern matching
```

### Pattern Matching

**OCaml:**
- `[]` - empty list
- `head :: tail` - cons pattern (destructure into first element and rest)
- Native syntax for linked lists

**Rust:**
- `[]` - empty slice
- `[head, tail @ ..]` - slice pattern (destructure using rest pattern `@`)
- Requires `&[T]` slice reference, not `Vec<T>` directly

### Tail Recursion

Both languages benefit from tail-call optimization:

**OCaml:**
```ocaml
let rec aux acc = function
  | [] -> acc
  | head :: tail -> aux (acc + head) tail
```

**Rust:**
```rust
fn aux(acc: i32, lst: &[i32]) -> i32 {
    match lst {
        [] => acc,
        [head, tail @ ..] => aux(acc + head, tail),
    }
}
```

Rust doesn't guarantee TCO, but LLVM often optimizes it in release builds.

### Memory Management

**OCaml:**
- Garbage collected
- Lists are immutable and share structure
- `append` creates new spine but shares tail

**Rust:**
- Manual memory control (ownership)
- `Vec<T>` is heap-allocated, `&[T]` is borrowed view
- `append` must clone elements or take ownership

## Key Mappings

| OCaml | Rust | Notes |
|-------|------|-------|
| `list` | `Vec<T>` or `&[T]` | Vec for owned, slice for borrowed |
| `[]` | `vec![]` or `&[]` | Empty list/slice |
| `head :: tail` | `[head, tail @ ..]` | Cons vs slice pattern |
| `List.length` | `.len()` or custom | Built-in method vs recursion |
| `List.map` | `.iter().map()` | Iterator-based (lazy) |
| `@` (append) | `extend` or custom | Different performance characteristics |

## Performance Notes

1. **Rust slices are O(1) access** - OCaml lists are O(n)
2. **Recursive functions** - Stack depth limits (use iterators in production)
3. **Tail recursion** - Reduces stack usage but not guaranteed in Rust
4. **Cloning** - Rust requires explicit `.clone()` for owned data

## Functional Patterns

Both examples demonstrate:
- **Structural recursion** - Base case + recursive case
- **Immutability** - Functions don't modify inputs
- **Pattern matching** - Exhaustive case analysis
- **Higher-order functions** - `map` and `filter` take function arguments

## When to Use

**Recursive style:**
- Learning functional patterns
- Small datasets
- Clear algorithmic expression

**Iterator style (Rust):**
- Production code
- Large datasets
- Composable transformations
- Better compiler optimization

## Next Steps

Example 003 will explore pattern matching in depth, including guards, nested patterns, and exhaustiveness checking.
