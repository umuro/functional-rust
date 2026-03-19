/// 734: Typestate Basics — compile-time state machine
/// Invalid transitions DO NOT compile. No runtime checks needed.
use std::marker::PhantomData;

// ── State marker types (zero-sized) ──────────────────────────────────────────

pub struct Red;
pub struct Green;
pub struct Yellow;

// ── Traffic Light ─────────────────────────────────────────────────────────────

/// `PhantomData<State>` is zero bytes at runtime.
/// The type parameter encodes which state we're in.
pub struct Light<State> {
    _state: PhantomData<State>,
}

impl Light<Red> {
    /// Only way to create a light — must start Red.
    pub fn new() -> Self {
        println!("Light: Red");
        Light {
            _state: PhantomData,
        }
    }

    /// Red → Green (consumes self, returns different type)
    pub fn go(self) -> Light<Green> {
        println!("Light: Green");
        Light {
            _state: PhantomData,
        }
    }
}

impl Light<Green> {
    /// Green → Yellow
    pub fn slow(self) -> Light<Yellow> {
        println!("Light: Yellow");
        Light {
            _state: PhantomData,
        }
    }
}

impl Light<Yellow> {
    /// Yellow → Red
    pub fn stop(self) -> Light<Red> {
        println!("Light: Red");
        Light {
            _state: PhantomData,
        }
    }
}

// ── Size check ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_cycle_compiles() {
        let red = Light::<Red>::new();
        let green = red.go();
        let yellow = green.slow();
        let _red2 = yellow.stop();
    }

    #[test]
    fn light_is_zero_sized() {
        assert_eq!(std::mem::size_of::<Light<Red>>(), 0);
        assert_eq!(std::mem::size_of::<Light<Green>>(), 0);
        assert_eq!(std::mem::size_of::<Light<Yellow>>(), 0);
    }

    // Uncommenting the test below would fail to COMPILE — which is the point:
    //
    // #[test]
    // fn invalid_transition_compile_error() {
    //     let r = Light::<Red>::new();
    //     let _ = r.slow();  // compile error: method not found
    // }
}
