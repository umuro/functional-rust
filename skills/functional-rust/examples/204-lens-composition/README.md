# 204: Lens Composition — Zoom Into Nested Structs

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

Compose two Lenses into one: reach any depth in a nested struct with a single accessor.

## The Problem This Solves

You have a Lens from `Person → Address` and another from `Address → Street`. Now you want to read or update the street number on a `Person`. Without composition, you'd do it manually:

```rust
let addr = person_to_address.get(&person);
let street = address_to_street.get(&addr);
let number = street_to_number.get(&street);
// And for set:
let new_street = Street { number: 99, ..street };
let new_addr = Address { street: new_street, ..addr };
let new_person = Person { address: new_addr, ..person.clone() };
```

That's seven lines to change one field through two levels of nesting. You're hand-assembling what should be automatic.

The power of composition is that lenses **snap together**: `A → B` composed with `B → C` gives you `A → C`. The get functions chain left-to-right; the set functions chain right-to-left. Once composed, the resulting Lens is indistinguishable from a Lens that was written directly. And composition is **associative**: `(A composed B) composed C` equals `A composed (B composed C)` — you can group the steps any way you want and get the same result. This example exists to solve exactly that pain.

## The Intuition

Imagine you have two transparent pipes. The first goes from your hand to a box, the second goes from inside that box to a smaller box inside it. Composition is just joining the two pipes end-to-end: now you have one pipe from your hand straight to the innermost box. You don't have to think about the intermediate box at all.

In code terms:
- **Composed get**: `|person| street_get(&address_get(person))`  — follow the chain inward
- **Composed set**: when setting, get the intermediate value, set inside it, then set the result back  
  `|number, person| address_set(street_set(number, address_get(person)), person)`

The outer Lens doesn't need to know what `B` is internally. The inner Lens doesn't need to know anything about `A`. They just need to agree on the type `B` at their junction.

## How It Works in Rust

**Three one-level Lenses:**

```rust
fn address_l() -> Lens<Person, Address> {
    Lens::new(
        |p| p.address.clone(),
        |a, p| Person { address: a, ..p.clone() },
    )
}

fn street_l() -> Lens<Address, Street> {
    Lens::new(
        |a| a.street.clone(),
        |s, a| Address { street: s, ..a.clone() },
    )
}

fn number_l() -> Lens<Street, u32> {
    Lens::new(|s| s.number, |n, s| Street { number: n, ..s.clone() })
}
```

**Compose them — the `compose` method:**

```rust
impl<S: 'static, A: 'static> Lens<S, A> {
    fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where A: Clone {
        let og = self.get;   // outer get:  S -> A
        let os = self.set;   // outer set:  (A, S) -> S
        let ig = inner.get;  // inner get:  A -> B
        let is = inner.set;  // inner set:  (B, A) -> A
        Lens {
            get: Box::new(move |s| (ig)(&(og)(s))),    // S -> A -> B
            set: Box::new(move |b, s| {
                let a = (og)(s);                        // extract A from S
                let a2 = (is)(b, &a);                  // set B inside A -> new A
                (os)(a2, s)                             // set new A inside S
            }),
        }
    }
}
```

**Use the composed Lens:**

```rust
// Compose once
let person_number = address_l()
    .compose(street_l())
    .compose(number_l());   // Lens<Person, u32>

// Read — all the way from Person to u32
let n = (person_number.get)(&alice);    // 42

// Write — one line, three levels deep
let alice2 = (person_number.set)(99, &alice);
assert_eq!(alice2.address.street.number, 99);
assert_eq!(alice2.address.city, "Springfield");  // unchanged

// Modify (transform in place)
let alice3 = person_number.over(|n| n + 1, &alice);
```

**Macro syntax for longer chains:**

```rust
macro_rules! compose_lenses {
    ($l:expr) => { $l };
    ($l:expr, $($rest:expr),+) => { $l.compose(compose_lenses!($($rest),+)) };
}

let street_name = compose_lenses!(address_l(), street_l(), name_l());
```

## What This Unlocks

- **N-level nesting, one-line access**: no matter how deep the struct, a composed Lens gives you a single `get` and `set` — no manual intermediate extraction.
- **Reusable building blocks**: define one Lens per field, combine them in any order. A Lens from `A→B` can be composed into multiple paths — you're not writing one function per *path*, just one function per *step*.
- **Associativity guarantees correctness**: you can compose left-to-right or right-to-left and always get the same result — no need to worry about grouping order.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Composition syntax | Infix `\|>>` operator: `a_l \|>> b_l \|>> c_l` | Method chain: `a_l().compose(b_l()).compose(c_l())` |
| Intermediate allocation | Closures, GC managed | `Box<dyn Fn>` — each composition allocates two closures |
| Macro sugar | `[%lens address.street.number]` in ppx | Custom `compose_lenses!` macro |
| Associativity | Holds by construction | Holds by construction — same closure semantics |
| Reuse of composed lens | Closures capture by value | `.compose()` consumes `self` — store the result, or rebuild |
