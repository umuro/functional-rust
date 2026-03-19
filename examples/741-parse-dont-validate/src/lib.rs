/// 741: Parse-Don't-Validate
/// Types that can ONLY be created via parsing. Once created, always valid.

// ── Error types ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidEmail(String),
    OutOfRange { value: i64, lo: i64, hi: i64 },
    InvalidChar(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyString => write!(f, "string is empty"),
            ParseError::InvalidEmail(s) => write!(f, "'{}' is not a valid email", s),
            ParseError::OutOfRange { value, lo, hi } => {
                write!(f, "{} not in range [{}, {}]", value, lo, hi)
            }
            ParseError::InvalidChar(c) => write!(f, "invalid character '{}'", c),
        }
    }
}

// ── NonEmptyString ────────────────────────────────────────────────────────────

/// A string guaranteed to be non-empty. Private field prevents direct construction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        if s.is_empty() {
            return Err(ParseError::EmptyString);
        }
        Ok(NonEmptyString(s.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl std::fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ── Email ─────────────────────────────────────────────────────────────────────

/// A validated email address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let at = s
            .find('@')
            .ok_or_else(|| ParseError::InvalidEmail(s.to_owned()))?;
        let (local, domain) = s.split_at(at);
        let domain = &domain[1..]; // skip '@'
        if local.is_empty() || !domain.contains('.') || domain.starts_with('.') {
            return Err(ParseError::InvalidEmail(s.to_owned()));
        }
        Ok(Email(s.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap()
    }
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap()
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// ── BoundedInt ────────────────────────────────────────────────────────────────

/// An integer constrained to [LO, HI].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundedInt<const LO: i64, const HI: i64>(i64);

impl<const LO: i64, const HI: i64> BoundedInt<LO, HI> {
    pub fn parse(n: i64) -> Result<Self, ParseError> {
        if n < LO || n > HI {
            return Err(ParseError::OutOfRange {
                value: n,
                lo: LO,
                hi: HI,
            });
        }
        Ok(BoundedInt(n))
    }

    pub fn value(self) -> i64 {
        self.0
    }
}

// ── Functions that REQUIRE parsed types ───────────────────────────────────────

/// This function only accepts valid emails — no runtime checks needed inside.
fn send_welcome(email: &Email) -> String {
    format!("Welcome email sent to {}", email)
}

/// Only accepts non-empty usernames — no `if name.is_empty()` guards needed.
fn create_account(username: &NonEmptyString, email: &Email) -> String {
    format!("Account '{}' created with email {}", username, email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_email_parses() {
        let e = Email::parse("user@example.com").unwrap();
        assert_eq!(e.domain(), "example.com");
        assert_eq!(e.local_part(), "user");
    }

    #[test]
    fn email_normalized_to_lowercase() {
        let e = Email::parse("USER@EXAMPLE.COM").unwrap();
        assert_eq!(e.as_str(), "user@example.com");
    }

    #[test]
    fn invalid_emails_rejected() {
        assert!(Email::parse("").is_err());
        assert!(Email::parse("noatsign").is_err());
        assert!(Email::parse("@nodomain").is_err());
        assert!(Email::parse("user@nodot").is_err());
    }

    #[test]
    fn non_empty_string_valid() {
        let s = NonEmptyString::parse("hello").unwrap();
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn non_empty_string_rejects_empty() {
        assert_eq!(NonEmptyString::parse(""), Err(ParseError::EmptyString));
    }

    #[test]
    fn bounded_int_valid() {
        type Score = BoundedInt<0, 10>;
        assert_eq!(Score::parse(5).unwrap().value(), 5);
        assert_eq!(Score::parse(0).unwrap().value(), 0);
        assert_eq!(Score::parse(10).unwrap().value(), 10);
    }

    #[test]
    fn bounded_int_out_of_range() {
        type Score = BoundedInt<0, 10>;
        assert!(Score::parse(-1).is_err());
        assert!(Score::parse(11).is_err());
    }
}
