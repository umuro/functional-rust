/// # Balanced Parentheses
///
/// Stack-based bracket matching using Vec as a stack.
/// Supports (), [], {}.

/// Idiomatic Rust with a Vec as stack.
pub fn is_balanced(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') { return false; }
            }
            ']' => {
                if stack.pop() != Some('[') { return false; }
            }
            '}' => {
                if stack.pop() != Some('{') { return false; }
            }
            _ => {} // ignore other characters
        }
    }
    stack.is_empty()
}

/// Recursive approach using a slice as an immutable stack (functional style).
pub fn is_balanced_recursive(s: &str) -> bool {
    fn matching(c: char) -> char {
        match c {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => ' ',
        }
    }

    fn check(chars: &[char], stack: &[char]) -> bool {
        match chars.first() {
            None => stack.is_empty(),
            Some(&c) => match c {
                '(' | '[' | '{' => {
                    let mut new_stack = stack.to_vec();
                    new_stack.push(c);
                    check(&chars[1..], &new_stack)
                }
                ')' | ']' | '}' => {
                    match stack.last() {
                        Some(&top) if top == matching(c) => {
                            let new_stack = &stack[..stack.len() - 1];
                            check(&chars[1..], new_stack)
                        }
                        _ => false,
                    }
                }
                _ => check(&chars[1..], stack),
            }
        }
    }

    let chars: Vec<char> = s.chars().collect();
    check(&chars, &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        assert!(is_balanced("([]{})"));
        assert!(is_balanced("((()))"));
        assert!(is_balanced("[{()}]"));
        assert!(is_balanced(""));
    }

    #[test]
    fn test_unbalanced() {
        assert!(!is_balanced("([)]"));
        assert!(!is_balanced("("));
        assert!(!is_balanced(")"));
        assert!(!is_balanced("((())"));
    }

    #[test]
    fn test_with_other_chars() {
        assert!(is_balanced("a(b[c]d)e"));
    }

    #[test]
    fn test_recursive() {
        assert!(is_balanced_recursive("([]{})"));
        assert!(!is_balanced_recursive("([)]"));
        assert!(is_balanced_recursive(""));
    }
}
