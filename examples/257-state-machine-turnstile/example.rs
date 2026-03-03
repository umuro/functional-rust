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
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (Self::Locked, Event::Coin) => Self::Unlocked,
            (Self::Unlocked, Event::Push) => Self::Locked,
            (Self::Locked, Event::Push) => Self::Locked,
            (Self::Unlocked, Event::Coin) => Self::Unlocked,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Locked => "Locked",
            Self::Unlocked => "Unlocked",
        }
    }
}

pub fn run_machine(initial: State, events: &[Event]) -> (State, Vec<(State, State)>) {
    let mut transitions = Vec::with_capacity(events.len());
    let final_state = events.iter().fold(initial, |s, &e| {
        let next = s.transition(e);
        transitions.push((s, next));
        next
    });
    (final_state, transitions)
}

pub fn transition(state: State, event: Event) -> State {
    state.transition(event)
}

pub fn fold_events(initial: State, events: &[Event]) -> Vec<State> {
    std::iter::once(initial)
        .chain(events.iter().scan(initial, |s, &e| {
            *s = transition(*s, e);
            Some(*s)
        }))
        .collect()
}

fn main() {
    let events = [
        Event::Coin,
        Event::Push,
        Event::Push,
        Event::Coin,
        Event::Coin,
        Event::Push,
    ];

    println!("=== Turnstile state machine ===");
    let (final_state, pairs) = run_machine(State::Locked, &events);
    for (before, after) in &pairs {
        println!("{} -> {}", before.name(), after.name());
    }
    println!("Final: {}", final_state.name());

    println!("\n=== Fold trace (including initial) ===");
    let trace = fold_events(State::Locked, &events);
    for s in &trace {
        println!("{}", s.name());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            vec![
                State::Locked,
                State::Unlocked,
                State::Locked,
                State::Locked,
            ]
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

/* Output:
   === Turnstile state machine ===
   Locked -> Unlocked
   Unlocked -> Locked
   Locked -> Locked
   Locked -> Unlocked
   Unlocked -> Unlocked
   Unlocked -> Locked
   Final: Locked

   === Fold trace (including initial) ===
   Locked
   Unlocked
   Locked
   Locked
   Unlocked
   Unlocked
   Locked
*/
