# OCaml vs Rust: UnsafeCell — The Foundation of Interior Mutability

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: mutable cells are first-class via `ref`.
   Interior mutability is transparent — the type system does not
   distinguish shared vs exclusive references. *)

let cell : int ref = ref 0

let set   (c : int ref) (v : int) : unit = c := v
let get   (c : int ref) : int           = !c
let upd   (c : int ref) (f : int -> int) : unit = c := f !c

let () =
  upd cell (fun v -> v + 5);
  upd cell (fun v -> v + 3);
  Printf.printf "Cell value: %d\n" (get cell);
  set cell 100;
  Printf.printf "After set:  %d\n" (get cell)
```

### Rust (idiomatic — using std::cell::Cell)
```rust
use std::cell::Cell;

fn main() {
    let cell = Cell::new(0_i32);
    cell.update(|v| v + 5);
    cell.update(|v| v + 3);
    println!("Cell value: {}", cell.get());
    cell.set(100);
    println!("After set:  {}", cell.get());
}
```

### Rust (from scratch — raw UnsafeCell)
```rust
use std::cell::UnsafeCell;

pub struct MyCell<T> {
    inner: UnsafeCell<T>,
}

impl<T: Copy> MyCell<T> {
    pub fn new(value: T) -> Self { Self { inner: UnsafeCell::new(value) } }

    pub fn set(&self, value: T) {
        // SAFETY: MyCell is !Sync; only one thread at a time.
        unsafe { *self.inner.get() = value }
    }

    pub fn get(&self) -> T {
        unsafe { *self.inner.get() }
    }

    pub fn update(&self, f: impl FnOnce(T) -> T) {
        self.set(f(self.get()));
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable cell type | `'a ref` | `UnsafeCell<T>` / `Cell<T>` |
| Allocate | `ref 0` | `UnsafeCell::new(0)` |
| Write | `c := v` | `c.set(v)` |
| Read | `!c` | `c.get()` |
| Mutate in place | `c := f !c` | `c.update(f)` |
| Thread safety | not guaranteed | `!Sync` enforced by compiler |

## Key Insights

1. **No special primitive in OCaml.** OCaml's `ref` is a heap-allocated mutable
   cell built into the runtime; the type system does not track aliasing, so
   mutation through any number of copies of a `ref` is always safe (and always
   possible).  In Rust, mutating through a shared reference requires an explicit
   opt-in — `UnsafeCell`.

2. **`UnsafeCell` is the *only* blessed escape hatch.** Rust's memory model
   forbids mutation through `&T` unless `UnsafeCell` is somewhere in the
   containment chain.  Every interior-mutability type in `std` —
   `Cell`, `RefCell`, `Mutex`, `RwLock`, `AtomicUsize` — wraps `UnsafeCell`
   internally.  Trying to sidestep it (e.g., casting `*const T` to `*mut T`
   from a `&T`) is undefined behaviour.

3. **`!Sync` is automatic.** Because `UnsafeCell<T>` does not implement `Sync`,
   any struct that contains it (like `MyCell`) also loses `Sync` for free.
   This means the compiler prevents you from accidentally sharing a
   `MyCell` across threads — you must use a `Mutex` or `RwLock` instead.

4. **The `unsafe` block documents intent, not just permission.** Writing
   `unsafe { *self.inner.get() = value }` forces you to articulate *why* this
   is safe (single-threaded, no aliased mutable references).  OCaml simply
   allows the mutation without comment.

5. **Two cells, two contracts.** `MyCell<T: Copy>` allows repeated overwrites
   and reads.  `MyOnceCell<T>` demonstrates a stricter contract — write-once
   — that is impossible to express cleanly without `UnsafeCell` but trivial
   to build with it.

## When to Use Each Style

**Use `std::cell::Cell<T>` when:** you need single-threaded interior mutability
for `Copy` types and want zero-cost, zero-unsafe code; the standard library
already built the safe wrapper for you.

**Use `UnsafeCell<T>` directly when:** you are building a new synchronisation
primitive, a custom allocator-backed container, or an FFI-friendly struct where
neither `Cell` nor `RefCell` fit the contract you need to enforce.
