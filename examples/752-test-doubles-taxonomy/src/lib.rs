#![allow(clippy::all)]
//! # Test Doubles Taxonomy
//!
//! Stub, Mock, Fake, Spy patterns in Rust.

use std::cell::RefCell;

/// The dependency trait
pub trait Logger {
    fn log(&self, message: &str);
    fn error(&self, message: &str);
    fn warn(&self, message: &str);
}

// ═══════════════════════════════════════════════════════════════════════════════
// 1. STUB: Returns canned values, ignores everything
// ═══════════════════════════════════════════════════════════════════════════════

/// A null logger that does nothing (simplest stub)
pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&self, _: &str) {}
    fn error(&self, _: &str) {}
    fn warn(&self, _: &str) {}
}

// ═══════════════════════════════════════════════════════════════════════════════
// 2. SPY: Records calls for later verification
// ═══════════════════════════════════════════════════════════════════════════════

/// A spy that records all calls
pub struct SpyLogger {
    pub logs: RefCell<Vec<String>>,
    pub errors: RefCell<Vec<String>>,
    pub warns: RefCell<Vec<String>>,
}

impl SpyLogger {
    pub fn new() -> Self {
        SpyLogger {
            logs: RefCell::new(Vec::new()),
            errors: RefCell::new(Vec::new()),
            warns: RefCell::new(Vec::new()),
        }
    }

    pub fn log_count(&self) -> usize {
        self.logs.borrow().len()
    }

    pub fn error_count(&self) -> usize {
        self.errors.borrow().len()
    }
}

impl Default for SpyLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for SpyLogger {
    fn log(&self, message: &str) {
        self.logs.borrow_mut().push(message.to_string());
    }

    fn error(&self, message: &str) {
        self.errors.borrow_mut().push(message.to_string());
    }

    fn warn(&self, message: &str) {
        self.warns.borrow_mut().push(message.to_string());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// 3. MOCK: Verifies expected interactions
// ═══════════════════════════════════════════════════════════════════════════════

/// A mock that verifies specific expectations
pub struct MockLogger {
    expected_logs: RefCell<Vec<String>>,
    actual_logs: RefCell<Vec<String>>,
}

impl MockLogger {
    pub fn new() -> Self {
        MockLogger {
            expected_logs: RefCell::new(Vec::new()),
            actual_logs: RefCell::new(Vec::new()),
        }
    }

    pub fn expect_log(&self, message: &str) {
        self.expected_logs.borrow_mut().push(message.to_string());
    }

    pub fn verify(&self) -> bool {
        *self.expected_logs.borrow() == *self.actual_logs.borrow()
    }
}

impl Default for MockLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for MockLogger {
    fn log(&self, message: &str) {
        self.actual_logs.borrow_mut().push(message.to_string());
    }

    fn error(&self, _: &str) {}
    fn warn(&self, _: &str) {}
}

// ═══════════════════════════════════════════════════════════════════════════════
// 4. FAKE: Simplified working implementation
// ═══════════════════════════════════════════════════════════════════════════════

/// A fake logger that prints to stdout (simpler than file I/O)
pub struct ConsoleLogger {
    prefix: String,
}

impl ConsoleLogger {
    pub fn new(prefix: &str) -> Self {
        ConsoleLogger {
            prefix: prefix.to_string(),
        }
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[{}:INFO] {}", self.prefix, message);
    }

    fn error(&self, message: &str) {
        println!("[{}:ERROR] {}", self.prefix, message);
    }

    fn warn(&self, message: &str) {
        println!("[{}:WARN] {}", self.prefix, message);
    }
}

/// Service using the logger
pub struct OrderProcessor<L: Logger> {
    logger: L,
}

impl<L: Logger> OrderProcessor<L> {
    pub fn new(logger: L) -> Self {
        OrderProcessor { logger }
    }

    pub fn process(&self, order_id: u64) -> Result<(), String> {
        self.logger.log(&format!("Processing order {}", order_id));
        if order_id == 0 {
            self.logger.error("Invalid order ID");
            return Err("Invalid order ID".to_string());
        }
        self.logger.log(&format!("Order {} completed", order_id));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_null_logger() {
        let processor = OrderProcessor::new(NullLogger);
        assert!(processor.process(123).is_ok());
    }

    #[test]
    fn test_spy_records_calls() {
        let spy = SpyLogger::new();
        let processor = OrderProcessor::new(spy);
        processor.process(123).unwrap();

        assert_eq!(processor.logger.log_count(), 2);
        assert_eq!(processor.logger.error_count(), 0);
    }

    #[test]
    fn test_spy_records_errors() {
        let spy = SpyLogger::new();
        let processor = OrderProcessor::new(spy);
        let _ = processor.process(0);

        assert_eq!(processor.logger.error_count(), 1);
    }

    #[test]
    fn test_mock_verification() {
        let mock = MockLogger::new();
        mock.expect_log("Processing order 42");
        mock.expect_log("Order 42 completed");

        let processor = OrderProcessor::new(mock);
        processor.process(42).unwrap();

        assert!(processor.logger.verify());
    }

    #[test]
    fn test_mock_verification_fails() {
        let mock = MockLogger::new();
        mock.expect_log("wrong message");

        let processor = OrderProcessor::new(mock);
        processor.process(42).unwrap();

        assert!(!processor.logger.verify());
    }
}
