# 574: if let and while let

**Difficulty:** 2  **Level:** Beginner

Match a single pattern in a conditional — cleaner than full `match` for one-arm cases.

## The Problem This Solves

Not every pattern match needs five arms. Sometimes you have an `Option<i32>` and you only want to do something if it's `Some`. A full `match` with a `None => {}` arm is syntactic overhead for what's conceptually a conditional.

The worse form is `match` used as `if`: `match maybe { Some(v) => use(v), _ => {} }`. The `_ => {}` arm is pure noise. `if let` removes it.

`while let` solves a specific loop problem: draining a stack, processing a channel until it's closed, iterating an iterator manually. `while stack.pop() != None` doesn't bind the value. `while stack.pop().is_some()` throws away the value. `while let Some(top) = stack.pop()` does exactly what you mean.

## The Intuition

`if let Some(v) = expr` is shorthand for `match expr { Some(v) => { ... } _ => {} }`. It's a match with exactly one useful arm. The binding (`v`) is in scope inside the block. Add an `else` branch for the mismatch case.

`while let` is the same thing in a loop: "keep looping as long as this pattern matches." The loop stops the moment it doesn't.

Python's `match` (3.10+) has no equivalent for `if let` — you'd write `if isinstance(x, SomeType)`. JavaScript has no equivalent — you'd unwrap manually. Both `if let` and `while let` are ergonomic features unique to Rust (and Swift, which has a very similar `if let`).

The sharp edge: `if let` is not exhaustive. If you add a variant to your enum, the `if let` silently misses it. Use `match` when you need to handle all cases. Use `if let` when you genuinely only care about one.

## How It Works in Rust

```rust
// Basic if let — no _ noise
let maybe: Option<i32> = Some(42);
if let Some(v) = maybe {
    println!("got {}", v);  // v: i32 in scope here
}

// if let + else — handle both cases without match
let r: Result<i32, &str> = Err("oops");
if let Ok(n) = r {
    println!("ok: {}", n);
} else {
    println!("failed");  // n is NOT in scope here
}

// while let — drain a stack naturally
let mut stack = vec![1, 2, 3, 4, 5];
while let Some(top) = stack.pop() {
    print!("{} ", top);  // 5 4 3 2 1
}

// while let — manual iterator control
let mut it = vec!["hello", "world"].into_iter();
while let Some(w) = it.next() {
    println!("word: {}", w);
}

// if let on enum with data
for msg in &msgs {
    if let Msg::Move { x, y } = msg {
        println!("move to ({},{})", x, y);  // only fires for Move variant
    } else if let Msg::Write(t) = msg {
        println!("write: {}", t);
    }
    // Quit silently ignored — intentional
}

// while let + early exit
while let Some(cmd) = queue.pop_front() {
    if cmd == "stop" { break; }
    println!("cmd: {}", cmd);
}
```

## What This Unlocks

- **Noise-free single-arm matching** — skip the `_ => {}` boilerplate of full match when you only care about one shape.
- **Natural stack/queue draining** — `while let Some(x) = collection.pop()` is the idiomatic loop pattern.
- **Chained conditional matching** — `if let ... else if let ...` for multi-variant, non-exhaustive dispatch.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Single-arm match | `match v with Some n -> ... \| _ -> ()` | `if let Some(n) = v { ... }` |
| With else | Two-arm match | `if let ... { } else { }` |
| Loop until none | `while`-loop + explicit check | `while let Some(x) = expr { }` |
| Exhaustiveness | All arms required (warning if not) | `if let` is explicitly non-exhaustive |
| Chained ifs | `match ... with \| ...` | `if let ... else if let ...` |
