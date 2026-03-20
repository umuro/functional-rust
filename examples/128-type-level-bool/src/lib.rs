#![allow(clippy::all)]
//! Example 128: Type-Level Booleans
//!
//! Encode `true`/`false` as *types* instead of values so the compiler enforces
//! logical constraints without any runtime checks.

use std::marker::PhantomData;

// ── Approach 1: Marker structs ────────────────────────────────────────────────
// Two zero-sized structs that act as compile-time labels.

pub struct True;
pub struct False;

/// Lift a type-level boolean to a runtime value.
pub trait Bool {
    const VALUE: bool;
}

impl Bool for True {
    const VALUE: bool = true;
}

impl Bool for False {
    const VALUE: bool = false;
}

// ── Type-level NOT ────────────────────────────────────────────────────────────

pub trait Not {
    type Output: Bool;
}

impl Not for True {
    type Output = False;
}

impl Not for False {
    type Output = True;
}

// ── Type-level AND ────────────────────────────────────────────────────────────

pub trait And<B: Bool> {
    type Output: Bool;
}

impl<B: Bool> And<B> for True {
    type Output = B; // True AND B = B
}

impl<B: Bool> And<B> for False {
    type Output = False; // False AND _ = False
}

// ── Type-level OR ─────────────────────────────────────────────────────────────

pub trait Or<B: Bool> {
    type Output: Bool;
}

impl<B: Bool> Or<B> for True {
    type Output = True; // True OR _ = True
}

impl<B: Bool> Or<B> for False {
    type Output = B; // False OR B = B
}

// ── Approach 2: Builder enforced at compile time ───────────────────────────────
//
// `Config<Validated, Logged>` where each type parameter is either `True` or
// `False`.  The `execute()` method is defined *only* on `Config<True, True>`,
// so calling it before completing both setup steps is a compile error — the
// method simply doesn't exist on the other variants.

pub struct Config<V, L> {
    pub host: String,
    pub port: u16,
    // PhantomData holds the type parameters without storing any data at runtime.
    _validated: PhantomData<V>,
    _logged: PhantomData<L>,
}

impl Config<False, False> {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Config {
            host: host.into(),
            port,
            _validated: PhantomData,
            _logged: PhantomData,
        }
    }
}

// validate() is available whenever V = False (transitions V: False → True).
impl<L> Config<False, L> {
    pub fn validate(self) -> Config<True, L> {
        Config {
            host: self.host,
            port: self.port,
            _validated: PhantomData,
            _logged: PhantomData,
        }
    }
}

// enable_logging() is available whenever L = False (transitions L: False → True).
impl<V> Config<V, False> {
    pub fn enable_logging(self) -> Config<V, True> {
        Config {
            host: self.host,
            port: self.port,
            _validated: PhantomData,
            _logged: PhantomData,
        }
    }
}

// execute() only exists on the fully-configured type.
impl Config<True, True> {
    pub fn execute(&self) -> String {
        format!("Executing on {}:{}", self.host, self.port)
    }
}

// ── Approach 3: Tagged value — attach a type-level boolean to any value ────────

pub struct Tagged<T, B> {
    pub value: T,
    _marker: PhantomData<B>,
}

impl<T, B: Bool> Tagged<T, B> {
    pub fn new(value: T) -> Self {
        Tagged {
            value,
            _marker: PhantomData,
        }
    }

    pub fn is_true() -> bool {
        B::VALUE
    }
}

// get_verified() only compiles when the tag is True.
impl<T> Tagged<T, True> {
    pub fn get_verified(&self) -> &T {
        &self.value
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_values() {
        assert!(True::VALUE);
        assert!(!False::VALUE);
    }

    #[test]
    fn test_not() {
        assert_eq!(<True as Not>::Output::VALUE, false);
        assert_eq!(<False as Not>::Output::VALUE, true);
    }

    #[test]
    fn test_and() {
        assert_eq!(<True as And<True>>::Output::VALUE, true);
        assert_eq!(<True as And<False>>::Output::VALUE, false);
        assert_eq!(<False as And<True>>::Output::VALUE, false);
        assert_eq!(<False as And<False>>::Output::VALUE, false);
    }

    #[test]
    fn test_or() {
        assert_eq!(<True as Or<True>>::Output::VALUE, true);
        assert_eq!(<True as Or<False>>::Output::VALUE, true);
        assert_eq!(<False as Or<True>>::Output::VALUE, true);
        assert_eq!(<False as Or<False>>::Output::VALUE, false);
    }

    #[test]
    fn test_config_validate_then_log() {
        let result = Config::new("localhost", 8080)
            .validate()
            .enable_logging()
            .execute();
        assert_eq!(result, "Executing on localhost:8080");
    }

    #[test]
    fn test_config_log_then_validate() {
        // Order of setup steps doesn't matter — both paths reach Config<True, True>.
        let result = Config::new("example.com", 443)
            .enable_logging()
            .validate()
            .execute();
        assert_eq!(result, "Executing on example.com:443");
    }

    #[test]
    fn test_tagged_true() {
        let v: Tagged<i32, True> = Tagged::new(42);
        assert_eq!(*v.get_verified(), 42);
        assert!(Tagged::<i32, True>::is_true());
    }

    #[test]
    fn test_tagged_false() {
        let v: Tagged<&str, False> = Tagged::new("hello");
        assert_eq!(v.value, "hello");
        assert!(!Tagged::<i32, False>::is_true());
    }

    #[test]
    fn test_bool_const_evaluation() {
        // Verify that Bool::VALUE can be used in const contexts.
        const T: bool = True::VALUE;
        const F: bool = False::VALUE;
        assert!(T);
        assert!(!F);
    }
}
