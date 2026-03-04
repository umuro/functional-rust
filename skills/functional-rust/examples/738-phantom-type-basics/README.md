# 738: PhantomData: Phantom Types and Type Markers

**Difficulty:** 4  **Level:** Expert

`PhantomData<T>` is zero bytes at runtime but makes the compiler treat a struct as if it owns or uses `T` — enabling typed IDs, capability tokens, and variance annotations with zero overhead.

## The Problem This Solves

Stringly-typed systems are fragile. A function `fn get_user(id: u64)` accepts any `u64` — you could accidentally pass an `order_id` and the compiler won't object. Runtime bugs from confusing IDs of different entity types are common in large codebases and expensive to track down.

The brute-force fix is newtype wrappers: `struct UserId(u64)` and `struct OrderId(u64)`. They're different types, but you still need to write a dozen `From`/`Into` impls if you want them to share methods. If you want `Id<User>` and `Id<Order>` to share the same generic `Id` implementation while remaining distinct types, you need `PhantomData`.

More broadly, `PhantomData` is the tool for encoding type-level information in a struct when that information has no runtime representation. Permission tokens, variance markers, lifetime attachments — all use `PhantomData` as the carrier.

## The Intuition

`PhantomData<T>` is a zero-sized type that tells the compiler "this struct logically involves `T`, even though no field actually holds a `T`." The compiler needs this for two reasons: variance analysis (how lifetimes flow through the type) and drop-check analysis (whether dropping the struct might drop a `T`).

For user code, the most common use is type-level tagging: `Id<User>` and `Id<Order>` are the same struct with the same `u64` field, but they're different types because their `Entity` type parameter differs. Assigning one to the other is a compile error. At runtime, both are just a `u64` — `PhantomData` adds exactly zero bytes.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Entity markers — zero-sized, exist only for type checking
pub struct User;
pub struct Order;

// Generic ID — same struct, different types depending on Entity
pub struct Id<Entity> {
    value:   u64,
    _entity: PhantomData<Entity>,  // 0 bytes, makes Id<User> ≠ Id<Order>
}

impl<Entity> Id<Entity> {
    pub fn new(value: u64) -> Self { Id { value, _entity: PhantomData } }
    pub fn value(&self) -> u64 { self.value }
}

pub type UserId  = Id<User>;
pub type OrderId = Id<Order>;

let user_id:  UserId  = Id::new(42);
let order_id: OrderId = Id::new(42);

// Same inner value, different types — compiler rejects mixing:
// let wrong: UserId = order_id;  // ERROR: expected Id<User>, got Id<Order>

// Size: just the u64 — PhantomData adds nothing
assert_eq!(std::mem::size_of::<UserId>(), std::mem::size_of::<u64>());  // 8 bytes

// ── Capability tokens ─────────────────────────────────────────────────────────
pub struct ReadPerm;
pub struct WritePerm;

pub struct Token<Perm> {
    id:    u32,
    _perm: PhantomData<Perm>,
}

// Functions only accept the right token type
fn read_resource(tok: &Token<ReadPerm>, name: &str) -> String { /* ... */ }
fn write_resource(tok: &Token<WritePerm>, name: &str, data: &str) -> String { /* ... */ }

let rt: Token<ReadPerm>  = Token::new(1);
let wt: Token<WritePerm> = Token::new(2);

read_resource(&rt, "config");        // fine
write_resource(&wt, "config", "v"); // fine

// read_resource(&wt, "x");          // ERROR: expected Token<ReadPerm>, got Token<WritePerm>
// write_resource(&rt, "x", "y");    // ERROR: same

// ── Key facts ─────────────────────────────────────────────────────────────────
assert_eq!(std::mem::size_of::<PhantomData<String>>(), 0);  // always zero
assert_eq!(std::mem::size_of::<PhantomData<Vec<u8>>>(), 0); // always zero
```

## What This Unlocks

- **Typed entity IDs** — `UserId`, `OrderId`, `ProductId` from one generic `Id<Entity>` struct; mixing them is a compile error with zero runtime cost.
- **Capability-safe tokens** — `Token<ReadPerm>` vs `Token<WritePerm>`; functions requiring specific permissions accept only the right token type.
- **Variance and drop-check control** — `PhantomData<T>` (covariant), `PhantomData<fn(T)>` (contravariant), `PhantomData<Cell<T>>` (invariant) — used when building safe abstractions over raw pointers.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom types | `type 'a t = T` — `'a` is phantom if unused in `T` | `struct Foo<T> { _t: PhantomData<T> }` — explicit phantom field |
| Zero-cost type tag | Phantom type variable (erased at runtime) | `PhantomData<T>` — zero bytes, erased at runtime |
| Typed IDs | `type user_id = int` — same underlying type, aliased | `Id<User>` vs `Id<Order>` — structurally different types; mixing is error |
| Capability tokens | Phantom type in module signature | `Token<ReadPerm>` vs `Token<WritePerm>` — separate generic instantiations |
