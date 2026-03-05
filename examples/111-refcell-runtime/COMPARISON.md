# OCaml vs Rust: RefCell<T> — Runtime Borrow Checking

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: mutable reference inside an immutable binding *)
let collect_items () =
  let items = ref [] in
  items := "first" :: !items;
  items := "second" :: !items;
  items := "third" :: !items;
  List.rev !items

(* Shared mutable stack with mutable record field *)
type 'a stack = { mutable data : 'a list }
let push s x = s.data <- x :: s.data
let pop s = match s.data with
  | [] -> None
  | x :: rest -> s.data <- rest; Some x
```

### Rust (idiomatic — RefCell interior mutability)
```rust
use std::cell::RefCell;

pub fn collect_items() -> Vec<String> {
    let items: RefCell<Vec<String>> = RefCell::new(Vec::new());
    items.borrow_mut().push("first".to_string());
    items.borrow_mut().push("second".to_string());
    items.borrow_mut().push("third".to_string());
    let borrowed = items.borrow();
    borrowed.clone()
}
```

### Rust (functional — interior-mutable Stack via &self)
```rust
pub struct Stack<T> {
    data: RefCell<Vec<T>>,
}

impl<T> Stack<T> {
    pub fn push(&self, value: T) {          // &self, not &mut self
        self.data.borrow_mut().push(value);
    }
    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable binding | `let x = ref value` | `let x = RefCell::new(value)` |
| Read access | `!x` (deref ref) | `x.borrow()` → `Ref<T>` |
| Write access | `x := new_val` | `x.borrow_mut()` → `RefMut<T>` |
| Mutable field | `mutable field : 'a` | `field: RefCell<T>` |
| Borrow violation | — (no rule) | Runtime panic |
| Safe fallible borrow | — | `try_borrow()` → `Result<Ref<T>, BorrowError>` |

## Key Insights

1. **OCaml has no borrow rules** — any `ref` value can be read and written freely at any point; mutation safety is the programmer's responsibility alone.

2. **RefCell defers the borrow checker to runtime** — Rust's "one writer XOR many readers" rule is enforced when `borrow()` / `borrow_mut()` is called, not at compile time. Violation panics rather than failing to compile.

3. **Interior mutability unlocks `&self` mutation** — `Stack::push(&self)` can mutate internal state without requiring `&mut self`, enabling shared ownership via `Rc<Stack<T>>` without `Rc<RefCell<Stack<T>>>`.

4. **`try_borrow` avoids panics** — when the borrow outcome is uncertain (e.g., in library code), `try_borrow()` returns `Result` instead of panicking, mirroring OCaml's implicit "it just works" but with explicit error handling.

5. **RAII guards — `Ref<T>` and `RefMut<T>`** — borrows are tracked via guard objects that decrement the borrow count when dropped, so sequential `borrow_mut()` calls in separate statements never overlap.

## When to Use Each Style

**Use `RefCell::borrow_mut()` directly** when mutating through a non-shared local value and you want clarity that each statement releases its borrow before the next starts.

**Use `RefCell` inside a struct** when you need `&self` methods that still mutate state — most commonly for mock/spy objects in tests, observers/loggers, and data structures shared via `Rc`.
