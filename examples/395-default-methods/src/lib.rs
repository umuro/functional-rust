//! Default Methods in Traits

pub trait Greeter {
    fn name(&self) -> &str;
    fn greeting(&self) -> String { format!("Hello, {}!", self.name()) }
    fn formal_greeting(&self) -> String { format!("Dear {}", self.name()) }
}

pub struct Person { pub name: String }
pub struct Robot { pub id: u32 }

impl Greeter for Person { fn name(&self) -> &str { &self.name } }
impl Greeter for Robot {
    fn name(&self) -> &str { "Robot" }
    fn greeting(&self) -> String { format!("BEEP BOOP - Unit {}", self.id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_person_greeting() { let p = Person { name: "Alice".into() }; assert!(p.greeting().contains("Alice")); }
    #[test] fn test_person_formal() { let p = Person { name: "Bob".into() }; assert!(p.formal_greeting().contains("Dear")); }
    #[test] fn test_robot_override() { let r = Robot { id: 42 }; assert!(r.greeting().contains("BEEP")); }
    #[test] fn test_robot_default() { let r = Robot { id: 1 }; assert!(r.formal_greeting().contains("Robot")); }
}
