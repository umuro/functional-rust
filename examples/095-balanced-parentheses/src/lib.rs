//! # Balanced Parentheses
//!
//! Stack-based bracket matching. OCaml uses a list as a stack;
//! Rust uses `Vec<char>` the same way.

// ---------------------------------------------------------------------------
// Approach A: Imperative with Vec stack
// ---------------------------------------------------------------------------

pub fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let expected = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => unreachable!(),
                };
                if stack.pop() != Some(expected) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

// ---------------------------------------------------------------------------
// Approach B: Fold-based — functional style
// ---------------------------------------------------------------------------

pub fn is_balanced_fold(s: &str) -> bool {
    let result = s.chars().try_fold(Vec::new(), |mut stack, c| match c {
        '(' | '[' | '{' => {
            stack.push(c);
            Some(stack)
        }
        ')' | ']' | '}' => {
            let expected = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => unreachable!(),
            };
            if stack.pop() == Some(expected) {
                Some(stack)
            } else {
                None
            }
        }
        _ => Some(stack),
    });
    matches!(result, Some(s) if s.is_empty())
}

// ---------------------------------------------------------------------------
// Approach C: Recursive — mirrors OCaml directly
// ---------------------------------------------------------------------------

pub fn is_balanced_recursive(s: &str) -> bool {
    fn check(chars: &[char], stack: &[char], i: usize) -> bool {
        if i == chars.len() {
            return stack.is_empty();
        }
        match chars[i] {
            '(' | '[' | '{' => {
                let mut new_stack = stack.to_vec();
                new_stack.push(chars[i]);
                check(chars, &new_stack, i + 1)
            }
            ')' | ']' | '}' => {
                let expected = match chars[i] {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => unreachable!(),
                };
                match stack.last() {
                    Some(&top) if top == expected => {
                        let new_stack = &stack[..stack.len() - 1];
                        check(chars, new_stack, i + 1)
                    }
                    _ => false,
                }
            }
            _ => check(chars, stack, i + 1),
        }
    }
    let chars: Vec<char> = s.chars().collect();
    check(&chars, &[], 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        assert!(is_balanced("([]{})"));
        assert!(is_balanced("((()))"));
        assert!(is_balanced("[{()}]"));
    }

    #[test]
    fn test_unbalanced() {
        assert!(!is_balanced("([)]"));
        assert!(!is_balanced("("));
        assert!(!is_balanced(")"));
    }

    #[test]
    fn test_empty() {
        assert!(is_balanced(""));
    }

    #[test]
    fn test_with_other_chars() {
        assert!(is_balanced("(a + b) * [c - {d}]"));
    }

    #[test]
    fn test_fold_version() {
        assert!(is_balanced_fold("([]{})"));
        assert!(!is_balanced_fold("([)]"));
    }

    #[test]
    fn test_recursive_version() {
        assert!(is_balanced_recursive("([]{})"));
        assert!(!is_balanced_recursive("([)]"));
    }
}
