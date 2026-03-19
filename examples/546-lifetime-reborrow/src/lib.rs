//! Reborrowing Patterns
//!
//! Creating sub-borrows from existing borrows.

/// Read a value through a reference.
pub fn read_value(r: &i32) -> i32 {
    *r
}

/// Increment through a mutable reference.
pub fn increment(r: &mut i32) {
    *r += 1;
}

/// Demonstrate implicit reborrow: &mut T -> &T.
pub fn implicit_reborrow_demo() -> i32 {
    let mut x = 42;
    let r = &mut x;

    // &mut T coerces to &T (implicit reborrow)
    let val = read_value(r); // r reborrowed as &i32
                             // r still valid — reborrow ended

    *r += 1; // can still use r
    val
}

/// Explicit reborrow with &*.
pub fn explicit_reborrow(r: &mut i32) -> i32 {
    let shared: &i32 = &*r; // explicit reborrow
    *shared
}

/// Reborrow in method chains.
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Counter { value }
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn increment(&mut self) -> &mut Self {
        self.value += 1;
        self // return &mut self for chaining
    }

    pub fn double(&mut self) -> &mut Self {
        self.value *= 2;
        self
    }
}

/// Reborrow through function parameter.
pub fn process_twice(r: &mut i32) {
    increment(r); // implicit reborrow
    increment(r); // another reborrow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_reborrow() {
        let result = implicit_reborrow_demo();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_explicit_reborrow() {
        let mut x = 10;
        let result = explicit_reborrow(&mut x);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_counter_chain() {
        let mut counter = Counter::new(1);
        counter.increment().double().increment();
        assert_eq!(counter.get(), 5); // (1+1)*2+1 = 5
    }

    #[test]
    fn test_process_twice() {
        let mut x = 0;
        process_twice(&mut x);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_reborrow_pattern() {
        let mut v = vec![1, 2, 3];
        let r = &mut v;

        // Each push reborrows r
        r.push(4);
        r.push(5);

        assert_eq!(*r, vec![1, 2, 3, 4, 5]);
    }
}
