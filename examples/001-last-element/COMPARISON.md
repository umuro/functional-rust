# OCaml vs Rust: Last Element

## Side-by-Side Comparison

### OCaml (Functional)
```ocaml
let rec last = function
  | [] -> None
  | [x] -> Some x
  | _ :: t -> last t
```

### Rust (Idiomatic)
```rust
fn last<T>(list: &[T]) -> Option<&T> {
    list.last()
}
```

### Rust (Functional Style)
```rust
fn last<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [.., last] => Some(last),
    }
}
```

---

## Key Differences

### 1. **Data Structure**

| OCaml | Rust |
|-------|------|
| `'a list` - Linked list (cons cells) | `&[T]` - Slice (contiguous memory) |
| Head access O(1), tail access O(1) | Index access O(1), any position |
| Recursive structure | Array view |

**Impact:** OCaml naturally uses recursion, Rust naturally uses indexing.

### 2. **Ownership & Borrowing**

| OCaml | Rust |
|-------|------|
| `Some x` - Returns owned value | `Some(&x)` - Returns borrowed reference |
| Garbage collected | Borrow checker enforced |
| No lifetime concerns | Lifetime must be valid |

**OCaml:**
```ocaml
let x = last [1; 2; 3]  (* x owns the value *)
```

**Rust:**
```rust
let list = vec![1, 2, 3];
let x = last(&list);  // x borrows from list
// list must stay alive while x exists
```

### 3. **Pattern Matching Syntax**

| Feature | OCaml | Rust |
|---------|-------|------|
| Empty list | `[]` | `[]` |
| Single element | `[x]` | `[x]` |
| Head + tail | `h :: t` | `[h, rest @ ..]` |
| All but last | N/A | `[init @ .., _]` |
| **Last element** | Manual recursion | `[.., last]` ✨ |

**Rust advantage:** Slice patterns can match from the end!

```rust
match list {
    [.., last] => Some(last),  // Direct last element match
}
```

### 4. **Performance**

| Approach | OCaml | Rust |
|----------|-------|------|
| Recursive | O(n) - Must traverse | O(n) - Stack overhead |
| Built-in | O(n) - `List.rev` then head | O(1) - Direct slice access ✨ |
| Memory | Stack frames | No recursion needed |

**Rust wins:** `.last()` is instant because slices know their length.

### 5. **Tail Recursion**

**OCaml (optimized):**
```ocaml
let last_tail lst =
  let rec aux acc = function
    | [] -> acc
    | h :: t -> aux (Some h) t
  in
  aux None lst
```

**Rust equivalent (but not idiomatic):**
```rust
fn last_tail<T>(list: &[T]) -> Option<&T> {
    list.iter().fold(None, |_, x| Some(x))
}
```

**Why tail recursion matters less in Rust:**
- Slices have O(1) access - no need for recursion
- Iterators compile to tight loops - no stack overhead
- `.last()` is already optimal

---

## When to Use Each Style

### Use `.last()` (Rust idiomatic)
```rust
let result = list.last();
```
✅ Simple, fast, clear  
✅ Most common case  
✅ Communicates intent

### Use pattern matching (learning FP)
```rust
match list {
    [] => None,
    [.., last] => Some(last),
}
```
✅ Educational value  
✅ Explicit logic  
✅ Translating OCaml

### Use recursion (avoid in production)
```rust
fn last_recursive<T>(list: &[T]) -> Option<&T> {
    match list {
        [] => None,
        [x] => Some(x),
        [_, rest @ ..] => last_recursive(rest),
    }
}
```
❌ Stack overflow risk  
❌ Slower than iteration  
❌ Not idiomatic Rust  
✅ OK for learning/comparison

---

## Type Signatures Explained

### OCaml
```ocaml
val last : 'a list -> 'a option
```
- `'a` = type parameter (any type)
- `'a list` = list of any type
- `'a option` = Some value or None
- **Returns owned value**

### Rust
```rust
fn last<T>(list: &[T]) -> Option<&T>
```
- `<T>` = generic type parameter
- `&[T]` = borrowed slice of T
- `Option<&T>` = Some reference or None
- **Returns borrowed reference** (note the `&`)

**Ownership difference:**
```rust
let list = vec![1, 2, 3];
let x = last(&list);  // x is Option<&i32>
// list still owns the data, x just borrows
```

---

## Compilation & Testing

### OCaml
```bash
ocaml example.ml
# ✓ All tests passed
```

### Rust
```bash
rustc example.rs && ./example
# last([1,2,3,4]) = Some(4)
# last([]) = None
# ✓ All tests passed

cargo test
# running 4 tests
# test tests::test_all_implementations ... ok
# test tests::test_empty ... ok
# test tests::test_multiple ... ok
# test tests::test_single ... ok
```

---

## Takeaways

1. **Rust's `.last()` is more efficient** - O(1) vs O(n)
2. **Pattern matching works similarly** - syntax slightly different
3. **Ownership matters** - Rust returns references, OCaml returns values
4. **Slice patterns are powerful** - `[.., last]` is elegant
5. **Recursion less important in Rust** - iterators + indexing preferred

**Philosophy:**
- OCaml: Recursion is natural (linked lists)
- Rust: Iteration is natural (contiguous arrays)

Both are functional, but Rust optimizes for the metal. 🦀
