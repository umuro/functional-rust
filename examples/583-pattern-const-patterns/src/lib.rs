//! # Constant Patterns
//!
//! Use named constants in pattern matching for cleaner, more maintainable code.

/// Age thresholds as constants.
pub const MIN_AGE: u32 = 18;
pub const MAX_AGE: u32 = 65;

/// Common port numbers.
pub const HTTP: u16 = 80;
pub const HTTPS: u16 = 443;
pub const ADMIN: u16 = 8080;

/// Classify age using constant patterns.
pub fn classify_age(age: u32) -> &'static str {
    match age {
        0 => "newborn",
        1..=17 => "minor",
        MIN_AGE..=MAX_AGE => "adult",
        _ => "senior",
    }
}

/// Alternative without constants (less maintainable).
pub fn classify_age_literal(age: u32) -> &'static str {
    match age {
        0 => "newborn",
        1..=17 => "minor",
        18..=65 => "adult",
        _ => "senior",
    }
}

/// Describe a port number using constant patterns.
pub fn describe_port(p: u16) -> &'static str {
    match p {
        HTTP => "HTTP",
        HTTPS => "HTTPS",
        ADMIN => "Admin",
        1..=1023 => "well-known",
        1024..=49151 => "registered",
        _ => "dynamic",
    }
}

/// Configuration with associated constants.
pub struct Config;

impl Config {
    pub const TIMEOUT: u32 = 30;
    pub const MAX_RETRIES: u32 = 3;
}

/// Classify timeout using associated constants.
pub fn classify_timeout(t: u32) -> &'static str {
    match t {
        0 => "none",
        Config::TIMEOUT => "default",
        1..=10 => "fast",
        _ => "slow",
    }
}

/// HTTP status code ranges as constants.
pub const INFO_START: u16 = 100;
pub const SUCCESS_START: u16 = 200;
pub const REDIRECT_START: u16 = 300;
pub const CLIENT_ERROR_START: u16 = 400;
pub const SERVER_ERROR_START: u16 = 500;

/// Classify HTTP status code.
pub fn http_status_class(code: u16) -> &'static str {
    match code {
        INFO_START..=199 => "informational",
        SUCCESS_START..=299 => "success",
        REDIRECT_START..=399 => "redirect",
        CLIENT_ERROR_START..=499 => "client error",
        SERVER_ERROR_START..=599 => "server error",
        _ => "unknown",
    }
}

/// Character class constants.
pub const NEWLINE: char = '\n';
pub const TAB: char = '\t';
pub const SPACE: char = ' ';

/// Classify whitespace characters.
pub fn classify_whitespace(c: char) -> &'static str {
    match c {
        NEWLINE => "newline",
        TAB => "tab",
        SPACE => "space",
        '\r' => "carriage return",
        _ if c.is_whitespace() => "other whitespace",
        _ => "not whitespace",
    }
}

/// Match on specific ASCII values.
pub fn describe_byte(b: u8) -> &'static str {
    const NUL: u8 = 0;
    const BELL: u8 = 7;
    const BACKSPACE: u8 = 8;

    match b {
        NUL => "null",
        BELL => "bell",
        BACKSPACE => "backspace",
        b'A'..=b'Z' => "uppercase letter",
        b'a'..=b'z' => "lowercase letter",
        b'0'..=b'9' => "digit",
        32..=126 => "printable",
        _ => "control/extended",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_age() {
        assert_eq!(classify_age(0), "newborn");
        assert_eq!(classify_age(10), "minor");
        assert_eq!(classify_age(17), "minor");
        assert_eq!(classify_age(18), "adult");
        assert_eq!(classify_age(40), "adult");
        assert_eq!(classify_age(65), "adult");
        assert_eq!(classify_age(66), "senior");
    }

    #[test]
    fn test_classify_age_approaches_equivalent() {
        for age in 0..=100 {
            assert_eq!(classify_age(age), classify_age_literal(age));
        }
    }

    #[test]
    fn test_describe_port() {
        assert_eq!(describe_port(80), "HTTP");
        assert_eq!(describe_port(443), "HTTPS");
        assert_eq!(describe_port(8080), "Admin");
        assert_eq!(describe_port(22), "well-known");
        assert_eq!(describe_port(3000), "registered");
        assert_eq!(describe_port(50000), "dynamic");
    }

    #[test]
    fn test_classify_timeout() {
        assert_eq!(classify_timeout(0), "none");
        assert_eq!(classify_timeout(5), "fast");
        assert_eq!(classify_timeout(30), "default");
        assert_eq!(classify_timeout(60), "slow");
    }

    #[test]
    fn test_http_status_class() {
        assert_eq!(http_status_class(100), "informational");
        assert_eq!(http_status_class(200), "success");
        assert_eq!(http_status_class(301), "redirect");
        assert_eq!(http_status_class(404), "client error");
        assert_eq!(http_status_class(500), "server error");
        assert_eq!(http_status_class(600), "unknown");
    }

    #[test]
    fn test_classify_whitespace() {
        assert_eq!(classify_whitespace('\n'), "newline");
        assert_eq!(classify_whitespace('\t'), "tab");
        assert_eq!(classify_whitespace(' '), "space");
        assert_eq!(classify_whitespace('a'), "not whitespace");
    }

    #[test]
    fn test_describe_byte() {
        assert_eq!(describe_byte(0), "null");
        assert_eq!(describe_byte(7), "bell");
        assert_eq!(describe_byte(b'A'), "uppercase letter");
        assert_eq!(describe_byte(b'z'), "lowercase letter");
        assert_eq!(describe_byte(b'5'), "digit");
        assert_eq!(describe_byte(b'!'), "printable");
    }
}
