#![allow(clippy::all)]
// 1039: Stack Using Vec
// Vec's push/pop operate at the end — perfect LIFO stack

/// A simple stack wrapper around Vec
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.items.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

fn basic_stack() {
    let mut s = Stack::new();
    assert!(s.is_empty());

    s.push(10);
    s.push(20);
    s.push(30);

    assert_eq!(s.size(), 3);
    assert_eq!(s.peek(), Some(&30));
    assert_eq!(s.pop(), Some(30));
    assert_eq!(s.pop(), Some(20));
    assert_eq!(s.pop(), Some(10));
    assert_eq!(s.pop(), None);
}

/// Vec directly as a stack (no wrapper needed)
fn vec_as_stack() {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.last(), Some(&3));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.len(), 2);
}

/// RPN (Reverse Polish Notation) calculator using a stack
fn eval_rpn(tokens: &[&str]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for &token in tokens {
        match token {
            "+" | "-" | "*" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            n => stack.push(n.parse().unwrap()),
        }
    }

    stack.pop().unwrap()
}

fn eval_test() {
    // 3 4 + 2 * = (3 + 4) * 2 = 14
    assert_eq!(eval_rpn(&["3", "4", "+", "2", "*"]), 14);
    // 5 1 2 + 4 * + 3 - = 5 + (1+2)*4 - 3 = 14
    assert_eq!(eval_rpn(&["5", "1", "2", "+", "4", "*", "+", "3", "-"]), 14);
}

/// Balanced parentheses checker
fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for ch in s.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn balance_test() {
    assert!(is_balanced("({[]})"));
    assert!(is_balanced(""));
    assert!(!is_balanced("({[})"));
    assert!(!is_balanced("(("));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        basic_stack();
    }

    #[test]
    fn test_vec_stack() {
        vec_as_stack();
    }

    #[test]
    fn test_rpn() {
        eval_test();
    }

    #[test]
    fn test_balanced() {
        balance_test();
    }
}
