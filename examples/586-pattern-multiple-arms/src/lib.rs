//! # Multiple Arms Pattern Matching
//!
//! Consolidate match arms using OR patterns for cleaner code.

/// Token types for a simple expression language.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LParen,
    RParen,
    Num(i64),
    Ident(String),
}

/// Get token category using OR patterns to consolidate.
pub fn token_type(t: &Token) -> &'static str {
    match t {
        Token::Plus | Token::Minus | Token::Star | Token::Slash => "arithmetic",
        Token::Eq | Token::Ne | Token::Lt | Token::Le | Token::Gt | Token::Ge => "comparison",
        Token::LParen | Token::RParen => "bracket",
        Token::Num(_) => "number",
        Token::Ident(_) => "identifier",
    }
}

/// Get operator precedence.
pub fn precedence(t: &Token) -> i32 {
    match t {
        Token::Plus | Token::Minus => 1,
        Token::Star | Token::Slash => 2,
        Token::Eq | Token::Ne | Token::Lt | Token::Le | Token::Gt | Token::Ge => 0,
        _ => -1,
    }
}

/// Check if token is an operator.
pub fn is_operator(t: &Token) -> bool {
    matches!(
        t,
        Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Eq
            | Token::Ne
            | Token::Lt
            | Token::Le
            | Token::Gt
            | Token::Ge
    )
}

/// Check if token is binary arithmetic operator.
pub fn is_arithmetic(t: &Token) -> bool {
    matches!(t, Token::Plus | Token::Minus | Token::Star | Token::Slash)
}

/// Get operator symbol as string.
pub fn operator_symbol(t: &Token) -> Option<&'static str> {
    match t {
        Token::Plus => Some("+"),
        Token::Minus => Some("-"),
        Token::Star => Some("*"),
        Token::Slash => Some("/"),
        Token::Eq => Some("=="),
        Token::Ne => Some("!="),
        Token::Lt => Some("<"),
        Token::Le => Some("<="),
        Token::Gt => Some(">"),
        Token::Ge => Some(">="),
        _ => None,
    }
}

/// HTTP status code category using range patterns.
pub fn status_category(code: u16) -> &'static str {
    match code {
        100..=199 => "informational",
        200..=299 => "success",
        300..=399 => "redirection",
        400..=499 => "client error",
        500..=599 => "server error",
        _ => "unknown",
    }
}

/// Check if status indicates success.
pub fn is_success(code: u16) -> bool {
    matches!(code, 200..=299)
}

/// Check if status indicates an error.
pub fn is_error(code: u16) -> bool {
    matches!(code, 400..=599)
}

/// Day of week classification.
pub fn day_type(day: &str) -> &'static str {
    match day {
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        "Saturday" | "Sunday" => "weekend",
        _ => "unknown",
    }
}

/// Check if character is a vowel.
pub fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

/// Classify character type.
pub fn char_type(c: char) -> &'static str {
    match c {
        'a'..='z' | 'A'..='Z' => "letter",
        '0'..='9' => "digit",
        ' ' | '\t' | '\n' | '\r' => "whitespace",
        '!' | '?' | '.' | ',' | ';' | ':' => "punctuation",
        '+' | '-' | '*' | '/' | '=' | '<' | '>' => "operator",
        '(' | ')' | '[' | ']' | '{' | '}' => "bracket",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_arithmetic() {
        assert_eq!(token_type(&Token::Plus), "arithmetic");
        assert_eq!(token_type(&Token::Minus), "arithmetic");
        assert_eq!(token_type(&Token::Star), "arithmetic");
        assert_eq!(token_type(&Token::Slash), "arithmetic");
    }

    #[test]
    fn test_token_type_comparison() {
        assert_eq!(token_type(&Token::Eq), "comparison");
        assert_eq!(token_type(&Token::Ne), "comparison");
        assert_eq!(token_type(&Token::Lt), "comparison");
    }

    #[test]
    fn test_token_type_values() {
        assert_eq!(token_type(&Token::Num(42)), "number");
        assert_eq!(token_type(&Token::Ident("x".into())), "identifier");
    }

    #[test]
    fn test_precedence() {
        assert_eq!(precedence(&Token::Plus), 1);
        assert_eq!(precedence(&Token::Star), 2);
        assert_eq!(precedence(&Token::Eq), 0);
        assert_eq!(precedence(&Token::LParen), -1);
    }

    #[test]
    fn test_is_operator() {
        assert!(is_operator(&Token::Plus));
        assert!(is_operator(&Token::Eq));
        assert!(!is_operator(&Token::Num(1)));
        assert!(!is_operator(&Token::LParen));
    }

    #[test]
    fn test_is_arithmetic() {
        assert!(is_arithmetic(&Token::Plus));
        assert!(is_arithmetic(&Token::Star));
        assert!(!is_arithmetic(&Token::Eq));
        assert!(!is_arithmetic(&Token::Num(1)));
    }

    #[test]
    fn test_operator_symbol() {
        assert_eq!(operator_symbol(&Token::Plus), Some("+"));
        assert_eq!(operator_symbol(&Token::Eq), Some("=="));
        assert_eq!(operator_symbol(&Token::Num(1)), None);
    }

    #[test]
    fn test_status_category() {
        assert_eq!(status_category(100), "informational");
        assert_eq!(status_category(200), "success");
        assert_eq!(status_category(301), "redirection");
        assert_eq!(status_category(404), "client error");
        assert_eq!(status_category(500), "server error");
        assert_eq!(status_category(999), "unknown");
    }

    #[test]
    fn test_is_success_error() {
        assert!(is_success(200));
        assert!(is_success(201));
        assert!(!is_success(404));
        assert!(is_error(400));
        assert!(is_error(500));
        assert!(!is_error(200));
    }

    #[test]
    fn test_day_type() {
        assert_eq!(day_type("Monday"), "weekday");
        assert_eq!(day_type("Friday"), "weekday");
        assert_eq!(day_type("Saturday"), "weekend");
        assert_eq!(day_type("Sunday"), "weekend");
        assert_eq!(day_type("Holiday"), "unknown");
    }

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('E'));
        assert!(!is_vowel('b'));
        assert!(!is_vowel('z'));
    }

    #[test]
    fn test_char_type() {
        assert_eq!(char_type('a'), "letter");
        assert_eq!(char_type('Z'), "letter");
        assert_eq!(char_type('5'), "digit");
        assert_eq!(char_type(' '), "whitespace");
        assert_eq!(char_type('!'), "punctuation");
        assert_eq!(char_type('+'), "operator");
        assert_eq!(char_type('('), "bracket");
        assert_eq!(char_type('@'), "other");
    }
}
