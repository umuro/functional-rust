📖 **[View on hightechmind.io →](https://hightechmind.io/rust/395-default-methods)**

---

# 395: Default Methods in Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Trait evolution is a challenge: adding a new required method to a published trait breaks all existing implementors. Default methods (introduced in Rust to solve this, analogous to Java 8's default interface methods) allow traits to provide method implementations that implementors can use or override. This enables adding new functionality to traits without breaking the ecosystem and reduces the "implement 20 methods just to satisfy a trait" problem by providing sensible defaults for derived functionality.

Default methods appear in `Iterator` (where only `next` is required but 70+ adapter methods have defaults), `Read`/`Write` in `std::io`, and virtually every non-trivial trait in `std`.

## Learning Outcomes

- Understand how default methods reduce the implementation burden for trait consumers
- Learn how implementations can override defaults for customized behavior
- See how default methods can call other (potentially non-default) methods in the same trait
- Understand the design pattern: minimal required interface + maximal default interface
- Learn how `Robot` overrides `greeting()` while inheriting the `formal_greeting()` default

## Rust Application

In `src/lib.rs`, `trait Greeter` requires only `fn name(&self) -> &str` but provides defaults for `greeting` and `formal_greeting` that both call `self.name()`. `Person` implements only the required `name` method and inherits both defaults. `Robot` overrides `greeting` with a custom message but uses the default `formal_greeting`. This is the minimal interface + default derivation pattern.

## OCaml Approach

OCaml achieves default methods through module functors: `module MakeGreeter (T : sig type t val name : t -> string end) = struct let greeting t = "Hello, " ^ T.name t ^ "!" end`. Implementors apply the functor to get the defaults. OCaml's class methods can also provide defaults via `method greeting = Printf.sprintf "Hello, %s!" self#name`. Both approaches parallel Rust's defaults.

## Key Differences

1. **Override mechanism**: Rust defaults are overridden by providing the method in the `impl` block; OCaml class defaults are overridden with `method! greeting = ...` or functor re-application.
2. **Required vs. optional**: Rust explicitly distinguishes required methods (no body) from default methods (with body); OCaml functors make all module members required in the parameter.
3. **Self access**: Rust default methods access `self` directly; OCaml class methods use `self#method_name`, functor defaults use the passed module value.
4. **Stability**: Adding a default method to a Rust trait is backward compatible; removing a default is breaking. OCaml functor additions are always breaking since they change the parameter signature.

## Exercises

1. **Iterator-like trait**: Design a `Stream` trait with a required `fn next(&mut self) -> Option<i32>` and default methods `fn collect_all(&mut self) -> Vec<i32>`, `fn take_n(&mut self, n: usize) -> Vec<i32>`, and `fn sum_all(&mut self) -> i32`. Implement for a `RangeStream` and a `FibStream`.
2. **Builder defaults**: Create a `Builder` trait with required `fn name(&self) -> &str` and defaults `fn build_json(&self) -> String` (JSON template) and `fn build_toml(&self) -> String` (TOML template). Two different structs should use the same defaults.
3. **Override verification**: Extend `Greeter` with a `fn loud_greeting(&self) -> String` default that calls `self.greeting().to_uppercase()`. Verify that when `Robot` overrides `greeting`, the `loud_greeting` default automatically uses the overridden version.
