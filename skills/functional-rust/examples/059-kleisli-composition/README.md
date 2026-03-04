# 059: Kleisli Composition

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Compose fallible functions `(A → Option<B>)` and `(B → Option<C>)` into `(A → Option<C>)`, propagating `None` automatically — the pattern your `and_then` chains already implement, formalised.

## The Problem This Solves

You have a pipeline of functions where each step can fail: parse a string into an integer, check it's positive, check it's even. Normal function composition only works when functions return plain values. The moment a function returns `Option<B>`, you can no longer just compose it with the next step — you have to stop and unwrap first.

So you write this by hand every time:

```rust
// Every pipeline looks like this:
let result = match parse_int(s) {
    None => None,
    Some(n) => match check_positive(n) {
        None => None,
        Some(n2) => safe_half(n2),
    },
};
```

You can clean it up with `and_then`:

```rust
let result = parse_int(s)
    .and_then(check_positive)
    .and_then(safe_half);
```

That's better — but it's still manual wiring. Every time you build a new pipeline you write the same chain. You can't store "parse-then-validate" as a single reusable function without writing a wrapper. You can't build pipelines dynamically from a list of steps.

Kleisli composition solves exactly that pain.

## The Intuition

Normal function composition: if you have `f: A → B` and `g: B → C`, you can make a new function `h: A → C` by doing `f` first, then `g`. This is just `g(f(a))`.

Kleisli composition is the same idea, but for functions that return wrapped values:

- `f: A → Option<B>` — might fail, gives `None` or `Some(b)`
- `g: B → Option<C>` — might fail, given a `B`
- Kleisli composition: `h: A → Option<C>` — runs `f`, and *if it succeeds*, runs `g` on the result, propagating `None` automatically

The name sounds fancy, but the concept is one you already know: **it's `and_then`**. Kleisli just lets you express it as a composition operator so you can build pipelines from pieces.

Think of it like Unix pipes: `cat file | grep pattern | sort`. Each step passes its output to the next. If `grep` produces nothing, `sort` has nothing to sort. Kleisli composition is pipes for fallible computations.

```rust
// Kleisli compose two fallible functions into one:
fn kleisli<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    move |a| f(a).and_then(|b| g(b))
    //           ^^^^^^^^  That's the "propagate None" part
}
```

The `?` operator in Rust is Kleisli composition. When you write `let n = parse_int(s)?;`, you are saying: "if this returns `None`/`Err`, stop here and propagate; otherwise give me the inner value." That's exactly what Kleisli `>=>` does.

## How It Works in Rust

```rust
// Step 1: The composition function
// Takes two monadic functions, returns their composition
fn kleisli<A, B, C>(
    f: impl Fn(A) -> Option<B>,
    g: impl Fn(B) -> Option<C>,
) -> impl Fn(A) -> Option<C> {
    // Returns a closure that runs f, then g on the result
    // and_then handles the None propagation automatically
    move |a| f(a).and_then(|b| g(b))
}

// Step 2: Individual steps — each is a "fallible function"
fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()           // None if s isn't a number
}

fn check_positive(n: i32) -> Option<i32> {
    if n > 0 { Some(n) } else { None }
}

fn safe_half(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n / 2) } else { None }
}

// Step 3: Compose the pipeline — each composition is itself a fallible function
let validate = kleisli(
    kleisli(parse_int, check_positive),  // parse → check positive
    safe_half,                            // → halve (only if even)
);

// validate is now a single function: &str -> Option<i32>
validate("42");  // Some(21)  — parsed, positive, even → halved
validate("0");   // None      — parsed but not positive
validate("7");   // None      — parsed, positive, but not even
validate("bad"); // None      — couldn't parse

// Step 4: The same pattern works for Result (error messages instead of None)
fn kleisli_result<A, B, C, E>(
    f: impl Fn(A) -> Result<B, E>,
    g: impl Fn(B) -> Result<C, E>,
) -> impl Fn(A) -> Result<C, E> {
    move |a| f(a).and_then(|b| g(b))
}

// Step 5: Dynamic pipelines — build from a list of steps at runtime
fn pipeline(steps: &[fn(i32) -> Option<i32>], x: i32) -> Option<i32> {
    // fold: thread the value through each step,
    // short-circuiting if any returns None
    steps.iter().fold(Some(x), |acc, step| acc.and_then(step))
}

let steps: Vec<fn(i32) -> Option<i32>> = vec![check_positive, safe_half];
pipeline(&steps, 50);  // Some(25)
pipeline(&steps, -1);  // None — fails at check_positive
```

## What This Unlocks

- **Reusable validation pipelines** — define each validation rule as `fn(T) -> Option<T>` or `fn(T) -> Result<T, E>`, then compose them into a single validator without boilerplate wiring.
- **Dynamic step lists** — when the pipeline steps aren't known at compile time (e.g., loaded from config or user input), use `fold` over a `Vec<fn(T) -> Option<T>>` for automatic short-circuit behaviour.
- **Understanding `?`** — knowing that `?` *is* Kleisli composition explains why you can stack `?` operators in a function and they all short-circuit to the same return type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Kleisli operator | `let (>=>) f g x = f x >>= g` — infix operator | Named function returning `impl Fn` closure |
| Composing two arrows | `parse_int >=> check_positive` — point-free | `kleisli(parse_int, check_positive)` — explicit call |
| Dynamic pipeline | `List.fold_left (fun acc f -> acc >>= f)` | `fold` over `&[fn(T) -> Option<T>]` |
| Closure capture | Values captured by value (GC-managed) | Must use `move` in closures; ownership tracked |
| Multiple closure types | Closures share `fn` type | Each closure has a unique type; use `impl Fn` or `Box<dyn Fn>` |
