# 1001-list-filter: Filtering Lists in Rust

Convert OCaml list filtering to idiomatic Rust, demonstrating iterator-based and recursive approaches.

## Learning Outcomes

By studying this example, you'll learn:

1. **Iterators in Rust** — How to use `.filter()`, `.collect()`, and other iterator combinators
2. **Functional Programming** — Applying functional idioms in Rust (map, filter, fold patterns)
3. **Recursive Patterns** — Implementing recursion in Rust with pattern matching
4. **Type Safety & Generics** — How Rust's type system enables reusable filtering functions
5. **Testing** — Comprehensive unit tests covering edge cases (empty, single, all/none match)

## The Problem

Given a list of integers, **filter and keep only elements matching a condition**.

Example: `[1, 2, 3, 4, 5, 6, 7, 8]` → evens: `[2, 4, 6, 8]`, odds: `[1, 3, 5, 7]`

## OCaml Approach

```ocaml
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds = List.filter (fun x -> x mod 2 <> 0) numbers
```

**Key traits:**
- **Immutable by default** — `List.filter` creates a new list
- **Higher-order functions** — Predicates are first-class values
- **Pattern matching** — Canonical for recursion
- **Linked lists** — Built-in `list` type is a singly-linked list

## Rust Approach

### 1. Idiomatic Rust (Iterators)

```rust
pub fn filter_iter<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

let evens = filter_iter(&numbers, |x| x % 2 == 0);
let odds = filter_iter(&numbers, |x| x % 2 != 0);
```

**Advantages:**
- **Lazy evaluation** — Iterators compose without intermediate allocations
- **Zero-copy scanning** — Borrows elements, doesn't clone until `.collect()`
- **In-place variant** — Use `.retain()` for mutable filtering without allocation
- **Chainable** — Compose with `.map()`, `.take()`, `.filter()`, etc.

### 2. Recursive Functional Style

```rust
pub fn filter_recursive<T: Clone>(items: &[T], pred: impl Fn(&T) -> bool + Copy) -> Vec<T> {
    match items {
        [] => Vec::new(),
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, pred);
            if pred(head) {
                let mut result = vec![head.clone()];
                result.append(&mut rest);
                result
            } else {
                rest
            }
        }
    }
}
```

**Parallels OCaml:** Direct translation of the functional pattern, uses pattern matching.

**Trade-offs:** More allocation (per-element vec creation), but demonstrates recursion idioms.

## 4 Key Differences: OCaml vs Rust

### 1. **List Representation**
| OCaml | Rust |
|-------|------|
| Singly-linked lists (`[H \| T]`) | Vectors (contiguous arrays) + iterators |
| Cheap cons (`::`) operation | Cheap indexing + borrowing |
| Natural recursion (list structure) | Natural iteration (contiguous data) |

**Impact:** Rust prefers iterators over recursion for performance; OCaml prefers recursion over iteration.

### 2. **Memory Management**
| OCaml | Rust |
|-------|------|
| Garbage collected | Owned/borrowed (compile-time) |
| Automatic cleanup | Explicit `.clone()` when copying |
| Immutable by default | Mutable/immutable chosen by programmer |

**Impact:** Rust requires explicit cloning (`|x| x % 2 == 0` borrows, then `.cloned().collect()` copies). OCaml is automatic.

### 3. **Function Types**
| OCaml | Rust |
|-------|------|
| `'a -> bool` | `impl Fn(&T) -> bool` |
| Implicit closures | Generic trait bounds + closure inference |
| `fun x -> x mod 2 = 0` | `\|x\| x % 2 == 0` |

**Impact:** Rust's trait bounds are more explicit but offer better zero-cost abstraction and specialization.

### 4. **Error Handling & Type Safety**
| OCaml | Rust |
|-------|------|
| `Option` + pattern matching | `Option<T>` + pattern matching |
| Partial functions (can panic) | Non-nullable references by default |
| Type inference (very broad) | Type inference (with explicit bounds) |

**Impact:** Rust's borrow checker prevents use-after-free; OCaml's GC handles it. Both support algebraic types.

## Test Coverage

Run tests:
```bash
cargo test -p example-1001-list-filter
```

**22 passing unit tests** covering:
- ✅ Multiple elements (evens, odds, complex predicates)
- ✅ Empty lists
- ✅ Single elements (match/no-match)
- ✅ All match / none match edge cases
- ✅ Generic types (strings, integers)
- ✅ Equivalence (iterator vs recursive)

## Quality Gates

All passing:
```bash
✅ cargo fmt -p example-1001-list-filter
✅ cargo clippy -p example-1001-list-filter -- -D warnings
✅ cargo test -p example-1001-list-filter (22 tests)
```

## Files

- `src/lib.rs` — Library with `filter_iter`, `filter_in_place`, `filter_recursive` + tests
- `src/example.rs` — Executable demonstrating all three approaches
- `example.ml` — OCaml equivalent (idiomatic + recursive)
- `Cargo.toml` — Package metadata (edition 2021, no dependencies)
- `README.md` — This file
- `COMPARISON.md` — Detailed side-by-side comparison

## Further Learning

1. **Laziness vs Eagerness** — Experiment with `.iter()` vs `.collect()` timing
2. **Performance** — Bench iterator vs recursive (iterators win for large lists)
3. **Generic Predicates** — Extend to `Fn(T) -> bool` vs `Fn(&T) -> bool` trade-offs
4. **Combinators** — Chain `.filter().map().take()` without intermediate allocations

---

**Category:** stdlib-list  
**Difficulty:** Beginner → Intermediate  
**Topics:** Iterators, Functional Programming, Generics, Testing
