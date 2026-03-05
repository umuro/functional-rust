//! # State Automata with Pattern Matching
//!
//! Implement finite state machines using enum states and tuple matching
//! for state transitions.

/// Process state with associated data.
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Idle,
    Running(u32),
    Paused(u32),
    Done(u32),
}

/// Events that can trigger state transitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Start,
    Tick,
    Pause,
    Resume,
    Stop,
}

/// State transition function using tuple matching.
pub fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle, Event::Start) => State::Running(0),
        (State::Running(n), Event::Tick) => State::Running(n + 1),
        (State::Running(n), Event::Pause) => State::Paused(n),
        (State::Running(n), Event::Stop) => State::Done(n),
        (State::Paused(n), Event::Resume) => State::Running(n),
        (State::Paused(n), Event::Stop) => State::Done(n),
        (s, _) => s, // Ignore invalid transitions
    }
}

/// Describe current state in human-readable form.
pub fn describe(s: &State) -> String {
    match s {
        State::Idle => "idle".into(),
        State::Running(n) => format!("running (tick {})", n),
        State::Paused(n) => format!("paused at {}", n),
        State::Done(n) => format!("done after {} ticks", n),
    }
}

/// Check if state is terminal.
pub fn is_terminal(s: &State) -> bool {
    matches!(s, State::Done(_))
}

/// Check if state is active (running or paused).
pub fn is_active(s: &State) -> bool {
    matches!(s, State::Running(_) | State::Paused(_))
}

/// Run a sequence of events.
pub fn run_sequence(events: &[Event]) -> State {
    events
        .iter()
        .fold(State::Idle, |s, &e| transition(s, e))
}

/// Traffic light state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Traffic {
    Red,
    Green,
    Yellow,
}

/// Get next traffic light state (simple cycle).
pub fn next_traffic(t: Traffic) -> Traffic {
    match t {
        Traffic::Red => Traffic::Green,
        Traffic::Green => Traffic::Yellow,
        Traffic::Yellow => Traffic::Red,
    }
}

/// Traffic light with timer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficTimed {
    Red(u32),
    Green(u32),
    Yellow(u32),
}

/// Tick traffic light timer.
pub fn tick_traffic(t: TrafficTimed) -> TrafficTimed {
    match t {
        TrafficTimed::Red(0) => TrafficTimed::Green(30),    // Green for 30 ticks
        TrafficTimed::Red(n) => TrafficTimed::Red(n - 1),
        TrafficTimed::Green(0) => TrafficTimed::Yellow(5),  // Yellow for 5 ticks
        TrafficTimed::Green(n) => TrafficTimed::Green(n - 1),
        TrafficTimed::Yellow(0) => TrafficTimed::Red(30),   // Red for 30 ticks
        TrafficTimed::Yellow(n) => TrafficTimed::Yellow(n - 1),
    }
}

/// Connection state machine.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnState {
    Disconnected,
    Connecting(String),
    Connected(String),
    Disconnecting,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnEvent {
    Connect(String),
    Ack,
    Disconnect,
    Timeout,
}

/// Connection state transition.
pub fn conn_transition(state: ConnState, event: ConnEvent) -> ConnState {
    match (state, event) {
        (ConnState::Disconnected, ConnEvent::Connect(addr)) => ConnState::Connecting(addr),
        (ConnState::Connecting(addr), ConnEvent::Ack) => ConnState::Connected(addr),
        (ConnState::Connecting(_), ConnEvent::Timeout) => ConnState::Disconnected,
        (ConnState::Connected(_), ConnEvent::Disconnect) => ConnState::Disconnecting,
        (ConnState::Disconnecting, ConnEvent::Ack) => ConnState::Disconnected,
        (s, _) => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle_to_running() {
        let s = transition(State::Idle, Event::Start);
        assert_eq!(s, State::Running(0));
    }

    #[test]
    fn test_running_tick() {
        let s = transition(State::Running(5), Event::Tick);
        assert_eq!(s, State::Running(6));
    }

    #[test]
    fn test_pause_resume() {
        let s = transition(State::Running(5), Event::Pause);
        assert_eq!(s, State::Paused(5));

        let s2 = transition(s, Event::Resume);
        assert_eq!(s2, State::Running(5));
    }

    #[test]
    fn test_stop_from_running() {
        let s = transition(State::Running(10), Event::Stop);
        assert_eq!(s, State::Done(10));
    }

    #[test]
    fn test_stop_from_paused() {
        let s = transition(State::Paused(7), Event::Stop);
        assert_eq!(s, State::Done(7));
    }

    #[test]
    fn test_invalid_transition_ignored() {
        let s = transition(State::Done(5), Event::Start);
        assert_eq!(s, State::Done(5));
    }

    #[test]
    fn test_describe() {
        assert_eq!(describe(&State::Idle), "idle");
        assert_eq!(describe(&State::Running(3)), "running (tick 3)");
        assert_eq!(describe(&State::Paused(5)), "paused at 5");
        assert_eq!(describe(&State::Done(10)), "done after 10 ticks");
    }

    #[test]
    fn test_is_terminal() {
        assert!(!is_terminal(&State::Idle));
        assert!(!is_terminal(&State::Running(0)));
        assert!(is_terminal(&State::Done(0)));
    }

    #[test]
    fn test_run_sequence() {
        let events = [
            Event::Start,
            Event::Tick,
            Event::Tick,
            Event::Pause,
            Event::Resume,
            Event::Tick,
            Event::Stop,
        ];
        let final_state = run_sequence(&events);
        assert_eq!(final_state, State::Done(3));
    }

    #[test]
    fn test_traffic_cycle() {
        let mut t = Traffic::Red;
        t = next_traffic(t);
        assert_eq!(t, Traffic::Green);
        t = next_traffic(t);
        assert_eq!(t, Traffic::Yellow);
        t = next_traffic(t);
        assert_eq!(t, Traffic::Red);
    }

    #[test]
    fn test_traffic_timed() {
        let mut t = TrafficTimed::Red(2);
        t = tick_traffic(t);
        assert_eq!(t, TrafficTimed::Red(1));
        t = tick_traffic(t);
        assert_eq!(t, TrafficTimed::Red(0));
        t = tick_traffic(t);
        assert_eq!(t, TrafficTimed::Green(30));
    }

    #[test]
    fn test_connection_happy_path() {
        let s = ConnState::Disconnected;
        let s = conn_transition(s, ConnEvent::Connect("127.0.0.1".into()));
        assert!(matches!(s, ConnState::Connecting(_)));
        let s = conn_transition(s, ConnEvent::Ack);
        assert!(matches!(s, ConnState::Connected(_)));
        let s = conn_transition(s, ConnEvent::Disconnect);
        assert_eq!(s, ConnState::Disconnecting);
        let s = conn_transition(s, ConnEvent::Ack);
        assert_eq!(s, ConnState::Disconnected);
    }

    #[test]
    fn test_connection_timeout() {
        let s = ConnState::Connecting("host".into());
        let s = conn_transition(s, ConnEvent::Timeout);
        assert_eq!(s, ConnState::Disconnected);
    }
}
