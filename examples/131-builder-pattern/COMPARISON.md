# OCaml vs Rust: Builder Pattern with Typestate

## Side-by-Side Code

### OCaml
```ocaml
(* Phantom types encode which fields have been set *)
type unset = Unset_t
type set = Set_t

type ('name, 'email) user_builder = {
  name : string option;
  email : string option;
  age : int option;
}

let empty_builder : (unset, unset) user_builder =
  { name = None; email = None; age = None }

(* set_name transitions the first phantom from unset to set *)
let set_name name (b : (unset, 'e) user_builder) : (set, 'e) user_builder =
  { b with name = Some name }

let set_email email (b : ('n, unset) user_builder) : ('n, set) user_builder =
  { b with email = Some email }

let set_age age b = { b with age = Some age }

type user = { user_name : string; user_email : string; user_age : int option }

(* build only accepts (set, set) — rejected for (unset, _) or (_, unset) *)
let build (b : (set, set) user_builder) : user =
  { user_name = Option.get b.name;
    user_email = Option.get b.email;
    user_age = b.age }
```

### Rust (idiomatic — phantom type parameters on a struct)
```rust
use std::marker::PhantomData;

pub struct Missing;
pub struct Present;

pub struct UserBuilder<N, E> {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    _phantom: PhantomData<(N, E)>,
}

// .name() available only when N = Missing; transitions to Present
impl<E> UserBuilder<Missing, E> {
    pub fn name(self, name: &str) -> UserBuilder<Present, E> {
        UserBuilder { name: Some(name.to_string()), ..self.into_next() }
    }
}

// .build() available only when both N = Present and E = Present
impl UserBuilder<Present, Present> {
    pub fn build(self) -> User {
        User {
            name: self.name.unwrap(),
            email: self.email.unwrap(),
            age: self.age,
        }
    }
}
```

### Rust (functional — shows the field-by-field transition chain)
```rust
// Usage — each call changes the phantom type, enforced at compile time:
let user = UserBuilder::new()   // UserBuilder<Missing, Missing>
    .name("Alice")              // UserBuilder<Present, Missing>
    .email("alice@example.com") // UserBuilder<Present, Present>
    .age(30)                    // UserBuilder<Present, Present> (unchanged)
    .build();                   // User — only legal because both are Present

// This does NOT compile — no .build() on UserBuilder<Present, Missing>:
// let _ = UserBuilder::new().name("Alice").build();
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Missing-field marker | `type unset = Unset_t` | `pub struct Missing;` |
| Present-field marker | `type set = Set_t` | `pub struct Present;` |
| Builder type | `('name, 'email) user_builder` | `UserBuilder<N, E>` |
| Initial state | `(unset, unset) user_builder` | `UserBuilder<Missing, Missing>` |
| After setting name | `(set, 'e) user_builder` | `UserBuilder<Present, E>` |
| Build constraint | `(set, set) user_builder` | `impl UserBuilder<Present, Present>` |
| Phantom runtime cost | Zero (type-erased) | Zero (`PhantomData` is zero-sized) |

## Key Insights

1. **Phantom types in OCaml are polymorphic type parameters.** The record
   `('name, 'email) user_builder` carries `'name` and `'email` only in
   phantom position — they never appear in the fields themselves, but the
   compiler tracks them. Rust achieves the same with `PhantomData<(N, E)>`.

2. **Transitions are function signatures.** In OCaml, `set_name` takes
   `(unset, 'e) user_builder` and returns `(set, 'e) user_builder`. In Rust,
   each setter is in a specific `impl<...>` block: `impl<E> UserBuilder<Missing, E>`
   means the method is only callable when the name slot is `Missing`, and
   it returns a builder with `Present` in that slot.

3. **`build()` is gated by impl specialisation.** OCaml restricts `build`
   by its argument type `(set, set) user_builder`. Rust restricts it by
   implementing `build` only on `impl UserBuilder<Present, Present>`. The
   compiler rejects calls to `build()` on any other combination.

4. **Order independence is free.** Because each setter is parameterised over
   the *other* field's state (`impl<E> UserBuilder<Missing, E>`), callers can
   provide required fields in any order. Both `name().email()` and
   `email().name()` produce `UserBuilder<Present, Present>`.

5. **Zero runtime cost.** In both languages the phantom types are erased
   before runtime. In Rust, `PhantomData` has size zero and is compiled away
   entirely. The builder struct is exactly as large as its concrete fields.

## When to Use Each Style

**Use the typestate builder when:** you have a struct with multiple required
fields and want the compiler — not runtime assertions — to guarantee that
callers cannot forget them. The API becomes self-documenting: missing a
required field is a type error, not a `Result::Err` or a panic.

**Use a plain mutable builder with `Result<T, E>` from `build()` when:**
the set of required vs. optional fields is dynamic, or when you have so many
required fields that the combinatorial explosion of phantom parameters becomes
unmanageable (> 4–5 parameters). At that point, consider a proc-macro crate
such as `typed-builder` which generates the typestate boilerplate for you.
