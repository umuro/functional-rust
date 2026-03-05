/// 751: Mocking via Traits — test doubles without external crates

use std::cell::RefCell;

// ── The dependency trait ───────────────────────────────────────────────────────

pub trait EmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

// ── Real implementation (production) ─────────────────────────────────────────

pub struct SmtpSender {
    pub host: String,
    pub port: u16,
}

impl EmailSender for SmtpSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        println!("[SMTP {}:{}] To={} Subject={} Body={}",
            self.host, self.port, to, subject, &body[..body.len().min(50)]);
        Ok(())
    }
}

// ── Mock implementation (tests) ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SentEmail {
    pub to:      String,
    pub subject: String,
    pub body:    String,
}

pub struct MockEmailSender {
    sent:        RefCell<Vec<SentEmail>>,
    should_fail: bool,
}

impl MockEmailSender {
    pub fn new() -> Self {
        MockEmailSender { sent: RefCell::new(Vec::new()), should_fail: false }
    }

    pub fn failing() -> Self {
        MockEmailSender { sent: RefCell::new(Vec::new()), should_fail: true }
    }

    pub fn sent_count(&self) -> usize {
        self.sent.borrow().len()
    }

    pub fn get_sent(&self) -> std::cell::Ref<Vec<SentEmail>> {
        self.sent.borrow()
    }

    pub fn last_sent(&self) -> Option<SentEmail> {
        self.sent.borrow().last().cloned()
    }
}

impl EmailSender for MockEmailSender {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        if self.should_fail {
            return Err("mock SMTP failure".to_owned());
        }
        self.sent.borrow_mut().push(SentEmail {
            to:      to.to_owned(),
            subject: subject.to_owned(),
            body:    body.to_owned(),
        });
        Ok(())
    }
}

// ── Business logic (depends on EmailSender trait) ─────────────────────────────

pub struct UserService<S: EmailSender> {
    sender: S,
}

impl<S: EmailSender> UserService<S> {
    pub fn new(sender: S) -> Self { UserService { sender } }

    pub fn welcome_user(&self, email: &str, name: &str) -> Result<(), String> {
        self.sender.send(
            email,
            "Welcome!",
            &format!("Hi {}, welcome aboard! We're glad to have you.", name),
        )
    }

    pub fn password_reset(&self, email: &str, token: &str) -> Result<(), String> {
        self.sender.send(
            email,
            "Password Reset",
            &format!("Your reset token: {}", token),
        )
    }
}

fn main() {
    let smtp = SmtpSender { host: "smtp.example.com".into(), port: 587 };
    let svc  = UserService::new(smtp);
    svc.welcome_user("alice@example.com", "Alice").unwrap();
    svc.password_reset("alice@example.com", "RESET-TOKEN-123").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_user_sends_email_with_correct_fields() {
        // Arrange
        let mock = MockEmailSender::new();
        let svc  = UserService::new(mock);

        // Act
        svc.welcome_user("alice@example.com", "Alice").unwrap();

        // Assert
        let sent = svc.sender.get_sent();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].to, "alice@example.com");
        assert_eq!(sent[0].subject, "Welcome!");
        assert!(sent[0].body.contains("Alice"));
    }

    #[test]
    fn welcome_user_sends_exactly_one_email() {
        let mock = MockEmailSender::new();
        let svc  = UserService::new(mock);
        svc.welcome_user("bob@example.com", "Bob").unwrap();
        assert_eq!(svc.sender.sent_count(), 1);
    }

    #[test]
    fn password_reset_includes_token_in_body() {
        let mock = MockEmailSender::new();
        let svc  = UserService::new(mock);
        svc.password_reset("user@example.com", "SECRET-42").unwrap();
        let email = svc.sender.last_sent().unwrap();
        assert!(email.body.contains("SECRET-42"), "token missing: {}", email.body);
    }

    #[test]
    fn failing_sender_propagates_error() {
        let mock = MockEmailSender::failing();
        let svc  = UserService::new(mock);
        let result = svc.welcome_user("user@example.com", "User");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("failure"));
    }

    #[test]
    fn multiple_calls_all_recorded() {
        let mock = MockEmailSender::new();
        let svc  = UserService::new(mock);
        svc.welcome_user("a@a.com", "A").unwrap();
        svc.welcome_user("b@b.com", "B").unwrap();
        svc.password_reset("c@c.com", "T").unwrap();
        assert_eq!(svc.sender.sent_count(), 3);
    }
}
