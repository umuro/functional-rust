#![allow(clippy::all)]
// 081: Newtype Pattern
// Wrapping primitives for type safety at zero cost

// Approach 1: Simple newtypes
#[derive(Debug, Clone, Copy)]
struct Meters(f64);

#[derive(Debug, Clone, Copy)]
struct Seconds(f64);

#[derive(Debug, Clone, Copy)]
struct MetersPerSecond(f64);

fn speed(distance: Meters, time: Seconds) -> Option<MetersPerSecond> {
    if time.0 == 0.0 {
        None
    } else {
        Some(MetersPerSecond(distance.0 / time.0))
    }
}

// Approach 2: Distinct ID types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderId(u64);

// These are different types — can't mix them!
fn find_user(id: UserId) -> String {
    format!("User #{}", id.0)
}
fn find_order(id: OrderId) -> String {
    format!("Order #{}", id.0)
}

// Approach 3: Newtype with conversions
#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed() {
        let s = speed(Meters(100.0), Seconds(10.0)).unwrap();
        assert!((s.0 - 10.0).abs() < 0.001);
        assert!(speed(Meters(100.0), Seconds(0.0)).is_none());
    }

    #[test]
    fn test_distinct_ids() {
        assert_eq!(find_user(UserId(42)), "User #42");
        assert_eq!(find_order(OrderId(7)), "Order #7");
        // UserId(1) != OrderId(1) — different types, won't even compile if compared
    }

    #[test]
    fn test_temperature() {
        let f: Fahrenheit = Celsius(100.0).into();
        assert!((f.0 - 212.0).abs() < 0.001);
        let c: Celsius = Fahrenheit(32.0).into();
        assert!(c.0.abs() < 0.001);
    }
}
