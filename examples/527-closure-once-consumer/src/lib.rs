#![allow(clippy::all)]
//! FnOnce for Consuming Closures
//!
//! Closures that consume their captured values — callable only once.

/// A resource that can only be "used" once.
pub struct OneTimeToken {
    value: String,
}

impl OneTimeToken {
    pub fn new(s: &str) -> Self {
        OneTimeToken {
            value: s.to_string(),
        }
    }

    pub fn consume(self) -> String {
        self.value
    }
}

/// FnOnce: captures and consumes a OneTimeToken.
pub fn make_token_consumer(token: OneTimeToken) -> impl FnOnce() -> String {
    move || token.consume()
}

/// Resource cleanup via FnOnce.
pub fn with_resource<R, T, F: FnOnce(R) -> T>(resource: R, f: F) -> T {
    f(resource)
}

/// Deferred action: run once, then nothing.
pub struct OnceAction<F: FnOnce()> {
    action: Option<F>,
}

impl<F: FnOnce()> OnceAction<F> {
    pub fn new(action: F) -> Self {
        OnceAction {
            action: Some(action),
        }
    }

    pub fn run(mut self) {
        if let Some(f) = self.action.take() {
            f();
        }
    }
}

/// Builder that produces a value once.
pub struct OnceBuilder<T> {
    builder: Option<Box<dyn FnOnce() -> T>>,
}

impl<T> OnceBuilder<T> {
    pub fn new(f: impl FnOnce() -> T + 'static) -> Self {
        OnceBuilder {
            builder: Some(Box::new(f)),
        }
    }

    pub fn build(mut self) -> Option<T> {
        self.builder.take().map(|f| f())
    }
}

/// Move-only resource for RAII.
pub struct FileHandle {
    name: String,
    is_open: bool,
}

impl FileHandle {
    pub fn open(name: &str) -> Self {
        FileHandle {
            name: name.to_string(),
            is_open: true,
        }
    }

    pub fn close(mut self) -> String {
        self.is_open = false;
        format!("Closed: {}", self.name)
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Use FnOnce with Result for error handling.
pub fn try_once<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<T, E> {
    f()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_token_consumer() {
        let token = OneTimeToken::new("secret123");
        let consumer = make_token_consumer(token);
        // Can only call once
        let value = consumer();
        assert_eq!(value, "secret123");
        // consumer(); // ERROR: already consumed
    }

    #[test]
    fn test_with_resource() {
        let handle = FileHandle::open("test.txt");
        let result = with_resource(handle, |h| h.close());
        assert_eq!(result, "Closed: test.txt");
    }

    #[test]
    fn test_once_action() {
        let counter = RefCell::new(0);
        let action = OnceAction::new(|| {
            *counter.borrow_mut() += 1;
        });
        action.run();
        assert_eq!(*counter.borrow(), 1);
        // action.run(); // ERROR: moved
    }

    #[test]
    fn test_once_builder() {
        let builder = OnceBuilder::new(|| vec![1, 2, 3]);
        let result = builder.build();
        assert_eq!(result, Some(vec![1, 2, 3]));
        // builder.build(); // ERROR: moved
    }

    #[test]
    fn test_file_handle() {
        let handle = FileHandle::open("data.txt");
        assert!(handle.is_open());
        let msg = handle.close();
        assert!(msg.contains("Closed"));
    }

    #[test]
    fn test_try_once_ok() {
        let result: Result<i32, &str> = try_once(|| Ok(42));
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_try_once_err() {
        let result: Result<i32, &str> = try_once(|| Err("failed"));
        assert_eq!(result, Err("failed"));
    }

    #[test]
    fn test_fn_once_in_option() {
        let consume = |s: String| s.len();
        let opt = Some("hello".to_string());
        let result = opt.map(consume);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_fn_once_trait_bound() {
        fn apply_once<T, F: FnOnce() -> T>(f: F) -> T {
            f()
        }

        let s = String::from("owned");
        let result = apply_once(move || s.len());
        assert_eq!(result, 5);
    }
}
