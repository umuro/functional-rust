# OCaml vs Rust: Test Isolation Patterns

## The Problem: Test Pollution

Global mutable state causes tests to interfere with each other:
- Order-dependent failures
- Flaky tests
- Non-reproducible bugs

## Solution: Dependency Injection

### Rust
```rust
pub trait Counter {
    fn increment(&self) -> u64;
}

pub struct Service<C: Counter> {
    counter: C,
}

impl<C: Counter> Service<C> {
    pub fn new(counter: C) -> Self {
        Service { counter }
    }
}
```

### OCaml
```ocaml
module type COUNTER = sig
  val increment : unit -> int
end

module Service (C : COUNTER) = struct
  let process () = C.increment ()
end
```

## Per-Test Isolation

### Rust
```rust
#[test]
fn test_1() {
    let counter = IsolatedCounter::new();  // Fresh state
    let service = Service::new(counter);
    assert_eq!(service.process(), 1);
}

#[test]
fn test_2() {
    let counter = IsolatedCounter::new();  // Another fresh state
    let service = Service::new(counter);
    assert_eq!(service.process(), 1);  // Same result!
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| DI mechanism | First-class modules | Generics + traits |
| Interior mutability | `ref` | `RefCell`, `Mutex` |
| Global state | Discouraged | `OnceLock`, `lazy_static` |
| Thread safety | Not automatic | `Sync` + `Send` bounds |
