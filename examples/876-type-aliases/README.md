📖 **[View on hightechmind.io →](https://hightechmind.io/rust/876-type-aliases)**

---

# 876-type-aliases — Type Aliases
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Long or repetitive type signatures hurt readability. `HashMap<String, Vec<(usize, f64)>>` is tedious to write repeatedly, and `Box<dyn Fn(&str) -> Result<i32, ParseError>>` is worse. Type aliases provide a way to name complex types for clarity and consistency. The key tradeoff: unlike newtypes, aliases are transparent to the type checker — `UserId` and `u64` are the same type. This makes aliases appropriate for documentation and convenience (not safety). Both Rust and OCaml use the `type` keyword for aliases, though OCaml's aliases can also be parameterized with additional constraints via module types.

## Learning Outcomes

- Define simple and parameterized type aliases using the `type` keyword
- Understand that type aliases are transparent — no new type is created
- Use aliases to clarify complex generic signatures like `Result<T, ParseError>`
- Contrast type aliases with newtypes for documentation vs safety purposes
- Recognize the common alias pattern `type Result<T> = std::result::Result<T, MyError>`

## Rust Application

The code shows `type UserId = u64` (simple alias), `type Point = (f64, f64)` (tuple alias for geometry), `type Polygon = Vec<Point>` (composition of aliases), and `type AppResult<T> = Result<T, String>` (generic alias simplifying error types). The `demonstrate_alias_transparency` function confirms that `UserId` and `u64` are interchangeable — no cast needed. `type Predicate<T> = Box<dyn Fn(&T) -> bool>` shows aliasing complex function types. `type Validator<T> = fn(&T) -> bool` shows the simpler function pointer variant.

## OCaml Approach

OCaml uses the same `type` keyword: `type user_id = int`, `type point = float * float`. Parameterized aliases: `type 'a validator = 'a -> bool`, `type ('a, 'b) transform = 'a -> 'b`. Like Rust, OCaml aliases are fully transparent — `user_id` and `int` unify without coercion. OCaml's module system allows `type t = MyModule.t` to import a type from another module, which is a common idiom for adapters and wrappers.

## Key Differences

1. **Transparency**: Both languages make aliases completely transparent — no implicit conversion, no runtime cost, and no new type is created.
2. **Safety**: Type aliases provide zero type safety compared to newtypes/abstract types; they are documentation only.
3. **Generic syntax**: Rust uses `type Foo<T> = Bar<T>`; OCaml uses `type 'a foo = 'a bar`.
4. **Module type aliases**: OCaml can alias module types (`module type S = OtherModule.S`); Rust has no direct equivalent at the module level.

## Exercises

1. Define `type Matrix = Vec<Vec<f64>>` and write `add_matrices`, `scale_matrix`, and `transpose` functions using this alias.
2. Create `type Parser<T> = Box<dyn Fn(&str) -> Result<T, String>>` and implement a combinator `and_then_parser` that chains two parsers.
3. Refactor any earlier example that uses a long type signature to use an alias, and explain whether a newtype would be more appropriate.
