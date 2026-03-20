#![allow(clippy::all)]
//! Closures as State Machine Transitions
//!
//! Pattern: a*b+ — zero-or-more 'a' followed by one-or-more 'b'

/// State machine result.
pub enum StateResult {
    Accept,
    Reject,
    Continue(Box<dyn Fn(char) -> StateResult>),
}

impl std::fmt::Debug for StateResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateResult::Accept => write!(f, "Accept"),
            StateResult::Reject => write!(f, "Reject"),
            StateResult::Continue(_) => write!(f, "Continue(<fn>)"),
        }
    }
}

/// State: start — expects 'a' or 'b'
pub fn state_start(c: char) -> StateResult {
    match c {
        'a' => StateResult::Continue(Box::new(state_after_a)),
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _ => StateResult::Reject,
    }
}

/// State: seen at least one 'a', expecting more 'a' or 'b'
pub fn state_after_a(c: char) -> StateResult {
    match c {
        'a' => StateResult::Continue(Box::new(state_after_a)),
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _ => StateResult::Reject,
    }
}

/// State: seen at least one 'b' after a's — expecting more 'b' or end
pub fn state_after_b(c: char) -> StateResult {
    match c {
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _ => StateResult::Reject,
    }
}

/// Run the state machine on input.
pub fn run_machine(input: &str) -> bool {
    let mut state: Box<dyn Fn(char) -> StateResult> = Box::new(state_start);
    let mut saw_b = false;

    for c in input.chars() {
        match state(c) {
            StateResult::Accept => return true,
            StateResult::Reject => return false,
            StateResult::Continue(next) => {
                if c == 'b' {
                    saw_b = true;
                }
                state = next;
            }
        }
    }

    // Must have seen at least one 'b' to accept
    saw_b
}

/// Alternative: enum-based state machine (more idiomatic Rust)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Start,
    AfterA,
    AfterB,
    Reject,
}

pub fn transition(state: State, c: char) -> State {
    match (state, c) {
        (State::Start, 'a') => State::AfterA,
        (State::Start, 'b') => State::AfterB,
        (State::AfterA, 'a') => State::AfterA,
        (State::AfterA, 'b') => State::AfterB,
        (State::AfterB, 'b') => State::AfterB,
        _ => State::Reject,
    }
}

pub fn run_enum_machine(input: &str) -> bool {
    let final_state = input.chars().fold(State::Start, transition);
    final_state == State::AfterB
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepts_b() {
        assert!(run_machine("b"));
    }

    #[test]
    fn test_accepts_ab() {
        assert!(run_machine("ab"));
    }

    #[test]
    fn test_accepts_aab() {
        assert!(run_machine("aab"));
    }

    #[test]
    fn test_accepts_aaabbb() {
        assert!(run_machine("aaabbb"));
    }

    #[test]
    fn test_rejects_a_only() {
        assert!(!run_machine("a"));
        assert!(!run_machine("aaa"));
    }

    #[test]
    fn test_rejects_empty() {
        assert!(!run_machine(""));
    }

    #[test]
    fn test_enum_machine_matches() {
        for input in &["b", "ab", "aab", "aaabbb", "bbb"] {
            assert_eq!(run_machine(input), run_enum_machine(input));
        }
        for input in &["a", "aaa", "", "ba", "aba"] {
            assert_eq!(run_machine(input), run_enum_machine(input));
        }
    }
}
