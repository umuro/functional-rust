// dyn Trait and fat pointers in Rust
use std::fmt;

trait Animal: fmt::Display {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
}

struct Dog { name: String }
struct Cat { name: String }
struct Parrot { name: String, word: String }

impl Animal for Dog {
    fn speak(&self) -> String { "Woof!".to_string() }
    fn name(&self) -> &str { &self.name }
}

impl Animal for Cat {
    fn speak(&self) -> String { "Meow!".to_string() }
    fn name(&self) -> &str { &self.name }
}

impl Animal for Parrot {
    fn speak(&self) -> String { format!("{}!", self.word) }
    fn name(&self) -> &str { &self.name }
}

impl fmt::Display for Dog { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Dog({})", self.name) } }
impl fmt::Display for Cat { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Cat({})", self.name) } }
impl fmt::Display for Parrot { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Parrot({})", self.name) } }

fn make_noise(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        println!("{} says: {}", animal.name(), animal.speak());
    }
}

fn largest_name<'a>(animals: &'a [Box<dyn Animal>]) -> &'a str {
    animals.iter().map(|a| a.name()).max_by_key(|n| n.len()).unwrap_or("")
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Rex".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
        Box::new(Parrot { name: "Polly".to_string(), word: "Hello".to_string() }),
    ];

    make_noise(&animals);

    // Fat pointer size = 2 * usize
    println!("\nSize of Box<dyn Animal>: {} bytes", std::mem::size_of::<Box<dyn Animal>>());
    println!("Size of Box<Dog>:        {} bytes", std::mem::size_of::<Box<Dog>>());
    println!("Longest name: {}", largest_name(&animals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_dispatch() {
        let a: Box<dyn Animal> = Box::new(Dog { name: "Buddy".to_string() });
        assert_eq!(a.speak(), "Woof!");
        assert_eq!(a.name(), "Buddy");
    }

    #[test]
    fn test_fat_pointer_size() {
        // fat pointer = 2 * pointer size
        assert_eq!(std::mem::size_of::<Box<dyn Animal>>(),
                   2 * std::mem::size_of::<usize>());
    }
}
