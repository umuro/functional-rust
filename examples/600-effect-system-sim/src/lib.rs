//! # Effect System Simulation
//!
//! Simulate algebraic effects using Result and continuation passing.

/// Effect type for IO operations.
#[derive(Debug, Clone)]
pub enum IoEffect {
    Print(String),
    Read,
}

/// Effect type for state operations.
#[derive(Debug, Clone)]
pub enum StateEffect<S> {
    Get,
    Put(S),
}

/// Simple effect handler using callbacks.
pub fn handle_io<T>(effect: IoEffect, on_print: impl Fn(&str) -> T, on_read: impl Fn() -> T) -> T {
    match effect {
        IoEffect::Print(s) => on_print(&s),
        IoEffect::Read => on_read(),
    }
}

/// State handler.
pub fn handle_state<S: Clone, T>(
    effect: StateEffect<S>,
    state: &mut S,
    on_get: impl Fn(&S) -> T,
    on_put: impl Fn() -> T,
) -> T {
    match effect {
        StateEffect::Get => on_get(state),
        StateEffect::Put(new_state) => {
            *state = new_state;
            on_put()
        }
    }
}

/// Run a stateful computation.
pub fn run_state<S: Clone, T>(init: S, f: impl FnOnce(&mut S) -> T) -> (T, S) {
    let mut state = init;
    let result = f(&mut state);
    (result, state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_io() {
        let result = handle_io(IoEffect::Print("hello".into()), |s| s.len(), || 0);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_run_state() {
        let (result, final_state) = run_state(0i32, |s| {
            *s += 10;
            *s *= 2;
            *s
        });
        assert_eq!(result, 20);
        assert_eq!(final_state, 20);
    }
}
