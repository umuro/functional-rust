# 202: Lens Basics — View, Set, and Over

**Difficulty:** ⭐⭐⭐  **Level:** Intermediate

A Lens bundles a getter and setter into one reusable value, and three operations — `view`, `set`, `over` — cover every access pattern you need.

## The Problem This Solves

You're writing a function that updates a field on a struct. You pass the struct in, return a new one. Straightforward. But now you need the same pattern for five different fields across three different structs. You end up writing fifteen near-identical functions: `update_name`, `update_age`, `update_address_city`, and so on.

This is brittle. When your struct gains a field, every update function might need touching. When you want to apply a transformation rather than a hard set — like "increment the age" — you add *another* fifteen functions: `increment_age`, `double_salary`, etc.

The root problem is that field access isn't first-class in Rust. You can't pass "the name field of Person" as a value — you have to write a function that hard-codes the field name. This prevents abstraction.

A Lens makes field access first-class. You define the getter and setter once, name the Lens, and then `view`, `set`, and `over` cover every operation without writing new functions. One Lens replaces a whole family of `update_*` helpers. This example exists to solve exactly that pain.

## The Intuition

There are three things you ever want to do with a field:

1. **Read it** — give me the value at this field (`view`)
2. **Replace it** — give me a new struct with this field changed to X (`set`)
3. **Transform it** — give me a new struct where this field has been run through function F (`over`)

`over` is not a bonus — it's the most useful of the three. "Increment the counter", "uppercase the name", "double the salary" are all `over`.

```
view  lens s       =>  A              (read the value)
set   lens a s     =>  S              (replace with a)
over  lens f s     =>  S              (transform with f)
```

`over` is just `set(f(view(s)), s)` — so only `get` and `set` are fundamental. But `over` is what you'll actually use.

The Lens itself is just two functions stored together:

```rust
struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,   // S = the struct, A = the field type
    set: Box<dyn Fn(A, &S) -> S>,
}
```

## How It Works in Rust

**Defining a Lens and using all three operations:**

```rust
fn name_lens() -> Lens<Person, String> {
    Lens::new(
        |p| p.name.clone(),                             // get
        |n, p| Person { name: n, ..p.clone() },         // set
    )
}

let alice = Person { name: "Alice".into(), age: 30 };
let nl = name_lens();

// view: read the field
assert_eq!(nl.view(&alice), "Alice");

// set: replace the field
let alice2 = nl.set("Alicia".into(), &alice);
assert_eq!(nl.view(&alice2), "Alicia");

// over: transform the field
let upper = nl.over(|n| n.to_uppercase(), &alice);
assert_eq!(nl.view(&upper), "ALICE");
```

**Trait-based Lenses (zero-cost, no heap allocation):**

The closure approach uses `Box<dyn Fn>` which allocates. For hot paths, use a trait instead:

```rust
trait LensLike<S, A> {
    fn get(s: &S) -> A;
    fn set(a: A, s: &S) -> S;
    fn over(f: impl FnOnce(A) -> A, s: &S) -> S {
        let a = Self::get(s);
        Self::set(f(a), s)
    }
}
```

Each Lens is now a zero-sized type — the compiler monomorphizes everything, no runtime dispatch, no allocation.

**Macro-generated Lenses (cut the boilerplate):**

```rust
make_lens!(PersonName, Person, name, String);
make_lens!(PersonAge,  Person, age,  u32);

// Now use directly as associated functions:
PersonName::get(&alice);                    // "Alice"
PersonAge::over(|a| a + 1, &alice);        // Person { age: 31, .. }
```

The macro generates the zero-sized struct and the `LensLike` impl — one line per field.

## What This Unlocks

- **Three operations cover everything**: `view`, `set`, `over` replace all your `get_*`/`update_*`/`transform_*` helper families.
- **First-class field access**: pass a Lens as a value, store it, return it, use it in generic code that doesn't know which struct or field it's working with.
- **Zero-cost when needed**: the trait-based encoding gives you the same abstraction with no runtime overhead — the compiler specializes it per type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Lens type | Record of two functions | Struct with `Box<dyn Fn>` or a trait |
| Zero-cost option | No (closures use GC heap) | Yes — trait-based, fully monomorphized |
| `over` / `modify` | Simple record combinator | Method on `Lens<S,A>` or trait default method |
| Boilerplate per field | Minimal — 2 lines | Macro helps; still more verbose |
| Polymorphic update | Implicit with parametric types | Requires explicit `Clone` bounds |
