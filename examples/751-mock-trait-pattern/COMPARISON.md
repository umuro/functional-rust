# OCaml vs Rust: Mock Trait Pattern

## Dependency Injection via Traits/Modules

### Rust
```rust
pub trait EmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

pub struct NotificationService<E: EmailSender> {
    sender: E,
}
```

### OCaml (Modules)
```ocaml
module type EMAIL_SENDER = sig
  val send : to_:string -> subject:string -> body:string -> (unit, string) result
end

module NotificationService (E : EMAIL_SENDER) = struct
  let notify_user email message =
    E.send ~to_:email ~subject:"Notification" ~body:message
end
```

## Mock Implementation

### Rust
```rust
pub struct MockEmailSender {
    pub calls: RefCell<Vec<(String, String, String)>>,
    pub should_fail: bool,
}

impl EmailSender for MockEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        self.calls.borrow_mut().push((to.into(), subject.into(), body.into()));
        if self.should_fail { Err("Mock failure".into()) } else { Ok(()) }
    }
}
```

### OCaml
```ocaml
module MockEmail : EMAIL_SENDER = struct
  let calls = ref []
  let should_fail = ref false
  
  let send ~to_ ~subject ~body =
    calls := (to_, subject, body) :: !calls;
    if !should_fail then Error "Mock failure" else Ok ()
end
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Abstraction | First-class modules | Traits + generics |
| Interior mutability | `ref` | `RefCell` |
| Mocking libraries | Less common | mockall, mockito |
| Type inference | Full | May need turbofish |
| Runtime polymorphism | Functors | dyn Trait |
