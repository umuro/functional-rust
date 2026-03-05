//! # Closure as Return — Returning Closures

/// Return closure with impl Trait
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// Return closure that captures multiple values
pub fn make_linear(slope: f64, intercept: f64) -> impl Fn(f64) -> f64 {
    move |x| slope * x + intercept
}

/// Return closure that maintains state (using RefCell)
pub fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// Return closure factory
pub fn make_multiplier_factory() -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    |factor| Box::new(move |x| x * factor)
}

/// Generic closure return
pub fn make_mapper<T, F: Fn(T) -> T + 'static>(f: F) -> Box<dyn Fn(T) -> T>
where
    T: 'static,
{
    Box::new(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
    }

    #[test]
    fn test_make_linear() {
        let f = make_linear(2.0, 1.0); // y = 2x + 1
        assert_eq!(f(3.0), 7.0);
    }

    #[test]
    fn test_counter() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_factory() {
        let factory = make_multiplier_factory();
        let times3 = factory(3);
        let times5 = factory(5);
        assert_eq!(times3(10), 30);
        assert_eq!(times5(10), 50);
    }

    #[test]
    fn test_generic_mapper() {
        let double = make_mapper(|x: i32| x * 2);
        assert_eq!(double(21), 42);
    }
}
