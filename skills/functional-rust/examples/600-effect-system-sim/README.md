# 600: Simulating Algebraic Effects

**Difficulty:** 4  **Level:** Advanced

Simulate an effect system in Rust: write pure business logic that performs logging, random number generation, and state updates — then plug in a production handler or a deterministic test handler without changing the logic.

## The Problem This Solves

You have business logic that needs to log messages, generate random numbers, and read/write some state. In the straightforward approach, all of these are direct calls: `println!`, `rand::thread_rng().gen()`, `self.state += 1`. The code works — but it's now impossible to test without accepting the side effects.

Want to test the logic without printing to the terminal? You'd need to capture stdout. Want deterministic random numbers in tests? You'd need to seed a specific RNG and thread it through every function. Want to inspect what the state looks like at each step? You'd need to add logging statements manually and remove them for production.

The deeper problem: **side effects are hardcoded into the logic**. Every function that calls `println!` is tightly coupled to stdout. Every function that calls `rand::thread_rng()` is tightly coupled to the OS random source. This coupling makes the logic hard to test, hard to replay, and hard to run in different contexts (embedded systems with no stdout, WASM environments with no OS random, sandboxed executors).

An **effect system** solves this by making the effects part of the function's _contract_. Instead of calling `println!` directly, your function calls `h.log("message")` — where `h` is a handler. The handler decides what `log` actually does. In production: prints to stdout. In tests: saves to a `Vec<String>`. In a sandboxed environment: drops the message silently. The logic is identical in all cases. This is exactly that pain solved.

## The Intuition

Think of a plugin interface in a game engine. Your game logic says "play a sound." The engine has a sound system. In production, the sound system plays audio through speakers. In a headless server, the sound system is a no-op. In a test, it records which sounds were "played" so you can assert on them.

Your game logic doesn't care _how_ sound is played — it just calls `engine.play_sound("explosion.wav")`. The engine (the "handler") decides what that means.

That's exactly what the `EffectHandler` trait does here:

```rust
trait EffectHandler {
    fn log(&mut self, msg: &str);               // "I want to log something"
    fn random(&mut self, lo: i32, hi: i32) -> i32; // "I want a random number in range"
    fn get_state(&self) -> i32;                 // "I want to read state"
    fn set_state(&mut self, v: i32);            // "I want to write state"
}
```

This is the "effect signature" — all the things your program is allowed to do. Crucially, it doesn't say _how_ those things are done. That's the handler's job.

The production handler connects to real systems. The test handler is deterministic and captures everything for inspection.

## How It Works in Rust

**Step 1: Define the effect signature as a trait**

```rust
trait EffectHandler {
    fn log(&mut self, msg: &str);
    fn random(&mut self, lo: i32, hi: i32) -> i32;
    fn get_state(&self) -> i32;
    fn set_state(&mut self, v: i32);
}
```

This is your "capability interface." The business logic only sees this trait — it knows nothing about how any of these are implemented.

**Step 2: The business logic uses the handler — no direct side effects**

```rust
fn simulate(h: &mut dyn EffectHandler) {
    h.log("Starting simulation");

    let v = h.get_state();
    h.set_state(v + 10);  // no direct mutation — goes through the handler
    h.log(&format!("State after +10: {}", h.get_state()));

    let r = h.random(1, 100);  // no rand::thread_rng() — handler decides
    h.log(&format!("Random roll: {}", r));

    h.set_state(h.get_state() + r);
    h.log(&format!("Final state: {}", h.get_state()));
}
```

`simulate` has no `println!`, no `rand`, no global state. Every effect is explicit through `h`. The function is pure in the sense that its only interaction with the outside world goes through the handler.

**Step 3: Production handler — real effects**

```rust
struct ProdHandler { state: i32 }

impl EffectHandler for ProdHandler {
    fn log(&mut self, msg: &str) {
        println!("[LOG] {}", msg);  // real stdout
    }
    fn random(&mut self, lo: i32, hi: i32) -> i32 {
        // pseudo-random using current state as seed
        (self.state.wrapping_mul(1664525).wrapping_add(1013904223).abs() % (hi - lo) + lo)
    }
    fn get_state(&self) -> i32 { self.state }
    fn set_state(&mut self, v: i32) { self.state = v; }
}
```

**Step 4: Test handler — deterministic and inspectable**

```rust
struct TestHandler {
    state: i32,
    log: Vec<String>,      // captures all log messages
    rand_seq: Vec<i32>,    // pre-determined random sequence
    rand_idx: usize,
}

impl EffectHandler for TestHandler {
    fn log(&mut self, msg: &str) {
        self.log.push(msg.to_string());  // capture instead of print
    }
    fn random(&mut self, _lo: i32, _hi: i32) -> i32 {
        let v = self.rand_seq[self.rand_idx % self.rand_seq.len()];
        self.rand_idx += 1;
        v  // deterministic — test controls the "random" values
    }
    fn get_state(&self) -> i32 { self.state }
    fn set_state(&mut self, v: i32) { self.state = v; }
}
```

**Step 5: Same logic, different handlers**

```rust
// Production
let mut prod = ProdHandler { state: 0 };
simulate(&mut prod);  // prints to stdout, uses pseudo-random

// Test — fully deterministic, no output
let mut test = TestHandler {
    state: 0,
    log: vec![],
    rand_seq: vec![7],  // "random" will always return 7
    rand_idx: 0,
};
simulate(&mut test);

// 0 + 10 + 7 = 17
assert_eq!(test.state, 17);
assert!(test.log[0].contains("Starting simulation"));
assert!(test.log.len() == 4);  // exactly 4 log entries
```

The test is completely deterministic. You can assert on every side effect. No stdout capture, no RNG seeding, no global state.

## What This Unlocks

- **Deterministic testing of effectful code**: Feed a fixed random sequence, inspect every log entry, assert on final state — without any test framework magic or environment manipulation.
- **Swappable backends**: Replace `ProdHandler` with a `NetworkHandler` (logs to a remote service), `WasmHandler` (logs to browser console), or `NoopHandler` (drops everything) — logic unchanged.
- **The insight behind Rust async**: `async`/`await` is an algebraic effect system. `await` is "perform the Async effect." The runtime (`tokio::main`, `async_std`) is the handler that decides how to schedule and resume futures. Understanding effect handlers illuminates how async works at a conceptual level.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Effects | OCaml 5 native `effect` keyword | Trait-based simulation (no native effects) |
| Performing an effect | `perform Effect_Name arg` | Method call `h.effect_name(arg)` |
| Effect handlers | `match_with` / `handle` blocks | `impl EffectHandler for Struct` |
| Resuming after effect | `continue k value` (delimited) | Return value from trait method |
| Composing handlers | Effect rows, algebraic | Multiple traits or nested handler structs |
| Type-level tracking | Effect types in function signatures | Trait bound on `h: &mut dyn EffectHandler` |
