#![allow(clippy::all)]
// Example 257: State Machine — Turnstile
//
// A turnstile has two states (Locked, Unlocked) and two events (Coin, Push).
// OCaml encodes this with `type` declarations and a `match` on a tuple.
// Rust encodes it with `enum` + `impl` methods, giving the state machine
// behaviour directly on the type rather than as a free function.

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — method on enum
//
// Defining `transition` as a method on `State` is idiomatic Rust:
// behaviour lives with the type.  The compiler exhaustively checks every
// (State, Event) combination at compile time — the same guarantee OCaml's
// pattern-match exhaustiveness check provides.
// ---------------------------------------------------------------------------

/// The two states a turnstile can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Locked,
    Unlocked,
}

/// Events that can be applied to a turnstile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Coin,
    Push,
}

impl State {
    /// Transition to the next state given an event.
    ///
    /// Mirrors the OCaml:
    ///   `let transition state event = match state, event with ...`
    ///
    /// Using `(self, event)` as the match discriminant mirrors OCaml's
    /// tuple pattern exactly.
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (Self::Locked, Event::Coin) => Self::Unlocked,
            (Self::Unlocked, Event::Push) => Self::Locked,
            // These arms keep the machine in its current state:
            (Self::Locked, Event::Push) => Self::Locked,
            (Self::Unlocked, Event::Coin) => Self::Unlocked,
        }
    }

    /// Human-readable name for display.
    pub fn name(self) -> &'static str {
        match self {
            Self::Locked => "Locked",
            Self::Unlocked => "Unlocked",
        }
    }
}

/// Run a sequence of events from `initial`, returning the final state.
/// Collects each (before, after) pair for inspection.
pub fn run_machine(initial: State, events: &[Event]) -> (State, Vec<(State, State)>) {
    let mut transitions = Vec::with_capacity(events.len());
    // Iterator fold mirrors OCaml's `List.fold_left`
    let final_state = events.iter().fold(initial, |s, &e| {
        let next = s.transition(e);
        transitions.push((s, next));
        next
    });
    (final_state, transitions)
}

// ---------------------------------------------------------------------------
// Solution 2: Functional style — free function mirrors OCaml directly
//
// OCaml uses a free `transition` function; this mirrors that style.
// In Rust, free functions and methods are equally idiomatic; here the
// free function makes the OCaml parallel explicit.
// ---------------------------------------------------------------------------

/// Free-function transition — mirrors OCaml's `let transition state event`.
pub fn transition(state: State, event: Event) -> State {
    state.transition(event)
}

/// Fold events over an initial state, collecting the trace.
/// Mirrors: `List.fold_left (fun s e -> transition s e) initial events`
pub fn fold_events(initial: State, events: &[Event]) -> Vec<State> {
    // `scan` is fold that yields every intermediate accumulator —
    // the functional equivalent of OCaml's fold with side-effecting printf.
    std::iter::once(initial)
        .chain(events.iter().scan(initial, |s, &e| {
            *s = transition(*s, e);
            Some(*s)
        }))
        .collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Basic transition rules ---------------------------------------------

    #[test]
    fn test_locked_coin_unlocks() {
        assert_eq!(State::Locked.transition(Event::Coin), State::Unlocked);
    }

    #[test]
    fn test_unlocked_push_locks() {
        assert_eq!(State::Unlocked.transition(Event::Push), State::Locked);
    }

    #[test]
    fn test_locked_push_stays_locked() {
        assert_eq!(State::Locked.transition(Event::Push), State::Locked);
    }

    #[test]
    fn test_unlocked_coin_stays_unlocked() {
        assert_eq!(State::Unlocked.transition(Event::Coin), State::Unlocked);
    }

    // -- Sequence from the OCaml original: Coin Push Push Coin Coin Push ----

    #[test]
    fn test_ocaml_sequence_final_state() {
        let events = [
            Event::Coin,
            Event::Push,
            Event::Push,
            Event::Coin,
            Event::Coin,
            Event::Push,
        ];
        let (final_state, _) = run_machine(State::Locked, &events);
        assert_eq!(final_state, State::Locked);
    }

    #[test]
    fn test_ocaml_sequence_transitions() {
        let events = [
            Event::Coin,
            Event::Push,
            Event::Push,
            Event::Coin,
            Event::Coin,
            Event::Push,
        ];
        let (_, pairs) = run_machine(State::Locked, &events);
        // Locked→Unlocked, Unlocked→Locked, Locked→Locked,
        // Locked→Unlocked, Unlocked→Unlocked, Unlocked→Locked
        assert_eq!(
            pairs,
            vec![
                (State::Locked, State::Unlocked),
                (State::Unlocked, State::Locked),
                (State::Locked, State::Locked),
                (State::Locked, State::Unlocked),
                (State::Unlocked, State::Unlocked),
                (State::Unlocked, State::Locked),
            ]
        );
    }

    #[test]
    fn test_empty_sequence() {
        let (final_state, pairs) = run_machine(State::Locked, &[]);
        assert_eq!(final_state, State::Locked);
        assert!(pairs.is_empty());
    }

    #[test]
    fn test_fold_events_trace() {
        let events = [Event::Coin, Event::Push, Event::Push];
        let trace = fold_events(State::Locked, &events);
        assert_eq!(
            trace,
            vec![State::Locked, State::Unlocked, State::Locked, State::Locked,]
        );
    }

    #[test]
    fn test_free_transition_matches_method() {
        for &state in &[State::Locked, State::Unlocked] {
            for &event in &[Event::Coin, Event::Push] {
                assert_eq!(transition(state, event), state.transition(event));
            }
        }
    }

    #[test]
    fn test_state_names() {
        assert_eq!(State::Locked.name(), "Locked");
        assert_eq!(State::Unlocked.name(), "Unlocked");
    }
}
