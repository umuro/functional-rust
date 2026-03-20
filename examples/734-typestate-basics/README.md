📖 **[View on hightechmind.io →](https://hightechmind.io/rust/734-typestate-basics)**

---

# 734-typestate-basics — Typestate Basics
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

State machines are everywhere: network connections, file handles, protocol sessions, UI wizards. The standard approach encodes state as a runtime enum and adds `match` checks before every operation. This moves errors from compile time to runtime — you can write `socket.send(data)` on a closed socket and only discover the bug at runtime. The typestate pattern encodes state in the type parameter itself, making invalid transitions a compile error. Zero-cost at runtime: all phantom data is erased.

## Learning Outcomes

- Encode state as zero-sized marker types (`Red`, `Green`, `Yellow`)
- Use `PhantomData<State>` to carry type-level state without runtime overhead
- Implement state transitions by consuming `self` and returning a different type
- Understand why invalid transitions fail at compile time rather than runtime
- Verify that `size_of::<Light<Red>>()` equals `size_of::<Light<Green>>()` — zero overhead

## Rust Application

`Light<State>` wraps `PhantomData<State>`. Each impl block is bound to a specific state: `impl Light<Red>` exposes only `go()`, which consumes the `Red` light and returns `Light<Green>`. Calling `light.go().go()` would fail because `Light<Green>` does not implement `go()`. The only entry point is `Light::<Red>::new()`, enforcing that all lights start red. The whole pattern compiles to zero instructions beyond the enclosed data.

## OCaml Approach

OCaml achieves typestate via phantom types in the module system. A `('state) light` type carries a phantom type variable, and functors or abstract types prevent construction of invalid states. The classic technique uses empty types or abstract module types as state markers. OCaml's GADTs (generalized algebraic data types) provide an even more powerful mechanism for encoding state invariants directly in the type index.

## Key Differences

1. **Syntax**: Rust uses `impl Light<Red> { fn go(self) -> Light<Green> }`; OCaml uses phantom type variables and module signatures to restrict operations.
2. **GADTs**: OCaml's GADTs can encode complex state invariants that Rust's trait system cannot easily express without macros.
3. **Consumption**: Rust's move semantics make "consume self to transition" natural; OCaml's immutable values must use abstract types to prevent reuse of old states.
4. **Zero cost**: Both approaches are zero-cost at runtime — phantom types carry no runtime representation in either language.

## Exercises

1. Add a `blink()` method to `Light<Yellow>` that returns `Light<Yellow>` (self-transition), and verify it compiles without allowing `blink()` on other states.
2. Model a vending machine with states `Idle`, `ItemSelected`, `PaymentReceived`, and `Dispensing`. Enforce that payment can only happen after selection.
3. Write a `sequence` function `fn traffic_cycle(light: Light<Red>) -> Light<Red>` that goes through a full Red→Green→Yellow→Red cycle and returns the light to its starting state.
