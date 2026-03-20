#![allow(clippy::all)]
//! Supertrait Pattern

use std::fmt::{Debug, Display};

pub trait Printable: Debug + Display {
    fn print(&self) {
        println!("Debug: {:?}, Display: {}", self, self);
    }
}

pub trait Entity: Clone + Default {
    fn id(&self) -> u64;
}

#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: u64,
    pub name: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User({})", self.name)
    }
}
impl Printable for User {}
impl Entity for User {
    fn id(&self) -> u64 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printable() {
        let u = User {
            id: 1,
            name: "Alice".into(),
        };
        assert!(format!("{:?}", u).contains("Alice"));
    }
    #[test]
    fn test_display() {
        let u = User {
            id: 1,
            name: "Bob".into(),
        };
        assert_eq!(format!("{}", u), "User(Bob)");
    }
    #[test]
    fn test_entity() {
        let u = User {
            id: 42,
            ..Default::default()
        };
        assert_eq!(u.id(), 42);
    }
    #[test]
    fn test_clone() {
        let u = User {
            id: 1,
            name: "X".into(),
        };
        let u2 = u.clone();
        assert_eq!(u2.id, 1);
    }
}
