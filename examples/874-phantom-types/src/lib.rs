// Example 080: Phantom Types
// Compile-time safety with phantom type parameters

use std::marker::PhantomData;

// === Approach 1: Units of measure ===
struct Meters;
struct Seconds;
struct MetersPerSecond;

#[derive(Debug, Clone, Copy)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> Quantity<U> {
    fn new(value: f64) -> Self {
        Quantity { value, _unit: PhantomData }
    }

    fn scale(self, factor: f64) -> Self {
        Quantity::new(self.value * factor)
    }
}

// Same-unit addition
impl<U> std::ops::Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Quantity::new(self.value + rhs.value)
    }
}

fn speed(distance: Quantity<Meters>, time: Quantity<Seconds>) -> Quantity<MetersPerSecond> {
    Quantity::new(distance.value / time.value)
}

// === Approach 2: State machine with phantom types ===
struct Locked;
struct Unlocked;

struct Door<State> {
    name: String,
    _state: PhantomData<State>,
}

impl Door<Unlocked> {
    fn new(name: &str) -> Self {
        Door { name: name.to_string(), _state: PhantomData }
    }

    fn lock(self) -> Door<Locked> {
        Door { name: self.name, _state: PhantomData }
    }

    fn walk_through(&self) -> String {
        format!("Walked through {}", self.name)
    }
}

impl Door<Locked> {
    fn unlock(self) -> Door<Unlocked> {
        Door { name: self.name, _state: PhantomData }
    }
    // Cannot walk_through a locked door — method doesn't exist!
}

// === Approach 3: Validated data ===
struct Unvalidated;
struct Validated;

struct Email<State> {
    address: String,
    _state: PhantomData<State>,
}

impl Email<Unvalidated> {
    fn new(address: &str) -> Self {
        Email { address: address.to_string(), _state: PhantomData }
    }

    fn validate(self) -> Result<Email<Validated>, String> {
        if self.address.contains('@') {
            Ok(Email { address: self.address, _state: PhantomData })
        } else {
            Err(format!("Invalid email: {}", self.address))
        }
    }
}

impl Email<Validated> {
    fn send(&self) -> String {
        format!("Sent to {}", self.address)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_addition() {
        let a = Quantity::<Meters>::new(10.0);
        let b = Quantity::<Meters>::new(20.0);
        let c = a + b;
        assert!((c.value - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_speed_calculation() {
        let d = Quantity::<Meters>::new(100.0);
        let t = Quantity::<Seconds>::new(10.0);
        let s = speed(d, t);
        assert!((s.value - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_scale() {
        let d = Quantity::<Meters>::new(5.0);
        let d2 = d.scale(3.0);
        assert!((d2.value - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_door_state_machine() {
        let door = Door::<Unlocked>::new("test");
        assert_eq!(door.walk_through(), "Walked through test");
        let locked = door.lock();
        let unlocked = locked.unlock();
        assert_eq!(unlocked.walk_through(), "Walked through test");
    }

    #[test]
    fn test_valid_email() {
        let email = Email::<Unvalidated>::new("a@b.com");
        let valid = email.validate().unwrap();
        assert_eq!(valid.send(), "Sent to a@b.com");
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::<Unvalidated>::new("nope");
        assert!(email.validate().is_err());
    }
}
