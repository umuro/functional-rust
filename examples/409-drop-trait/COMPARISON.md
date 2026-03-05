# OCaml vs Rust: Drop Trait and RAII

## Side-by-Side Code

### OCaml — Explicit cleanup or with_* pattern
```ocaml
type file_handle = { name: string; mutable closed: bool }

let open_file name =
  { name; closed = false }

let close_file fh =
  if not fh.closed then fh.closed <- true

(* RAII via with_file *)
let with_file name f =
  let fh = open_file name in
  Fun.protect ~finally:(fun () -> close_file fh) (fun () -> f fh)

let () =
  with_file "data.txt" (fun f ->
    Printf.printf "Using: %s\n" f.name
  )
  (* Automatically closed, even on exception *)
```

### Rust — Drop trait (automatic RAII)
```rust
struct FileHandle {
    name: String,
    is_open: bool,
}

impl FileHandle {
    fn open(name: &str) -> Self {
        FileHandle { name: name.to_string(), is_open: true }
    }
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        if self.is_open {
            // Close file
            self.is_open = false;
        }
    }
}

fn main() {
    {
        let f = FileHandle::open("data.txt");
        println!("Using: {}", f.name);
    } // Drop called automatically here
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Cleanup mechanism | GC finalizers, `with_*` patterns | `Drop` trait (deterministic) |
| When cleanup runs | GC-dependent (non-deterministic) | End of scope (deterministic) |
| Explicit cleanup | `close_file fh`, `Fun.protect` | `std::mem::drop(x)` |
| RAII idiom | `with_file name (fun f -> ...)` | Just use scope: `{ let f = ... }` |
| Order | Non-deterministic | Reverse of creation |
| Exception safety | `Fun.protect ~finally:...` | Automatic via Drop |

---

## Drop Order

Rust drops in reverse order of creation:

```rust
{
    let a = Resource::new("A");  // Created first
    let b = Resource::new("B");
    let c = Resource::new("C");  // Created last
}
// Drop order: C, B, A (reverse)
```

---

## RAII Patterns

### Lock Guard
```rust
struct MutexGuard<'a, T> { /* ... */ }

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Release lock
    }
}

fn use_data(mutex: &Mutex<Data>) {
    let guard = mutex.lock();  // Lock acquired
    // Use guard...
} // Lock released here via Drop
```

### Transaction (Commit or Rollback)
```rust
struct Transaction { committed: bool }

impl Drop for Transaction {
    fn drop(&mut self) {
        if !self.committed {
            // Rollback
        }
    }
}

fn transfer(tx: Transaction) -> Result<(), Error> {
    // Do work...
    tx.commit();  // Marks as committed
    Ok(())
}  // If commit not called, rollback in drop
```

---

## Explicit Drop

```rust
let f = FileHandle::open("data.txt");
// ... use f ...
std::mem::drop(f);  // Drop now, before scope ends
// f is no longer usable
```

---

## 5 Takeaways

1. **Rust's Drop is deterministic; OCaml's finalizers are not.**
   Drop runs at end of scope, not when GC decides.

2. **RAII is built into Rust's ownership model.**
   No need for `with_*` wrappers — just use scope.

3. **Drop order is reverse of creation.**
   Important for resources with dependencies.

4. **Cannot implement both Drop and Copy.**
   Copy types don't have custom destructors.

5. **`std::mem::drop(x)` forces early cleanup.**
   Useful when you need cleanup before scope ends.
