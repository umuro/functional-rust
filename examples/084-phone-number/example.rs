/// Phone Number Parser — Validation Pipeline
///
/// Ownership: Input is borrowed &str. Result returns owned String on success.
/// The and_then chain mirrors OCaml's Result.bind pipeline.

/// Extract only digits from input
fn digits_only(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Validate a phone number using Result chaining
pub fn validate(s: &str) -> Result<String, &'static str> {
    let d = digits_only(s);

    // Normalize 11-digit numbers starting with 1
    let d = if d.len() == 11 && d.starts_with('1') {
        d[1..].to_string()
    } else if d.len() == 10 {
        d
    } else {
        return Err("wrong number of digits");
    };

    // Validate area code
    let area = d.as_bytes()[0];
    if area == b'0' || area == b'1' {
        return Err("invalid area code");
    }

    // Validate exchange
    let exchange = d.as_bytes()[3];
    if exchange == b'0' || exchange == b'1' {
        return Err("invalid exchange");
    }

    Ok(d)
}

/// Version 2: Using and_then chain (more functional)
pub fn validate_chain(s: &str) -> Result<String, &'static str> {
    let d = digits_only(s);

    normalize_length(d)
        .and_then(check_area_code)
        .and_then(check_exchange)
}

fn normalize_length(d: String) -> Result<String, &'static str> {
    match d.len() {
        11 if d.starts_with('1') => Ok(d[1..].to_string()),
        10 => Ok(d),
        _ => Err("wrong number of digits"),
    }
}

fn check_area_code(d: String) -> Result<String, &'static str> {
    if d.as_bytes()[0] == b'0' || d.as_bytes()[0] == b'1' {
        Err("invalid area code")
    } else {
        Ok(d)
    }
}

fn check_exchange(d: String) -> Result<String, &'static str> {
    if d.as_bytes()[3] == b'0' || d.as_bytes()[3] == b'1' {
        Err("invalid exchange")
    } else {
        Ok(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_10_digit() {
        assert_eq!(validate("(223) 456-7890"), Ok("2234567890".into()));
    }

    #[test]
    fn test_valid_11_digit() {
        assert_eq!(validate("1-223-456-7890"), Ok("2234567890".into()));
    }

    #[test]
    fn test_invalid_area_code() {
        assert_eq!(validate("(023) 456-7890"), Err("invalid area code"));
    }

    #[test]
    fn test_invalid_exchange() {
        assert_eq!(validate("(223) 056-7890"), Err("invalid exchange"));
    }

    #[test]
    fn test_wrong_length() {
        assert_eq!(validate("123"), Err("wrong number of digits"));
    }

    #[test]
    fn test_chain_version() {
        assert_eq!(validate_chain("(223) 456-7890"), Ok("2234567890".into()));
        assert_eq!(validate_chain("(023) 456-7890"), Err("invalid area code"));
    }
}

fn main() {
    println!("{:?}", validate("(223) 456-7890"), Ok("2234567890".into()));
    println!("{:?}", validate("1-223-456-7890"), Ok("2234567890".into()));
    println!("{:?}", validate("(023) 456-7890"), Err("invalid area code"));
}
