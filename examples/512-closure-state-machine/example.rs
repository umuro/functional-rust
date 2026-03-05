//! # 512. Closures as State Machine Transitions
//! Pattern: a*b+ — zero-or-more 'a' followed by one-or-more 'b'

/// Enum to represent state machine outputs
enum StateResult {
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
fn state_start(c: char) -> StateResult {
    match c {
        'a' => StateResult::Continue(Box::new(state_after_a)),
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _   => StateResult::Reject,
    }
}

/// State: seen at least one 'a', expecting more 'a' or 'b'
fn state_after_a(c: char) -> StateResult {
    match c {
        'a' => StateResult::Continue(Box::new(state_after_a)),
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _   => StateResult::Reject,
    }
}

/// State: in 'b' sequence — only accepts more 'b'
fn state_after_b(c: char) -> StateResult {
    match c {
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _   => StateResult::Reject,
    }
}

/// Run the state machine over an input string
/// Accept condition: ended in 'b' state (pattern: a*b+)
fn run_machine(input: &str) -> bool {
    let mut chars = input.chars();
    let first = match chars.next() {
        None => return false,
        Some(c) => c,
    };

    let initial = state_start(first);
    let final_state = chars.fold(initial, |state, c| {
        match state {
            StateResult::Continue(f) => f(c),
            other => other, // Reject stays Reject
        }
    });

    // Accept: we ended in the 'b' state — check via Continue(state_after_b)
    // Since we can't inspect closures, we track acceptance differently:
    // a*b+ is accepted if last char was 'b' and pattern was valid
    matches!(final_state, StateResult::Continue(_))
        && input.ends_with('b')
        && !matches!(final_state, StateResult::Reject)
}

/// Alternative: explicit state enum + closure transitions
#[derive(Clone, Debug, PartialEq)]
enum LexState { Start, InA, InB, Error }

fn make_lexer() -> impl FnMut(char) -> LexState {
    let mut state = LexState::Start;
    move |c: char| {
        state = match (&state, c) {
            (LexState::Start, 'a') => LexState::InA,
            (LexState::Start, 'b') => LexState::InB,
            (LexState::InA,   'a') => LexState::InA,
            (LexState::InA,   'b') => LexState::InB,
            (LexState::InB,   'b') => LexState::InB,
            _                      => LexState::Error,
        };
        state.clone()
    }
}

fn accepts_pattern(input: &str) -> bool {
    if input.is_empty() { return false; }
    let mut lexer = make_lexer();
    let last_state = input.chars().map(|c| lexer(c)).last();
    last_state == Some(LexState::InB)
}

fn main() {
    let tests = ["b", "ab", "aab", "abb", "aabb", "", "a", "ba", "abc", "bbb"];
    println!("Pattern: a*b+ (zero-or-more 'a', one-or-more 'b')");
    println!("{:<12} {:<12} {}", "Input", "Closure SM", "Enum SM");
    for s in &tests {
        println!("{:<12} {:<12} {}",
            format!("{:?}", s),
            run_machine(s),
            accepts_pattern(s));
    }

    // Demonstrate stateful lexer
    println!("\nStateful lexer trace of \"aabb\":");
    let mut lexer = make_lexer();
    for c in "aabb".chars() {
        println!("  '{}' -> {:?}", c, lexer(c));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepts_pattern() {
        assert!(accepts_pattern("b"));
        assert!(accepts_pattern("ab"));
        assert!(accepts_pattern("aab"));
        assert!(accepts_pattern("abb"));
        assert!(accepts_pattern("bbb"));
    }

    #[test]
    fn test_rejects_pattern() {
        assert!(!accepts_pattern(""));
        assert!(!accepts_pattern("a"));
        assert!(!accepts_pattern("ba"));
        assert!(!accepts_pattern("abc"));
    }

    #[test]
    fn test_stateful_lexer() {
        let mut lex = make_lexer();
        assert_eq!(lex('a'), LexState::InA);
        assert_eq!(lex('b'), LexState::InB);
        assert_eq!(lex('b'), LexState::InB);
        assert_eq!(lex('a'), LexState::Error);
    }
}
