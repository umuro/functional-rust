📖 **[View on hightechmind.io →](https://hightechmind.io/rust/615-optics-intro)**

---

# 615: Optics Intro — Lenses, Prisms, and the Hierarchy

**Difficulty:** 5  **Level:** Master

Optics are composable accessors for data structures. A Lens accesses a field that always exists; a Prism accesses a variant that might exist; together they form the foundation of a larger composable hierarchy.

## The Problem This Solves

You write code that reads and updates nested data structures. At every level, you choose between two bad options: write verbose nested struct-update expressions (`Event { location: Location { coords: Coords { lat: ..., ..e.location.coords }, ..e.location }, ..e }`) or write a proliferation of one-off helper functions that don't compose.

The deeper problem is that data access in most languages isn't **compositional**. You can't combine "get the city of an address" with "get the address of a person" to automatically get "get the city of a person's address" — you have to write that third function explicitly. And when you have many fields, many nesting levels, and both structs (product types) and enums (sum types), the explosion of accessor functions becomes unmanageable.

Optics solve this at the category level: they provide a unified abstraction where accessors compose automatically, work for both structs and enums, and guarantee correct behaviour through algebraic laws. This example exists to solve exactly that pain.

## The Intuition

**The hierarchy** — each level is a more powerful optic:

| Optic | Works on | Get returns | Set behaviour |
|-------|----------|-------------|---------------|
| **Lens** | Product types (structs) | `A` — always succeeds | Always replaces |
| **Prism** | Sum types (enums) | `Option<A>` — may fail | Only if variant matches |
| **Traversal** | Collections / multiple targets | Iterator of `A` | Applies to each element |

Think of them as drill bits of increasing flexibility:
- A **Lens** is a standard bit — it always reaches the target.
- A **Prism** is a conditional bit — it reaches the target only if the material is right.
- A **Traversal** is a multi-bit — it reaches multiple targets at once.

**Lenses** are for struct fields. Every `Person` has an `age`, so `age_lens.view(&person)` always returns a `u32`.

**Prisms** are for enum variants. Not every `Json` is a `JString`, so `jstring_prism.preview(&json)` returns `Option<String>`.

The key insight: **a Lens composed with a Prism gives you a Traversal** — you get a "get into a field that might exist", which is exactly a partial accessor. The composition rules are what make optics powerful: mix and match, and the type system tells you what you get.

## How It Works in Rust

**Lens — struct field access:**

```rust
struct Lens<S, A> {
    get: fn(&S) -> A,
    set: fn(A, S) -> S,
}

let name_lens: Lens<Person, String> = Lens::new(
    |p| p.name.clone(),
    |v, mut p| { p.name = v; p },
);

let city_lens: Lens<Address, String> = Lens::new(
    |a| a.city.clone(),
    |v, mut a| { a.city = v; a },
);

// Get city via two lenses
let addr = addr_lens.view(&person);
let city = city_lens.view(&addr);
```

Note: `fn` pointer versions (as in this example) avoid heap allocation but can't close over values. For composition that closes over state, use `Box<dyn Fn>` (shown in examples 201–205).

**Prism — enum variant access:**

```rust
struct Prism<S, A> {
    preview: fn(&S) -> Option<A>,   // may return None
    review:  fn(A) -> S,            // always succeeds
}

let some_prism: Prism<Option<i32>, i32> = Prism::new(
    |o| *o,           // Option<i32> already is Option<i32>
    |a| Some(a),
);

some_prism.preview(&Some(42))  // Some(42)
some_prism.preview(&None)      // None
some_prism.review(7)           // Some(7)
```

**Using `over` for transformation:**

```rust
let p2 = age_lens.over(|a| a + 1, person.clone());
// person is immutable; p2 has age incremented
```

## What This Unlocks

- **One mental model for all data access**: Lens, Prism, and Traversal cover struct fields, enum variants, and collections uniformly — the same `view`/`set`/`over` API applies across all three.
- **Composable by construction**: Lens composed with Prism gives a Traversal; Lens composed with Lens gives a Lens; the type system enforces the composition rules automatically.
- **Foundation for richer abstractions**: once you understand the optics hierarchy, libraries like `lens` or `lenses` crates — and the Van Laarhoven encoding that lets Haskell compose them all with simple function composition — become accessible.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | Record `{ get: 's -> 'a; set: 'a -> 's -> 's }` | `struct Lens<S,A>` with `fn` pointers or `Box<dyn Fn>` |
| Prism type | Record `{ preview: 's -> 'a option; review: 'a -> 's }` | `struct Prism<S,A>` with same signatures |
| Traversal | `'s -> 'a list` + `'a list -> 's -> 's` | `.iter().map()` + collect |
| Composition | Function composition (infix `@@`) | `.compose()` method or manual closure chaining |
| Iso | Bijection pair | `fn to()` / `fn from()` — bidirectional conversion |
