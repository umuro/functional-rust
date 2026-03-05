/// 752: Test Doubles Taxonomy — Stub, Mock, Fake, Spy in Rust

use std::cell::RefCell;

// ── The dependency trait ──────────────────────────────────────────────────────

pub trait Logger {
    fn log(&self, message: &str);
    fn error(&self, message: &str);
    fn warn(&self, message: &str);
}

// ── 1. Stub: returns nothing, ignores everything ──────────────────────────────

pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&self, _: &str)   {}
    fn error(&self, _: &str) {}
    fn warn(&self, _: &str)  {}
}

// ── 2. Fake: working but simplified (in-memory) ───────────────────────────────

pub struct InMemoryLogger {
    logs:   RefCell<Vec<String>>,
    errors: RefCell<Vec<String>>,
    warns:  RefCell<Vec<String>>,
}

impl InMemoryLogger {
    pub fn new() -> Self {
        InMemoryLogger {
            logs:   RefCell::new(Vec::new()),
            errors: RefCell::new(Vec::new()),
            warns:  RefCell::new(Vec::new()),
        }
    }
    pub fn logs(&self)   -> Vec<String> { self.logs.borrow().clone() }
    pub fn errors(&self) -> Vec<String> { self.errors.borrow().clone() }
    pub fn warns(&self)  -> Vec<String> { self.warns.borrow().clone() }
    pub fn all_count(&self) -> usize {
        self.logs.borrow().len() + self.errors.borrow().len() + self.warns.borrow().len()
    }
}

impl Logger for InMemoryLogger {
    fn log(&self, msg: &str)   { self.logs.borrow_mut().push(msg.to_owned()); }
    fn error(&self, msg: &str) { self.errors.borrow_mut().push(msg.to_owned()); }
    fn warn(&self, msg: &str)  { self.warns.borrow_mut().push(msg.to_owned()); }
}

// ── 3. Mock: records calls, asserts on them ────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LogCall {
    pub level:   &'static str,
    pub message: String,
}

pub struct MockLogger {
    calls: RefCell<Vec<LogCall>>,
}

impl MockLogger {
    pub fn new() -> Self { MockLogger { calls: RefCell::new(Vec::new()) } }
    pub fn call_count(&self) -> usize { self.calls.borrow().len() }
    pub fn calls(&self) -> Vec<LogCall> { self.calls.borrow().clone() }
    pub fn assert_called_with(&self, level: &str, msg: &str) {
        let calls = self.calls.borrow();
        assert!(
            calls.iter().any(|c| c.level == level && c.message.contains(msg)),
            "Expected a {} call containing '{}', got: {:?}",
            level, msg, calls
        );
    }
    pub fn assert_call_count(&self, expected: usize) {
        assert_eq!(self.call_count(), expected,
            "Expected {} calls, got {}", expected, self.call_count());
    }
}

impl Logger for MockLogger {
    fn log(&self, msg: &str) {
        self.calls.borrow_mut().push(LogCall { level: "log", message: msg.to_owned() });
    }
    fn error(&self, msg: &str) {
        self.calls.borrow_mut().push(LogCall { level: "error", message: msg.to_owned() });
    }
    fn warn(&self, msg: &str) {
        self.calls.borrow_mut().push(LogCall { level: "warn", message: msg.to_owned() });
    }
}

// ── 4. Spy: wraps real impl, also records calls ───────────────────────────────

pub struct SpyLogger<Inner: Logger> {
    inner:      Inner,
    call_count: RefCell<usize>,
}

impl<I: Logger> SpyLogger<I> {
    pub fn new(inner: I) -> Self {
        SpyLogger { inner, call_count: RefCell::new(0) }
    }
    pub fn call_count(&self) -> usize { *self.call_count.borrow() }
}

impl<I: Logger> Logger for SpyLogger<I> {
    fn log(&self, msg: &str) {
        *self.call_count.borrow_mut() += 1;
        self.inner.log(msg);        // also calls the real implementation
    }
    fn error(&self, msg: &str) {
        *self.call_count.borrow_mut() += 1;
        self.inner.error(msg);
    }
    fn warn(&self, msg: &str) {
        *self.call_count.borrow_mut() += 1;
        self.inner.warn(msg);
    }
}

// ── Business logic ────────────────────────────────────────────────────────────

pub fn process_items(items: &[i32], logger: &dyn Logger) -> (usize, usize) {
    let mut ok = 0usize;
    let mut errs = 0usize;
    for &item in items {
        if item < 0 {
            logger.error(&format!("negative item: {}", item));
            errs += 1;
        } else if item == 0 {
            logger.warn("zero item encountered");
            ok += 1;
        } else {
            logger.log(&format!("processing: {}", item));
            ok += 1;
        }
    }
    (ok, errs)
}

fn main() {
    let data = &[1i32, -2, 0, 3, -4];
    let logger = InMemoryLogger::new();
    let (ok, errs) = process_items(data, &logger);
    println!("ok={} errs={}", ok, errs);
    println!("Logs:   {:?}", logger.logs());
    println!("Errors: {:?}", logger.errors());
    println!("Warns:  {:?}", logger.warns());
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Stub test: we don't care about logging behaviour ─────────────────────

    #[test]
    fn stub_does_not_panic() {
        let stub = NullLogger;
        process_items(&[1, 2, 3], &stub);   // just verify it runs
    }

    // ── Fake tests: verify observable state ───────────────────────────────────

    #[test]
    fn fake_records_errors_for_negative_items() {
        let fake = InMemoryLogger::new();
        process_items(&[1, -2, -3], &fake);
        assert_eq!(fake.errors().len(), 2);
        assert!(fake.errors()[0].contains("-2"));
    }

    #[test]
    fn fake_records_warns_for_zero() {
        let fake = InMemoryLogger::new();
        process_items(&[0, 0], &fake);
        assert_eq!(fake.warns().len(), 2);
    }

    // ── Mock tests: verify interactions ──────────────────────────────────────

    #[test]
    fn mock_assert_called_with_error_for_negative() {
        let mock = MockLogger::new();
        process_items(&[-42], &mock);
        mock.assert_called_with("error", "-42");
    }

    #[test]
    fn mock_assert_call_count() {
        let mock = MockLogger::new();
        process_items(&[1, 2, -3, 0, -5], &mock);
        mock.assert_call_count(5);  // one call per item
    }

    // ── Spy tests: real impl + call recording ────────────────────────────────

    #[test]
    fn spy_wraps_fake_and_counts() {
        let inner = InMemoryLogger::new();
        let spy   = SpyLogger::new(inner);
        process_items(&[1, -2, 0], &spy);
        assert_eq!(spy.call_count(), 3);
        // Inner fake also received the calls:
        assert_eq!(spy.inner.errors().len(), 1);
    }
}
