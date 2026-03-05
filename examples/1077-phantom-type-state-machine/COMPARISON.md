# OCaml vs Rust: Phantom Type State Machine

## Side-by-Side Code

### OCaml
```ocaml
type opened
type closed
type 'state handle = { name: string; content: string list }

let open_file name : opened handle =
  { name; content = ["line1"; "line2"; "line3"] }

let read_line (h : opened handle) n : string =
  List.nth h.content n

let close_file (_ : opened handle) : closed handle =
  { name = "closed"; content = [] }
```

### Rust (idiomatic)
```rust
use std::marker::PhantomData;

struct Opened;
struct Closed;

struct FileHandle<State> {
    name: String,
    content: Vec<String>,
    _state: PhantomData<State>,
}

impl FileHandle<Opened> {
    fn read_line(&self, n: usize) -> Option<&str> {
        self.content.get(n).map(|s| s.as_str())
    }

    fn close(self) -> FileHandle<Closed> {
        FileHandle { name: self.name, content: vec![], _state: PhantomData }
    }
}
```

### Rust (runtime comparison — enum-based)
```rust
enum FileState { Open, Closed }

struct RuntimeFileHandle {
    name: String,
    content: Vec<String>,
    state: FileState,
}

impl RuntimeFileHandle {
    fn read_line(&self, n: usize) -> Result<&str, &'static str> {
        if self.state == FileState::Closed {
            return Err("cannot read from closed file");
        }
        self.content.get(n).map(|s| s.as_str()).ok_or("out of range")
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom parameter | `type 'state handle` | `struct FileHandle<State>` |
| State markers | `type opened` (abstract) | `struct Opened;` (zero-sized) |
| Phantom carrier | Built into type parameter | `PhantomData<State>` |
| State transition | Returns new phantom type | `self` consumed, new type returned |

## Key Insights

1. **Both languages achieve zero-cost type-level state machines** — the phantom parameter exists only for the type checker, never at runtime.
2. **Rust's move semantics add an extra guarantee** — `close(self)` consumes the handle, so you can't accidentally keep using it. OCaml's `close_file` returns a new value but doesn't prevent keeping the old one.
3. **OCaml's abstract types vs Rust's zero-sized types** — OCaml's `type opened` has no constructors; Rust's `struct Opened;` is a unit struct. Both serve as compile-time-only markers.
4. **`PhantomData` is Rust's explicit marker** — OCaml doesn't need an equivalent because type parameters don't affect struct layout. Rust needs `PhantomData` to tell the compiler the type parameter is intentional.
5. **Runtime alternatives exist in both** — OCaml can use variants, Rust can use enums. But phantom types catch errors at compile time with zero runtime cost.

## When to Use Each Style

**Use phantom types when:** State transitions must be enforced at compile time — connection states, protocol phases, resource lifecycles. The errors become impossible, not just caught.
**Use runtime enums when:** States are dynamic or determined by external input, and you can't know the state at compile time.
