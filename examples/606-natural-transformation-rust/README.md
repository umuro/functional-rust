📖 **[View on hightechmind.io →](https://hightechmind.io/rust/606-natural-transformation-rust)**

---

# 606: Natural Transformations in Rust

**Difficulty:** 5  **Level:** Master

A practical toolkit of container-to-container conversions in idiomatic Rust, with automated verification that each one preserves structure correctly.

## The Problem This Solves

Every Rust codebase accumulates a collection of conversions between container types: `Option` to `Result`, `Result` to `Option`, `Vec` to `Option` (take the first element), `Option` to `Vec`. These get written inline, copied, or reimplemented slightly differently each time they're needed.

Worse, it's easy to write a conversion that's subtly wrong — one that behaves differently depending on what's *inside* the container. A conversion from `Option<i32>` that peeks at the value and returns `None` for negative numbers is NOT the same kind of thing as a conversion that just rewraps the container. The first is a filter; the second is a natural transformation. Conflating them leads to buggy data pipelines.

Natural transformations are the conversions that are "honest" — they convert the container shape without ever looking at the contents. The key property they satisfy is that it doesn't matter whether you map over the values first and then convert, or convert first and then map — the results are identical. This is the **naturality condition**, and this example verifies it automatically for each conversion.

The payoff: a library of composable, verified container conversions that can be chained confidently. This exists to solve exactly that pain.

## The Intuition

You know what `Option<T>` and `Vec<T>` are. A **natural transformation** from `Option` to `Vec` is a rule for converting one to the other that never inspects `T`.

The rule: `None → []`, `Some(x) → [x]`. This rule works the same for `T = i32`, `T = String`, `T = MyStruct`. You never unwrap `x` to look at it. You just move it.

The naturality condition says this conversion "commutes" with any `map`. Concretely:

```
Some(5)  ──map(x*2)──▶  Some(10)
  │                         │
 η│                        η│   ← η = opt_to_vec (our natural transformation)
  ▼                         ▼
 [5]  ──map(x*2)──▶      [10]

Both paths arrive at the same answer. That's naturality.
```

Contrast with something that's NOT a natural transformation:

```rust
// NOT natural — peeks inside T, behavior depends on value
fn suspicious_convert(opt: Option<i32>) -> Vec<i32> {
    match opt {
        Some(x) if x > 0 => vec![x],  // inspects the value!
        _ => vec![],
    }
}
// naturality_check fails: suspicious_convert(Some(-5).map(|x| -x)) ≠
//                          suspicious_convert(Some(-5)).map(|x| -x)
```

Rust's parametric generics make natural transformations easy to identify: any function `fn<T>(Option<T>) -> Vec<T>` that compiles without trait bounds on `T` (other than what's needed for ownership) is automatically a natural transformation. The compiler prevents you from inspecting `T`.

## How It Works in Rust

```rust
// η: Option<A> -> Vec<A>
// None becomes empty list; Some(x) becomes single-element list
fn opt_to_vec<A>(o: Option<A>) -> Vec<A> {
    match o { Some(x) => vec![x], None => vec![] }
}

// η: Vec<A> -> Option<A>  (take first element, or None if empty)
fn vec_to_opt<A>(v: Vec<A>) -> Option<A> {
    v.into_iter().next()
}

// η: Result<A,E> -> Option<A>  (discard the error, keep success)
fn result_to_opt<A,E>(r: Result<A,E>) -> Option<A> { r.ok() }

// η: Option<A> -> Result<A, &'static str>
fn opt_to_result<A>(o: Option<A>) -> Result<A, &'static str> {
    o.ok_or("missing value")
}

// Automated naturality checker for Option -> Vec
// Verifies: opt_to_vec(opt.map(f)) == opt_to_vec(opt).map(f)
fn naturality_opt_to_vec<A: Clone + PartialEq, B: PartialEq>(
    opt: Option<A>,
    f: impl Fn(A) -> B,
) -> bool {
    // Left side: map first (inside Option), then convert to Vec
    let left: Vec<B> = opt_to_vec(opt.clone().map(&f));
    // Right side: convert to Vec first, then map
    let right: Vec<B> = opt_to_vec(opt).into_iter().map(f).collect();
    left == right  // must be equal for a natural transformation
}

fn main() {
    // Basic conversions
    println!("{:?}", opt_to_vec(Some(42)));    // [42]
    println!("{:?}", opt_to_vec(None::<i32>)); // []
    println!("{:?}", vec_to_opt(vec![1,2,3])); // Some(1)
    
    // Verify naturality automatically
    assert!(naturality_opt_to_vec(Some(5), |x: i32| x * 2)); // ✓
    assert!(naturality_opt_to_vec(None::<i32>, |x: i32| x * 2)); // ✓
    
    // Chain multiple natural transformations
    let r: Result<i32, &str> = Ok(42);
    let o: Option<i32> = result_to_opt(r);   // Result -> Option
    let v: Vec<i32>    = opt_to_vec(o);       // Option -> Vec
    println!("{:?}", v);  // [42]
}
```

The chain `Result -> Option -> Vec` works because each step is a natural transformation — you can reason about the whole pipeline by composing the individual steps, knowing the inner values are never touched.

## What This Unlocks

- **API boundary normalization:** Standardize on one conversion library for `Option ↔ Result ↔ Vec`. Everyone uses the same functions, so code reads uniformly and the naturality guarantee means no surprises when values pass through.
- **Composable data pipelines:** Chain natural transformations freely — `result_to_opt ∘ vec_to_opt ∘ parse_line` composes because each step is independently verified to be natural.
- **Property testing hook:** The `naturality_check` pattern generalizes to property-based testing (QuickCheck/proptest): generate random `opt` and `f`, assert naturality. Find bugs in conversions that "accidentally" inspect the value.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Natural transformation type | `type 'a nt = { apply: 'a . 'a F -> 'a G }` — rank-2 polymorphic record | Generic function `fn<A>(F<A>) -> G<A>`; no rank-2 types needed for basic cases |
| Polymorphism guarantee | Parametricity theorem: any polymorphic function is automatically natural | Same via parametric generics — compiler prevents inspecting `A` |
| Passing nat-transform as value | Single polymorphic value `nt` | Monomorphized per use; closures or function pointers |
| Naturality verification | Equational proofs or QuickCheck | Explicit `naturality_check` function or proptest |
| Composing transforms | `fun x -> g.apply (f.apply x)` | `fn composed(x) { g(f(x)) }` — just function composition |
| Examples | `List.of_option`, `Option.join`, `List.hd_opt` | `opt_to_vec`, `vec_to_opt`, `result_to_opt`, `Option::ok_or` |
