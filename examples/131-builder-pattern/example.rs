// Example 131: Builder Pattern with Typestate
//
// Phantom type parameters track which required fields have been set.
// build() is only callable when all required fields are Present — guaranteed
// at compile time with zero runtime overhead.

use std::marker::PhantomData;

pub struct Missing;
pub struct Present;

#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: Option<u32>,
}

pub struct UserBuilder<N, E> {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    _phantom: PhantomData<(N, E)>,
}

impl UserBuilder<Missing, Missing> {
    pub fn new() -> Self {
        UserBuilder {
            name: None,
            email: None,
            age: None,
            _phantom: PhantomData,
        }
    }
}

impl<E> UserBuilder<Missing, E> {
    pub fn name(self, name: &str) -> UserBuilder<Present, E> {
        UserBuilder {
            name: Some(name.to_string()),
            email: self.email,
            age: self.age,
            _phantom: PhantomData,
        }
    }
}

impl<N> UserBuilder<N, Missing> {
    pub fn email(self, email: &str) -> UserBuilder<N, Present> {
        UserBuilder {
            name: self.name,
            email: Some(email.to_string()),
            age: self.age,
            _phantom: PhantomData,
        }
    }
}

impl<N, E> UserBuilder<N, E> {
    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
}

impl UserBuilder<Present, Present> {
    pub fn build(self) -> User {
        User {
            name: self.name.expect("Present guarantees name is Some"),
            email: self.email.expect("Present guarantees email is Some"),
            age: self.age,
        }
    }
}

fn main() {
    // Both required fields provided — compiles fine.
    let user = UserBuilder::new()
        .name("Alice")
        .email("alice@example.com")
        .age(30)
        .build();
    println!("user = {:?}", user);

    // Order is irrelevant — email first, then name.
    let user2 = UserBuilder::new()
        .email("bob@example.com")
        .name("Bob")
        .build();
    println!("user2 = {:?}", user2);

    // These lines do NOT compile — uncomment to see the type error:
    // let bad = UserBuilder::new().name("Alice").build();
    // error[E0599]: no method named `build` found for struct
    //               `UserBuilder<Present, Missing>` in the current scope
}

/* Output:
   user = User { name: "Alice", email: "alice@example.com", age: Some(30) }
   user2 = User { name: "Bob", email: "bob@example.com", age: None }
*/
