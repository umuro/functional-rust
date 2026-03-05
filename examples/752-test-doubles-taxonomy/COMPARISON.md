# OCaml vs Rust: Test Doubles Taxonomy

## The Four Test Doubles

| Type | Purpose | Example |
|------|---------|---------|
| **Stub** | Returns canned values | NullLogger |
| **Spy** | Records calls for verification | SpyLogger |
| **Mock** | Verifies expected interactions | MockLogger |
| **Fake** | Simplified working impl | ConsoleLogger |

## Stub (Simplest)

### Rust
```rust
pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&self, _: &str) {}
    fn error(&self, _: &str) {}
}
```

### OCaml
```ocaml
module NullLogger : LOGGER = struct
  let log _ = ()
  let error _ = ()
end
```

## Spy (Records Calls)

### Rust
```rust
pub struct SpyLogger {
    pub logs: RefCell<Vec<String>>,
}

impl Logger for SpyLogger {
    fn log(&self, message: &str) {
        self.logs.borrow_mut().push(message.to_string());
    }
}
```

## Mock (Verifies Expectations)

### Rust
```rust
let mock = MockLogger::new();
mock.expect_log("Processing order 42");
mock.expect_log("Order 42 completed");

process_order(42, &mock);

assert!(mock.verify());
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Interior mutability | `ref` | `RefCell` |
| Modules vs Traits | First-class modules | Trait objects/generics |
| Mocking libraries | Less common | mockall, mockito |
| Verification | Manual | Can be automated |

## When to Use Each

- **Stub**: When you don't care about interactions
- **Spy**: When you want to verify what was called
- **Mock**: When you need strict interaction checking
- **Fake**: When you need a simpler but working implementation
