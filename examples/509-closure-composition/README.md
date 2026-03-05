📖 **[View on hightechmind.io →](https://hightechmind.io/rust/509-closure-composition)**

---

# 509: Closure Composition

**Difficulty:** 3  **Level:** Intermediate

Build complex transformations from simple pieces by chaining functions: `compose(f, g)(x) = f(g(x))`.

## The Problem This Solves

You have three transformations: double a number, add 1, then square it. Without composition, you write one big function `|x| (x * 2 + 1).pow(2)` — or worse, nested calls `square(add1(double(x)))` that read right-to-left and obscure the order of operations.

As pipelines grow, so does the cognitive load of tracking what happens when. You want to name sub-transformations, test them independently, and combine them in different orders for different use cases. Composition lets you treat functions as building blocks.

The deeper issue: as business logic grows, you want to express it as a sequence of named steps. Composition gives you this without inventing a framework — just functions combining functions.

## The Intuition

Function composition is math's `∘` operator: `(f ∘ g)(x) = f(g(x))`. It's a right-to-left combinator. The pipe version is left-to-right: `pipe(f, g)(x) = g(f(x))` — which reads like English: "first do f, then g."

Python doesn't have built-in composition, but you'd write `compose = lambda f, g: lambda x: f(g(x))`. JavaScript: `const compose = (f, g) => x => f(g(x))`. In OCaml, `|>` pipes left-to-right and `@@` applies right-to-left. Rust provides neither natively but lets you build them.

The result of composition is just another closure. Composing closures has zero runtime overhead — the compiler inlines the whole chain.

## How It Works in Rust

```rust
// compose: mathematical notation — right-to-left (g applied first, then f)
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(B) -> C,
    G: Fn(A) -> B,
{
    move |x| f(g(x))   // captures both f and g by move
}

// pipe: data-flow notation — left-to-right (f applied first, then g)
fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

let double = |x: i32| x * 2;
let inc    = |x: i32| x + 1;
let square = |x: i32| x * x;

// compose(inc, double)(5) = inc(double(5)) = 11
let double_then_inc = compose(inc, double);
println!("{}", double_then_inc(5)); // 11

// pipe reads left-to-right: double(5)=10, inc(10)=11, square(11)=121
let process = pipe(pipe(double, inc), square);
println!("{}", process(5)); // 121

// Builder pattern for multi-step pipelines
struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}
impl<T: 'static> Pipeline<T> {
    fn new() -> Self { Pipeline { steps: Vec::new() } }
    fn then(mut self, f: impl Fn(T) -> T + 'static) -> Self {
        self.steps.push(Box::new(f));
        self
    }
    fn run(self) -> impl Fn(T) -> T {
        move |x| self.steps.iter().fold(x, |acc, f| f(acc))
    }
}

let pipeline = Pipeline::new()
    .then(|x: i32| x * 2)  // step 1
    .then(|x| x + 1)        // step 2
    .then(|x| x * x)        // step 3
    .run();
println!("{}", pipeline(3)); // ((3*2)+1)^2 = 49
```

## What This Unlocks

- **Named transformation stages** — define `normalize`, `validate`, `format` as closures; compose them differently for different endpoints.
- **Testable sub-transforms** — each composed step is an independent closure you can unit-test before combining.
- **Middleware pipelines** — HTTP middleware, data processing stages, and event transformers all follow the composition pattern.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compose operator | Custom `( >> )` or `( << )` | Custom `compose()` function |
| Mathematical `f ∘ g` | `let compose f g x = f (g x)` | `fn compose<...>(f, g) -> impl Fn` |
| Pipe forward | `\|>` built-in operator | `.pipe(f)` via extension trait or custom `pipe()` |
| Type inference | Polymorphic — `'a -> 'b` chain natural | Generic bounds — more verbose but explicit |
| Point-free style | Natural — `let h = f \|> g` | Possible but needs wrapper functions |
