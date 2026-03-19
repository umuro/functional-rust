#![allow(clippy::all)]
//! dyn Trait and Fat Pointers

pub trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
}

pub struct Dog {
    pub name: String,
}
pub struct Cat {
    pub name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn make_noise(animals: &[Box<dyn Animal>]) -> Vec<String> {
    animals
        .iter()
        .map(|a| format!("{}: {}", a.name(), a.speak()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dog() {
        let d = Dog {
            name: "Rex".to_string(),
        };
        assert_eq!(d.speak(), "Woof!");
    }

    #[test]
    fn test_cat() {
        let c = Cat {
            name: "Whiskers".to_string(),
        };
        assert_eq!(c.speak(), "Meow!");
    }

    #[test]
    fn test_heterogeneous_vec() {
        let animals: Vec<Box<dyn Animal>> = vec![
            Box::new(Dog {
                name: "Rex".to_string(),
            }),
            Box::new(Cat {
                name: "Whiskers".to_string(),
            }),
        ];
        let noises = make_noise(&animals);
        assert!(noises[0].contains("Woof"));
        assert!(noises[1].contains("Meow"));
    }

    #[test]
    fn test_trait_object_size() {
        // Fat pointer: 2 words (ptr + vtable)
        assert_eq!(
            std::mem::size_of::<&dyn Animal>(),
            2 * std::mem::size_of::<usize>()
        );
    }
}
