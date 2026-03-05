//! # Phantom State Machine
//! Typestate pattern for compile-time state transitions

use std::marker::PhantomData;

// States
pub struct Idle;
pub struct Running;
pub struct Stopped;

// State machine
pub struct StateMachine<S> { _state: PhantomData<S> }

impl StateMachine<Idle> {
    pub fn new() -> Self { StateMachine { _state: PhantomData } }
    pub fn start(self) -> StateMachine<Running> { StateMachine { _state: PhantomData } }
}

impl StateMachine<Running> {
    pub fn stop(self) -> StateMachine<Stopped> { StateMachine { _state: PhantomData } }
    pub fn process(&self) -> &str { "Processing..." }
}

impl StateMachine<Stopped> {
    pub fn restart(self) -> StateMachine<Running> { StateMachine { _state: PhantomData } }
}

impl Default for StateMachine<Idle> { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_machine() {
        let machine = StateMachine::new();
        let running = machine.start();
        assert_eq!(running.process(), "Processing...");
        let stopped = running.stop();
        let _restarted = stopped.restart();
    }
}
