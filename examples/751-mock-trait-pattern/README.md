# 751: Mocking via Traits: Test Doubles in Rust

**Difficulty:** 3  **Level:** Advanced

Create a test double by implementing a trait differently for tests — no mocking framework needed, because traits *are* the interface.

## The Problem This Solves

In production code, your `UserService` sends real emails via SMTP. In tests, you don't want to hit a real mail server — that's slow, fragile, and sends actual emails to real people. You need a way to substitute a fake implementation that records what was sent so you can assert on it.

In Python and JavaScript, mocking frameworks like `unittest.mock` or Jest's `jest.fn()` work by patching function references at runtime. Rust's type system makes runtime patching impossible — but it makes a better approach obvious: define the dependency as a trait, and implement that trait differently in tests.

This pattern also forces good design. If you can't easily swap out a dependency, it's probably too tightly coupled. Designing around a `Sender` trait rather than a concrete `SmtpSender` struct makes your business logic more testable *and* more flexible.

## The Intuition

Think of a trait as a Java interface. Your `UserService<S: EmailSender>` accepts *any* type that can send email. In production you pass `SmtpSender`. In tests you pass `MockEmailSender`, which records all calls in a `Vec` instead of hitting a network.

Unlike Jest's `jest.fn()`, there's no global registry or monkey-patching. The compiler verifies at compile time that your mock satisfies the same contract as the real implementation. If you add a method to the `EmailSender` trait, the mock fails to compile until you implement it.

`RefCell<Vec<SentEmail>>` lets the mock record sent emails even when the `send` method only has `&self` (shared reference). This is interior mutability — a common pattern in Rust test doubles.

## How It Works in Rust

```rust
// The interface — both real and mock implement this
pub trait EmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

// Production: hits a real SMTP server
pub struct SmtpSender { pub host: String, pub port: u16 }

impl EmailSender for SmtpSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        // ... real SMTP connection
        Ok(())
    }
}

// Test double: records calls, can be configured to fail
pub struct MockEmailSender {
    sent: RefCell<Vec<SentEmail>>,  // interior mutability — &self can still record
    should_fail: bool,
}

impl EmailSender for MockEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        if self.should_fail { return Err("mock SMTP failure".into()); }
        self.sent.borrow_mut().push(SentEmail {
            to: to.into(), subject: subject.into(), body: body.into(),
        });
        Ok(())
    }
}

// Business logic — generic over any EmailSender
pub struct UserService<S: EmailSender> { sender: S }

impl<S: EmailSender> UserService<S> {
    pub fn welcome_user(&self, email: &str, name: &str) -> Result<(), String> {
        self.sender.send(email, "Welcome!", &format!("Hi {name}, welcome!"))
    }
}

// In tests — inject the mock
#[test]
fn welcome_user_sends_to_correct_address() {
    let mock = MockEmailSender::new();
    let svc = UserService::new(mock);
    svc.welcome_user("alice@example.com", "Alice").unwrap();

    let sent = svc.sender.get_sent();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].to, "alice@example.com");
    assert!(sent[0].body.contains("Alice"));
}

#[test]
fn error_from_sender_propagates() {
    let mock = MockEmailSender::failing();  // configured to fail
    let svc = UserService::new(mock);
    assert!(svc.welcome_user("x@y.com", "X").is_err());
}
```

Key points:
- The mock is a plain Rust struct implementing the same trait — no special framework
- `RefCell` provides interior mutability so `&self` methods can mutate state
- `MockEmailSender::failing()` tests the error path without touching a real server
- The compiler guarantees mock and real implementation have the same interface

## What This Unlocks

- **Test business logic without infrastructure**: services that send emails, write to databases, or call HTTP APIs can be fully tested without any external system
- **Verify call counts and arguments**: the mock records everything — assert that exactly one email was sent, with the right subject, containing the right token
- **Test error paths**: create a mock configured to fail and verify your error handling is correct

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mocking approach | First-class functions or modules | Trait implementations — no framework |
| Runtime patching | Possible via mutable references | Impossible by design; use generics |
| Interior mutability | References are mutable | `RefCell<T>` for `&self` mutation |
| Compile-time safety | OCaml module types | Generic bound `S: EmailSender` verified at compile time |
| Failure injection | Pass a different function | Construct mock with `MockEmailSender::failing()` |
| External mock library | N/A | `mockall` crate for auto-generated mocks |
