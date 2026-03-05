// Typeclass-style optics: "Has-X" pattern

// Trait for any type with a "name" field
trait HasName {
    fn get_name(&self) -> &str;
    fn set_name(self, name: String) -> Self;
    fn map_name(self, f: impl Fn(&str) -> String) -> Self where Self: Sized {
        let new_name = f(self.get_name());
        self.set_name(new_name)
    }
}

// Trait for any type with an "id" field
trait HasId {
    fn get_id(&self) -> u64;
    fn set_id(self, id: u64) -> Self;
}

// Generic function: works on any type with a name
fn capitalize_name<T: HasName>(x: T) -> T {
    x.map_name(|s| {
        let mut c = s.chars();
        match c.next() {
            None    => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    })
}

fn greet_named<T: HasName>(x: &T) -> String {
    format!("Hello, {}!", x.get_name())
}

// Concrete types implementing the traits
#[derive(Debug,Clone)]
struct Person  { name: String, age: u32, id: u64 }
#[derive(Debug,Clone)]
struct Account { name: String, balance: f64, id: u64 }
#[derive(Debug,Clone)]
struct Device  { name: String, model: String }

impl HasName for Person {
    fn get_name(&self) -> &str { &self.name }
    fn set_name(mut self, n: String) -> Self { self.name = n; self }
}
impl HasName for Account {
    fn get_name(&self) -> &str { &self.name }
    fn set_name(mut self, n: String) -> Self { self.name = n; self }
}
impl HasName for Device {
    fn get_name(&self) -> &str { &self.name }
    fn set_name(mut self, n: String) -> Self { self.name = n; self }
}

impl HasId for Person  { fn get_id(&self)->u64{self.id} fn set_id(mut self,id:u64)->Self{self.id=id;self} }
impl HasId for Account { fn get_id(&self)->u64{self.id} fn set_id(mut self,id:u64)->Self{self.id=id;self} }

// Polymorphic: works on any HasName + HasId
fn tag_entity<T: HasName + HasId>(x: T) -> String {
    format!("[{}:{}]", x.get_id(), x.get_name())
}

fn main() {
    let p = Person  { name:"alice".into(),   age:30,  id:1 };
    let a = Account { name:"bob_account".into(), balance:1000.0, id:2 };
    let d = Device  { name:"my-laptop".into(), model:"X1".into() };

    println!("{}", greet_named(&p));
    println!("{}", greet_named(&a));
    println!("{}", greet_named(&d));

    let p2 = capitalize_name(p.clone());
    let a2 = capitalize_name(a.clone());
    println!("cap person: {}", p2.name);
    println!("cap account: {}", a2.name);

    println!("{}", tag_entity(p));
    println!("{}", tag_entity(a));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn capitalize() {
        let p = Person{name:"alice".into(),age:0,id:0};
        assert_eq!(capitalize_name(p).name, "Alice");
    }
    #[test] fn greet() {
        let d = Device{name:"laptop".into(),model:"".into()};
        assert_eq!(greet_named(&d), "Hello, laptop!");
    }
}
