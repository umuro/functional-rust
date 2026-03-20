📖 **[View on hightechmind.io →](https://hightechmind.io/rust/738-phantom-type-basics)**

---

# 738-phantom-type-basics — Phantom Type Basics

## Problem Statement

Sometimes you need a type to carry extra compile-time information that has no runtime representation. A `UserId` and a `ProductId` are both `u64`, but mixing them up is a logic error. Phantom types solve this: `Tagged<u64, UserTag>` and `Tagged<u64, ProductTag>` are different types at compile time but identical at runtime. This pattern prevents unit confusion (meters vs. feet), validates data provenance (raw vs. validated input), and creates marker-based permission systems — all at zero runtime cost.

## Learning Outcomes

- Use `PhantomData<Tag>` to carry type-level information without runtime overhead
- Create a `Tagged<T, Tag>` wrapper that makes distinct "branded" types from the same value type
- Model validation state with `Validated` and `Unvalidated` markers on `UserId`
- Understand why `PhantomData` is necessary for the compiler to accept unused type parameters
- See how phantom types prevent mixing IDs of different domain entities

## Rust Application

`Tagged<T, Tag>` wraps a `T` value with `PhantomData<Tag>`. `UserId<Unvalidated>` exposes only `new` and `validate`; `UserId<Validated>` exposes `get`. The `validate` method consumes the unvalidated ID and optionally returns the validated form, enforcing that only positive IDs can be validated. No runtime cost: `PhantomData<Tag>` is a zero-sized type.

## OCaml Approach

OCaml phantom types use type variables in a similar position: `type ('state) user_id = UserId of int64`. Modules provide encapsulation: only the module that implements `validate` can construct `Validated user_id`. OCaml 5 adds `[@@unboxed]` to eliminate even the boxing overhead. Jane Street's `Id` module uses this exact pattern for all entity IDs in their trading systems.

## Key Differences

1. **Syntax**: Rust requires `PhantomData<Tag>` as an explicit field; OCaml's phantom variables appear naturally in type signatures without a dummy field.
2. **Encapsulation**: Both use module/crate boundaries to prevent construction of phantom-typed values without going through the designated constructor.
3. **Multiple phantoms**: Rust can combine multiple phantom parameters in a tuple `PhantomData<(A, B)>`; OCaml uses multiple type variables directly.
4. **Performance**: Both are zero-cost — `PhantomData<T>` is zero bytes; OCaml's phantom variables add no runtime overhead.

## Exercises

1. Create `ProductId<State>` and `OrderId<State>` phantom types and write a function `create_order(user: UserId<Validated>, product: ProductId<Validated>) -> OrderId<Unvalidated>`.
2. Add a `Sanitized` marker and a `sanitize(raw: Tagged<String, Raw>) -> Tagged<String, Sanitized>` function that strips HTML tags. Ensure `render` only accepts `Tagged<String, Sanitized>`.
3. Implement a `TypeMap<Tag, V>` that stores values keyed by phantom-tagged keys, preventing retrieval with the wrong tag type.
