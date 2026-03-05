//! State Machine Macro
//!
//! Defining state machines with macros.

/// Simple state machine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

/// Door state machine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DoorState {
    Open,
    Closed,
    Locked,
}

impl DoorState {
    pub fn can_open(&self) -> bool {
        matches!(self, DoorState::Closed)
    }

    pub fn can_close(&self) -> bool {
        matches!(self, DoorState::Open)
    }

    pub fn can_lock(&self) -> bool {
        matches!(self, DoorState::Closed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_light_cycle() {
        let mut light = TrafficLight::Red;
        light = light.next(); // Green
        assert_eq!(light, TrafficLight::Green);
        light = light.next(); // Yellow
        assert_eq!(light, TrafficLight::Yellow);
        light = light.next(); // Red
        assert_eq!(light, TrafficLight::Red);
    }

    #[test]
    fn test_door_open() {
        let door = DoorState::Closed;
        assert!(door.can_open());
    }

    #[test]
    fn test_door_locked_cannot_open() {
        let door = DoorState::Locked;
        assert!(!door.can_open());
    }

    #[test]
    fn test_door_close() {
        let door = DoorState::Open;
        assert!(door.can_close());
    }

    #[test]
    fn test_door_lock() {
        let door = DoorState::Closed;
        assert!(door.can_lock());
    }
}
