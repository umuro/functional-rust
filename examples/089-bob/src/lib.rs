/// Bob — String Pattern Matching
///
/// Ownership: Input is borrowed &str. Responses are &'static str (no allocation).

fn is_question(s: &str) -> bool {
    s.trim().ends_with('?')
}

fn is_yelling(s: &str) -> bool {
    let has_letter = s.chars().any(|c| c.is_alphabetic());
    has_letter && s == s.to_uppercase()
}

fn is_silence(s: &str) -> bool {
    s.trim().is_empty()
}

pub fn response_for(s: &str) -> &'static str {
    match (is_silence(s), is_yelling(s), is_question(s)) {
        (true, _, _) => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, true, false) => "Whoa, chill out!",
        (_, false, true) => "Sure.",
        _ => "Whatever.",
    }
}

/// Version 2: Using if-else chain (more readable for some)
pub fn response_for_v2(s: &str) -> &'static str {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if is_yelling(s) && is_question(s) {
        "Calm down, I know what I'm doing!"
    } else if is_yelling(s) {
        "Whoa, chill out!"
    } else if is_question(s) {
        "Sure."
    } else {
        "Whatever."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yelling() {
        assert_eq!(response_for("WATCH OUT!"), "Whoa, chill out!");
    }

    #[test]
    fn test_question() {
        assert_eq!(response_for("Does this work?"), "Sure.");
    }

    #[test]
    fn test_yelling_question() {
        assert_eq!(
            response_for("WHAT IS THIS?"),
            "Calm down, I know what I'm doing!"
        );
    }

    #[test]
    fn test_silence() {
        assert_eq!(response_for("   "), "Fine. Be that way!");
    }

    #[test]
    fn test_normal() {
        assert_eq!(response_for("Hi"), "Whatever.");
    }

    #[test]
    fn test_v2_matches() {
        for s in &[
            "WATCH OUT!",
            "Does this work?",
            "WHAT IS THIS?",
            "   ",
            "Hi",
        ] {
            assert_eq!(response_for(s), response_for_v2(s));
        }
    }
}
