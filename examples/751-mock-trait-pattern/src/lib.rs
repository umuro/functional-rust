#![allow(clippy::all)]
//! # Mock Trait Pattern
//!
//! Test doubles without external crates using trait-based mocking.

use std::cell::RefCell;

/// The dependency trait that will be mocked
pub trait EmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

/// Real implementation for production
pub struct SmtpSender {
    pub host: String,
    pub port: u16,
}

impl EmailSender for SmtpSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        // In production, this would send a real email
        println!(
            "[SMTP {}:{}] To={} Subject={} Body={}",
            self.host, self.port, to, subject, body
        );
        Ok(())
    }
}

/// A service that depends on EmailSender
pub struct NotificationService<E: EmailSender> {
    sender: E,
}

impl<E: EmailSender> NotificationService<E> {
    pub fn new(sender: E) -> Self {
        NotificationService { sender }
    }

    pub fn notify_user(&self, email: &str, message: &str) -> Result<(), String> {
        self.sender.send(email, "Notification", message)
    }

    pub fn send_welcome(&self, email: &str, name: &str) -> Result<(), String> {
        let body = format!("Welcome, {}!", name);
        self.sender.send(email, "Welcome to our service", &body)
    }
}

/// Mock implementation for testing - records all calls
pub struct MockEmailSender {
    pub calls: RefCell<Vec<(String, String, String)>>,
    pub should_fail: bool,
}

impl MockEmailSender {
    pub fn new() -> Self {
        MockEmailSender {
            calls: RefCell::new(Vec::new()),
            should_fail: false,
        }
    }

    pub fn failing() -> Self {
        MockEmailSender {
            calls: RefCell::new(Vec::new()),
            should_fail: true,
        }
    }

    pub fn call_count(&self) -> usize {
        self.calls.borrow().len()
    }

    pub fn last_call(&self) -> Option<(String, String, String)> {
        self.calls.borrow().last().cloned()
    }
}

impl Default for MockEmailSender {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailSender for MockEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push((to.to_string(), subject.to_string(), body.to_string()));

        if self.should_fail {
            Err("Mock failure".to_string())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_user_sends_email() {
        let mock = MockEmailSender::new();
        let service = NotificationService::new(mock);

        service.notify_user("user@example.com", "Hello!").unwrap();

        let calls = service.sender.calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "user@example.com");
        assert_eq!(calls[0].1, "Notification");
        assert_eq!(calls[0].2, "Hello!");
    }

    #[test]
    fn test_send_welcome_formats_name() {
        let mock = MockEmailSender::new();
        let service = NotificationService::new(mock);

        service.send_welcome("alice@example.com", "Alice").unwrap();

        let (_, _, body) = service.sender.last_call().unwrap();
        assert!(body.contains("Alice"));
    }

    #[test]
    fn test_failing_sender() {
        let mock = MockEmailSender::failing();
        let service = NotificationService::new(mock);

        let result = service.notify_user("user@example.com", "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_calls_recorded() {
        let mock = MockEmailSender::new();
        let service = NotificationService::new(mock);

        service.notify_user("a@example.com", "msg1").unwrap();
        service.notify_user("b@example.com", "msg2").unwrap();
        service.notify_user("c@example.com", "msg3").unwrap();

        assert_eq!(service.sender.call_count(), 3);
    }

    #[test]
    fn test_mock_default() {
        let mock = MockEmailSender::default();
        assert_eq!(mock.call_count(), 0);
        assert!(!mock.should_fail);
    }
}
