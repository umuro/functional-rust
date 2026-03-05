# OCaml vs Rust: Cell<T> — Interior Mutability for Copy Types

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml ref — a mutable cell, always heap-allocated *)
let counter = ref 0
let () =
  counter := !counter + 1;
  counter := !counter + 1;
  assert (!counter = 2)

(* Mutable field in a record *)
type config = { name : string; mutable call_count : int }
let use_config c =
  c.call_count <- c.call_count + 1
```

### Rust (idiomatic — Cell<T>)
```rust
use std::cell::Cell;

// Immutable binding, mutable interior
let counter = Cell::new(0u32);
counter.set(counter.get() + 1);
counter.set(counter.get() + 1);
assert_eq!(counter.get(), 2);

// Struct with selectively mutable field
struct Config { name: String, call_count: Cell<u32> }
fn use_config(c: &Config) {           // shared ref — no &mut needed
    c.call_count.set(c.call_count.get() + 1);
}
```

### Rust (functional / cached)
```rust
use std::cell::Cell;

struct CachedSquare { input: i32, cache: Cell<Option<i32>> }

impl CachedSquare {
    fn get(&self) -> i32 {
        match self.cache.get() {
            Some(v) => v,
            None => {
                let v = self.input * self.input;
                self.cache.set(Some(v));
                v
            }
        }
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable cell | `'a ref` | `Cell<T>` where `T: Copy` |
| Read cell | `!r` (dereference) | `cell.get()` |
| Write cell | `r := value` | `cell.set(value)` |
| Mutable record field | `mutable field : t` | `field: Cell<T>` |
| Swap and return old | `let old = !r in r := v; old` | `cell.replace(v)` |
| Function receiver | passes record by value or ref | `&self` (shared ref) |

## Key Insights

1. **No `mut` binding required.** In OCaml every `ref` is implicitly mutable;
   in Rust a `Cell<T>` binding can be immutable (`let c = Cell::new(0)`) yet
   still accept `set` calls.  The mutability lives *inside* the type, not on
   the binding.

2. **Copy-only constraint.** OCaml `ref` works for any type.  `Cell<T>` requires
   `T: Copy` because it can only move values in/out — it never hands out a
   reference to the interior, which is exactly how it sidesteps the borrow
   checker's aliasing rules.

3. **Shared-reference mutation.** Rust normally forbids mutating through `&T`.
   `Cell` is the explicit exception for single-threaded code: `cell.set(v)`
   compiles on `&Cell<T>`.  The OCaml equivalent is a `mutable` record field or
   a `ref` value stored in a record — mutation through any alias is always allowed.

4. **No runtime cost.** Unlike `RefCell<T>`, `Cell<T>` performs no borrow-count
   bookkeeping at runtime.  The safety guarantee comes entirely from the
   copy-only API at compile time.

5. **Not `Sync`.** `Cell<T>` is `!Sync`, so it cannot be shared across threads.
   For multi-threaded use, reach for `Mutex<T>` or `AtomicT`; for single-threaded
   non-Copy types use `RefCell<T>`.  OCaml's GIL (in the classic runtime) means
   refs are also not truly thread-safe without explicit coordination.

## When to Use Each Style

**Use `Cell<T>` when:** you have a `Copy` field (counters, flags, cached numeric
results) that must be mutable through a shared reference in single-threaded code
and you want zero runtime overhead.

**Use `RefCell<T>` when:** the inner type is not `Copy` (e.g. `String`, `Vec`)
and you are still in a single-threaded context.

**Use OCaml `ref` / `mutable` when:** you are in OCaml — every value can be
wrapped in a `ref` without type-system restrictions; there is no Copy/non-Copy
distinction to worry about.
