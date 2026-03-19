#![allow(clippy::all)]
//! # Isomorphism Pattern
//! Bidirectional conversion between types.

pub struct Iso<A, B> {
    pub to: Box<dyn Fn(A) -> B>,
    pub from: Box<dyn Fn(B) -> A>,
}

impl<A, B> Iso<A, B> {
    pub fn new(to: impl Fn(A) -> B + 'static, from: impl Fn(B) -> A + 'static) -> Self {
        Iso {
            to: Box::new(to),
            from: Box::new(from),
        }
    }
}

pub fn celsius_fahrenheit() -> Iso<f64, f64> {
    Iso::new(|c| c * 9.0 / 5.0 + 32.0, |f| (f - 32.0) * 5.0 / 9.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_iso() {
        let iso = celsius_fahrenheit();
        let f = (iso.to)(100.0);
        assert!((f - 212.0).abs() < 0.001);
    }
}
