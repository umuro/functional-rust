📖 **[View on hightechmind.io →](https://hightechmind.io/rust/532-lifetime-multiple)**

---

# 532: Multiple Lifetime Parameters

**Difficulty:** 4  **Level:** Intermediate-Advanced

Use separate lifetime labels when two inputs have different valid scopes and the output only borrows from one of them.

## The Problem This Solves

A single `'a` ties all annotated references to the *same* scope — meaning if you annotate both inputs and the output with `'a`, the compiler treats the output as valid for the shorter of the two inputs. That's conservative and sometimes wrong.

```rust
// BAD: both inputs share 'a — compiler infers output valid for min(s1, s2)
fn first_of<'a>(x: &'a str, y: &'a str) -> &'a str { x }

let long = String::from("long-lived");
let result;
{
    let short = String::from("short");
    result = first_of(&long, &short); // output appears tied to 'short' too!
}
println!("{}", result); // compiler may reject this — even though result only borrows `long`
```

With a second lifetime parameter, you express the precise relationship: the output depends on `x`, not `y`. The caller can then let `y` die without affecting `result`.

## The Intuition

Every `'a`, `'b`, `'c` is a separate label — like naming different regions on a map. When you write `'a, 'b`, you're saying "x and y come from different regions; I'm not claiming they expire at the same time."

The output annotation tells the compiler *which region* the return value points into. If you return `x`, the output's lifetime label should match `x`'s label. `y`'s lifetime becomes irrelevant to the output's validity.

Lifetime bounds (`'a: 'b`, read "'a outlives 'b") let you express ordering when one region must contain another.

## How It Works in Rust

**Single lifetime — overly restrictive:**

```rust
fn first_of<'a>(x: &'a str, y: &'a str) -> &'a str { x }
// Compiler unifies 'a to min(x's lifetime, y's lifetime)
// Even though we only return x, y constrains the result
```

**Two lifetimes — precise:**

```rust
fn first_of<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    //              ^^           ^^   ^^ return from 'a region only
    x // output tied to 'a — 'b doesn't constrain it
}

// Now this works:
let long = String::from("long-lived");
let result;
{
    let short = String::from("short"); // 'b region
    result = first_of(&long, &short);  // 'a = long's scope, 'b = short's scope
}                                      // short ('b) drops — but result only borrows 'a!
println!("{}", result);                // fine
```

**Struct with two independent borrowed fields:**

```rust
struct Pair<'a, 'b> {
    first: &'a str,   // borrows from one source
    second: &'b str,  // borrows from a different source — independent lifetimes
}

impl<'a, 'b> Pair<'a, 'b> {
    fn get_first(&self) -> &'a str { self.first }   // tied to 'a, not self
    fn get_second(&self) -> &'b str { self.second } // tied to 'b, not self
}
```

**Lifetime bounds — expressing ordering:**

```rust
// 'long: 'short means 'long must outlive 'short
fn use_while_valid<'long, 'short>(item: &'long str, _temp: &'short str) -> &'long str
where
    'long: 'short,
{
    item
}
```

## What This Unlocks

- **Returning one of two inputs** — when you only return from the first argument, you don't want the second argument's lifetime to restrict the caller's use of the result.
- **Structs with fields from different sources** — a `ParseResult` that borrows both the parsed value and the remaining input can have independent lifetimes for each field.
- **Lifetime bounds in trait implementations** — express that a type's lifetime must outlive a constraint without merging them.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Multiple GC roots | GC tracks all values automatically | Multiple lifetime params explicitly name scope relationships |
| Return from one of two args | Type system doesn't track which source | Different `'a`, `'b` params declare exactly which source the output comes from |
| Independent borrows | Any number of references anywhere | Multiple `'a, 'b` params let different borrows be independent |
| Lifetime bounds | N/A | `'a: 'b` syntax expresses "outlives" relationships |
| Inference | Full type inference everywhere | Lifetimes usually inferred; explicit needed when output source is ambiguous |
