📖 **[View on hightechmind.io →](https://hightechmind.io/rust/751-mock-trait-pattern)**

---

# 751-mock-trait-pattern — Mock Trait Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Code that sends emails, makes HTTP requests, or writes to databases is hard to test: real calls are slow, cost money, and have side effects. The mock trait pattern defines a dependency as a trait, injects it through generics or `dyn Trait`, and provides a test implementation that records calls without performing real work. This is the foundation of dependency injection in Rust and is used in every major Rust web framework's testing guide.

## Learning Outcomes

- Define `EmailSender` as a trait with a `send` method
- Inject the dependency generically: `NotificationService<E: EmailSender>`
- Implement `MockEmailSender` using `RefCell<Vec<SentMessage>>` to record calls
- Verify recorded calls in tests: count, recipients, subjects, bodies
- Understand when to use `dyn Trait` (runtime polymorphism) vs. generics (compile-time)

## Rust Application

`SmtpSender` is the production implementation; `MockEmailSender` stores `RefCell<Vec<(String, String, String)>>` recording each `(to, subject, body)` call. `NotificationService<E: EmailSender>` calls `sender.send(...)` in `notify_user` and `send_welcome`. Tests create a `MockEmailSender`, pass it to `NotificationService::new`, call service methods, then inspect `mock.sent_emails()` to verify the right messages were sent to the right recipients.

## OCaml Approach

OCaml modules serve as the primary abstraction for mocking. A functor `Make(Sender: SENDER_SIG)` creates a service that depends on the `SENDER_SIG` module type. Tests pass a mock module to the functor. OCaml's first-class modules and functors make this pattern very natural. Alternatively, mutable reference cells (`ref`) capture calls in a mock module implementation.

## Key Differences

1. **Mechanism**: Rust uses traits + generics or `dyn Trait`; OCaml uses module types + functors or first-class modules.
2. **Interior mutability**: Rust's `RefCell` is needed to mutate the call log through a `&self` reference; OCaml uses `ref` cells or `Queue.t` directly.
3. **Ergonomics**: Rust's `mockall` crate generates mock structs from trait definitions via derive macros; OCaml's `mockmod` is less mature.
4. **Runtime cost**: Rust generics generate monomorphized code with zero `dyn` overhead; OCaml's functor application is also zero-cost.

## Exercises

1. Add a `FailingEmailSender` that always returns `Err("SMTP connection refused")` and write tests for how `NotificationService` handles delivery failures.
2. Implement a `RecordingEmailSender` that records calls AND delegates to another sender — useful for integration tests that log without blocking real sends.
3. Use `dyn EmailSender` instead of a generic parameter in `NotificationService` and compare the ergonomics and performance implications.
