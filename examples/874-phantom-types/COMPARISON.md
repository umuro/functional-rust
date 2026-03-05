# Comparison: Phantom Types

## Units of Measure

**OCaml:**
```ocaml
type meters
type seconds
type 'a quantity = { value : float }

let meters v : meters quantity = { value = v }
let seconds v : seconds quantity = { value = v }

let add_same (a : 'a quantity) (b : 'a quantity) : 'a quantity =
  { value = a.value +. b.value }
```

**Rust:**
```rust
struct Meters;
struct Seconds;

struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> std::ops::Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Quantity::new(self.value + rhs.value) }
}
```

## State Machine

**OCaml:**
```ocaml
type unlocked
type locked
type 'state door = { name : string }

let lock (d : unlocked door) : locked door = { name = d.name }
let walk_through (d : unlocked door) = Printf.sprintf "Walked through %s" d.name
```

**Rust:**
```rust
struct Door<State> { name: String, _state: PhantomData<State> }

impl Door<Unlocked> {
    fn lock(self) -> Door<Locked> { Door { name: self.name, _state: PhantomData } }
    fn walk_through(&self) -> String { format!("Walked through {}", self.name) }
}
// Door<Locked> has no walk_through — won't compile!
```

## Validated Data

**OCaml:**
```ocaml
type 'a email = Email of string
let validate_email (Email s : unvalidated email) : validated email option =
  if String.contains s '@' then Some (Email s) else None
let send_email (Email s : validated email) = Printf.sprintf "Sent to %s" s
```

**Rust:**
```rust
impl Email<Unvalidated> {
    fn validate(self) -> Result<Email<Validated>, String> { /* ... */ }
}
impl Email<Validated> {
    fn send(&self) -> String { format!("Sent to {}", self.address) }
}
```
