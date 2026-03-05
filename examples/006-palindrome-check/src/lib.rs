// 006: Palindrome Check

// Approach 1: Reverse and compare (allocating)
fn is_palindrome_rev(s: &str) -> bool {
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

// Approach 2: Iterator comparison (zero allocation)
fn is_palindrome_iter(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Approach 3: Case-insensitive, alphanumeric only
fn is_palindrome_clean(s: &str) -> bool {
    let clean: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    clean.iter().eq(clean.iter().rev())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_rev() {
        assert!(is_palindrome_rev("racecar"));
        assert!(is_palindrome_rev("abba"));
        assert!(!is_palindrome_rev("hello"));
        assert!(is_palindrome_rev(""));
        assert!(is_palindrome_rev("a"));
    }

    #[test]
    fn test_palindrome_iter() {
        assert!(is_palindrome_iter("racecar"));
        assert!(!is_palindrome_iter("abc"));
    }

    #[test]
    fn test_palindrome_clean() {
        assert!(is_palindrome_clean("A man, a plan, a canal: Panama"));
        assert!(is_palindrome_clean("Race Car"));
        assert!(!is_palindrome_clean("hello world"));
    }
}
