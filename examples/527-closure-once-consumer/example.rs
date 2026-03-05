//! # 527. FnOnce for Consuming Closures
//! Closures that consume their captured values — callable only once.

/// A resource that can only be "used" once
struct OneTimeToken {
    value: String,
}

impl OneTimeToken {
    fn new(s: &str) -> Self { OneTimeToken { value: s.to_string() } }
    fn consume(self) -> String { self.value } // consumes self
}

/// FnOnce: captures and consumes a OneTimeToken
fn make_token_consumer(token: OneTimeToken) -> impl FnOnce() -> String {
    move || token.consume() // token is moved and consumed on call
}

/// Resource cleanup via FnOnce
fn with_resource<R, T, F: FnOnce(R) -> T>(resource: R, f: F) -> T {
    f(resource) // resource is moved into f, cleaned up after
}

/// Deferred action: run once, then nothing
struct OnceAction {
    action: Option<Box<dyn FnOnce()>>,
}

impl OnceAction {
    fn new(f: impl FnOnce() + 'static) -> Self {
        OnceAction { action: Some(Box::new(f)) }
    }

    fn trigger(&mut self) {
        if let Some(f) = self.action.take() {
            f(); // FnOnce called, action dropped
        } else {
            println!("[already triggered — no-op]");
        }
    }
}

fn main() {
    // Basic FnOnce — consume a String
    let msg = String::from("Hello, once!");
    let greet = move || {
        println!("{}", msg); // msg is moved into closure, consumed on call
    };
    greet(); // OK
    // greet(); // ERROR: use of moved value — closure is FnOnce!

    // Token consumer
    let token = OneTimeToken::new("secret-auth-token-xyz");
    let consume = make_token_consumer(token);
    let value = consume(); // token consumed here
    println!("Token value: {}", value);
    // consume(); // ERROR: consume is FnOnce, already called

    // with_resource: transfer and use
    let data = vec![1, 2, 3, 4, 5];
    let sum = with_resource(data, |v| v.into_iter().sum::<i32>());
    println!("Sum: {}", sum);
    // data is gone — moved into with_resource

    // OnceAction: deferred one-shot execution
    let mut action = OnceAction::new(|| println!("Action executed!"));
    action.trigger(); // runs
    action.trigger(); // no-op (already consumed)
    action.trigger(); // no-op

    // FnOnce in threads — spawn and consume
    let payload = vec![10, 20, 30];
    let handle = std::thread::spawn(move || {
        // payload is consumed — moved into thread
        payload.iter().sum::<i32>()
    });
    println!("Thread result: {}", handle.join().unwrap());

    // Using call_once pattern
    fn call_once<F: FnOnce() -> T, T>(f: F) -> T { f() }
    let name = String::from("Rust");
    let result = call_once(move || format!("Hello from {}!", name));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_consumer() {
        let token = OneTimeToken::new("test-token");
        let f = make_token_consumer(token);
        assert_eq!(f(), "test-token");
    }

    #[test]
    fn test_with_resource() {
        let v = vec![1, 2, 3];
        let result = with_resource(v, |data| data.len());
        assert_eq!(result, 3);
    }

    #[test]
    fn test_once_action() {
        let mut count = 0;
        // Can't test mutable capture in OnceAction easily without Arc/Mutex
        // Just test it runs once
        let mut action = OnceAction::new(|| println!("once!"));
        action.trigger();
        action.trigger(); // second call is no-op
        assert!(action.action.is_none());
        let _ = count; count += 1; let _ = count; // suppress warnings
    }
}
