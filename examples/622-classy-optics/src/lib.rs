#![allow(clippy::all)]
//! # Classy Optics
//! Type-class based optics.

pub trait HasName {
    fn name(&self) -> &str;
    fn set_name(&mut self, n: String);
}

#[derive(Clone)]
pub struct User {
    pub name: String,
}
impl HasName for User {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, n: String) {
        self.name = n;
    }
}

pub fn greet(x: &impl HasName) -> String {
    format!("Hello, {}!", x.name())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_classy() {
        let u = User { name: "Bob".into() };
        assert_eq!(greet(&u), "Hello, Bob!");
    }
}
