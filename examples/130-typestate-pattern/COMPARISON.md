# OCaml vs Rust: Typestate Pattern

## Side-by-Side Code

### OCaml (GADT-based state machine)
```ocaml
type open_state = Open_s
type closed_state = Closed_s
type locked_state = Locked_s

type _ door =
  | OpenDoor  : open_state door
  | ClosedDoor  : closed_state door
  | LockedDoor  : locked_state door

let close_door : open_state door -> closed_state door = fun _ -> ClosedDoor
let open_door  : closed_state door -> open_state door = fun _ -> OpenDoor
let lock_door  : closed_state door -> locked_state door = fun _ -> LockedDoor
let unlock_door : locked_state door -> closed_state door = fun _ -> ClosedDoor
```

### Rust (idiomatic — phantom type parameters)
```rust
use std::marker::PhantomData;

pub struct Open;
pub struct Closed;
pub struct Locked;

pub struct Door<State> {
    pub material: String,
    _state: PhantomData<State>,
}

impl Door<Open> {
    pub fn close(self) -> Door<Closed> {
        Door { material: self.material, _state: PhantomData }
    }
    pub fn walk_through(&self) -> String {
        format!("Walking through {} door", self.material)
    }
}

impl Door<Closed> {
    pub fn open(self) -> Door<Open> {
        Door { material: self.material, _state: PhantomData }
    }
    pub fn lock(self) -> Door<Locked> {
        Door { material: self.material, _state: PhantomData }
    }
}

impl Door<Locked> {
    pub fn unlock(self) -> Door<Closed> {
        Door { material: self.material, _state: PhantomData }
    }
}
```

### Rust (builder typestate — URL required before send)
```rust
pub struct NoUrl;
pub struct HasUrl;

pub struct HttpRequest<UrlState> {
    url: Option<String>,
    body: Option<String>,
    _state: PhantomData<UrlState>,
}

impl HttpRequest<NoUrl> {
    pub fn url(self, url: &str) -> HttpRequest<HasUrl> {
        HttpRequest { url: Some(url.to_string()), body: self.body, _state: PhantomData }
    }
}

impl HttpRequest<HasUrl> {
    pub fn send(self) -> String { /* guaranteed to have url */ }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| State marker | `type open_state = Open_s` | `pub struct Open;` |
| Parameterised container | `type _ door = ...` (GADT) | `struct Door<State>` |
| Phantom field | implicit in GADT | `_state: PhantomData<State>` |
| Transition function | `close_door : open_state door -> closed_state door` | `fn close(self) -> Door<Closed>` |
| Method availability | constrained by GADT constructor | constrained by `impl Door<Open>` |

## Key Insights

1. **GADT vs phantom generics**: OCaml uses GADTs to index a single type by a phantom state type; Rust uses a generic parameter and `PhantomData` to achieve the same effect without GADTs.
2. **Method-level enforcement**: Rust's `impl` blocks are per concrete instantiation (`impl Door<Open>`), so `close()` literally does not exist on `Door<Closed>` — no trait, no method, the compiler cannot even see it.
3. **Zero runtime cost**: Both approaches are zero-cost. OCaml's GADT state values are erased; Rust's `PhantomData<State>` is a zero-sized type with no memory footprint.
4. **Consuming transitions**: Rust transitions consume `self` by value, making it impossible to hold a reference to the "old" state after the transition — a stronger guarantee than OCaml's functional style.
5. **Builder pattern synergy**: The typestate pattern composes naturally with the builder pattern in Rust, letting you enforce that required fields are set (e.g., a URL) before certain operations (e.g., `send()`) are even callable.

## When to Use Each Style

**Use idiomatic Rust phantom generics when:** you need compile-time protocol enforcement with zero runtime cost — API clients, resource lifecycle management (open/close/lock), builder patterns where some fields must precede others.

**Use OCaml GADTs when:** you need to work with heterogeneous collections of state-indexed values in a single algebraic type, or when you want exhaustive pattern matching across all states in one `match` expression.
