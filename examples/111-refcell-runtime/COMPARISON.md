# OCaml vs Rust: RefCell<T> — Runtime Borrow Checking

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has no borrow checker — mutation is always available via ref *)
let items = ref [] in
items := "first" :: !items;
items := "second" :: !items;
let result = List.rev !items in
assert (result = ["first"; "second"; "third"])

(* Mutable field in a record — no special ceremony needed *)
type 'a stack = { mutable data : 'a list }
let push s x = s.data <- x :: s.data
let pop s = match s.data with
  | [] -> None
  | x :: rest -> s.data <- rest; Some x
```

### Rust (idiomatic — interior mutability)
```rust
use std::cell::RefCell;

// Immutable binding; RefCell provides controlled interior mutation
let items: RefCell<Vec<String>> = RefCell::new(Vec::new());
items.borrow_mut().push("first".to_string());
items.borrow_mut().push("second".to_string());
let snapshot = items.borrow().clone();
assert_eq!(snapshot, vec!["first", "second"]);
```

### Rust (shared mutable stack with &self API)
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
| Mutable reference | `'a ref` | `RefCell<T>` |
| Get a shared borrow | `!cell` (dereference) | `cell.borrow()` → `Ref<T>` |
| Get an exclusive borrow | `cell := value` | `cell.borrow_mut()` → `RefMut<T>` |
| Non-panicking borrow | N/A (no borrow checker) | `cell.try_borrow()` → `Result<Ref<T>, BorrowError>` |
| Method receiver with interior mutation | `s.data <- x` (mutable field) | `&self` + `RefCell` inside struct |

## Key Insights

1. **No borrow checker in OCaml:** OCaml's `ref` and mutable record fields are always writable — there is no concept of shared vs exclusive borrows, so `RefCell` has no direct OCaml equivalent. It exists purely to satisfy Rust's borrow checker.

2. **`&self` that mutates:** `RefCell` lets you write `push(&self, …)` instead of `push(&mut self, …)`. This unlocks `Rc<Stack<T>>` sharing: multiple owners can call `push` without fighting over `&mut` access, because exclusivity is checked at the `borrow_mut()` call site rather than at the function signature.

3. **Panic on violation:** If you call `borrow_mut()` while a `Ref` or another `RefMut` is alive, the program panics. OCaml simply allows the mutation; Rust defers the safety check to runtime. Use `try_borrow_mut()` to get a `Result` instead of a panic.

4. **`Cell<T>` vs `RefCell<T>`:** For `Copy` types (integers, booleans), `Cell<T>` is cheaper — it copies values in and out with no reference counting overhead. `RefCell<T>` is needed when you must hand out actual references (`&T` / `&mut T`) to the interior, which `Cell` cannot do.

5. **Borrow guards as RAII:** `Ref<T>` and `RefMut<T>` are guard objects. The borrow count increments when they are created and decrements when they drop. This mirrors OCaml's GC-managed heap values, where the runtime tracks what is live — Rust just makes the tracking explicit and scope-bound.

## When to Use Each Style

**Use `RefCell<T>` when:** you need interior mutability for a non-`Copy` type — for example, a struct field that must be mutated via `&self`, a shared observer/logger behind `Rc`, or a recursive data structure whose nodes need parent references.

**Use `Cell<T>` instead when:** the value is `Copy` (integers, bools, small structs that derive `Copy`) and you only need get/set semantics — `Cell` has zero runtime overhead and cannot panic.

**Prefer `Mutex<T>` or `RwLock<T>` when:** the value crosses thread boundaries (`RefCell` is not `Sync`).
