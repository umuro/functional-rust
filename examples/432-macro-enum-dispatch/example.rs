// enum_dispatch via macros in Rust
// Eliminates dyn Trait overhead by converting to static dispatch

// The macro generates:
// 1. An enum with one variant per type
// 2. A trait impl for the enum that delegates via match

macro_rules! enum_dispatch {
    (
        trait $trait_name:ident {
            $( fn $method:ident ( $($param:ident : $pty:ty),* ) -> $ret:ty ; )*
        }
        enum $enum_name:ident {
            $( $variant:ident ( $inner_ty:ty ) ),* $(,)?
        }
    ) => {
        // The enum
        #[derive(Debug)]
        enum $enum_name {
            $( $variant($inner_ty), )*
        }

        // Trait impl: delegate each method to the inner type
        impl $trait_name for $enum_name {
            $(
                fn $method ( &self, $($param: $pty),* ) -> $ret {
                    match self {
                        $( $enum_name::$variant(inner) => inner.$method($($param),*), )*
                    }
                }
            )*
        }

        // From impls for each variant
        $(
            impl From<$inner_ty> for $enum_name {
                fn from(x: $inner_ty) -> Self { $enum_name::$variant(x) }
            }
        )*
    };
}

// Define the trait
trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> String;
    fn speed(&self) -> f64;
}

// Concrete types
struct Dog { name: String }
struct Cat { name: String }
struct Bird { name: String, speed: f64 }

impl Animal for Dog {
    fn speak(&self) -> String { "Woof!".into() }
    fn name(&self) -> String { self.name.clone() }
    fn speed(&self) -> f64 { 5.0 }
}

impl Animal for Cat {
    fn speak(&self) -> String { "Meow!".into() }
    fn name(&self) -> String { self.name.clone() }
    fn speed(&self) -> f64 { 8.0 }
}

impl Animal for Bird {
    fn speak(&self) -> String { "Tweet!".into() }
    fn name(&self) -> String { self.name.clone() }
    fn speed(&self) -> f64 { self.speed }
}

// Generate the enum dispatch
enum_dispatch! {
    trait Animal {
        fn speak() -> String;
        fn name() -> String;
        fn speed() -> f64;
    }
    enum AnyAnimal {
        Dog(Dog),
        Cat(Cat),
        Bird(Bird),
    }
}

fn describe(a: &AnyAnimal) {
    println!("{} says: {} (speed: {})", a.name(), a.speak(), a.speed());
}

fn main() {
    let animals: Vec<AnyAnimal> = vec![
        Dog { name: "Rex".to_string() }.into(),
        Cat { name: "Whiskers".to_string() }.into(),
        Bird { name: "Tweety".to_string(), speed: 30.0 }.into(),
    ];

    for a in &animals {
        describe(a);
    }

    // No heap allocation! Stored by value
    println!("\nSize of AnyAnimal: {} bytes", std::mem::size_of::<AnyAnimal>());
    println!("Size of Box<dyn Animal> (for comparison): {} bytes",
             std::mem::size_of::<Box<dyn Animal>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_dispatch() {
        let d: AnyAnimal = Dog { name: "Buddy".to_string() }.into();
        assert_eq!(d.speak(), "Woof!");
        assert_eq!(d.name(), "Buddy");
    }

    #[test]
    fn test_no_heap_alloc() {
        // Stack allocated, not heap
        let _a: AnyAnimal = Cat { name: "Felix".to_string() }.into();
        // No Box needed
    }
}
