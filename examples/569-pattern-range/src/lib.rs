//! Range Patterns
//!
//! Matching ranges with ..= syntax.

/// Match numeric ranges.
pub fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

/// Match character ranges.
pub fn char_type(c: char) -> &'static str {
    match c {
        'a'..='z' => "lowercase",
        'A'..='Z' => "uppercase",
        '0'..='9' => "digit",
        _ => "other",
    }
}

/// Inclusive range with guard.
pub fn categorize(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0 => "zero",
        1..=100 => "small positive",
        101..=i32::MAX => "large positive",
    }
}

/// Range in struct field.
pub struct Temperature(pub i32);

pub fn temp_status(t: &Temperature) -> &'static str {
    match t.0 {
        ..=-10 => "freezing",
        -9..=0 => "cold",
        1..=20 => "cool",
        21..=30 => "warm",
        31.. => "hot",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), 'A');
        assert_eq!(grade(85), 'B');
        assert_eq!(grade(55), 'F');
    }

    #[test]
    fn test_char_type() {
        assert_eq!(char_type('a'), "lowercase");
        assert_eq!(char_type('Z'), "uppercase");
        assert_eq!(char_type('5'), "digit");
    }

    #[test]
    fn test_categorize() {
        assert_eq!(categorize(-5), "negative");
        assert_eq!(categorize(0), "zero");
        assert_eq!(categorize(50), "small positive");
    }

    #[test]
    fn test_temp() {
        assert_eq!(temp_status(&Temperature(-20)), "freezing");
        assert_eq!(temp_status(&Temperature(25)), "warm");
    }
}
