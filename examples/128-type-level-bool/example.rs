//! Example 128: Type-Level Booleans — standalone demo

use std::marker::PhantomData;

pub struct True;
pub struct False;

pub trait Bool {
    const VALUE: bool;
}
impl Bool for True {
    const VALUE: bool = true;
}
impl Bool for False {
    const VALUE: bool = false;
}

pub trait Not {
    type Output: Bool;
}
impl Not for True {
    type Output = False;
}
impl Not for False {
    type Output = True;
}

pub trait And<B: Bool> {
    type Output: Bool;
}
impl<B: Bool> And<B> for True {
    type Output = B;
}
impl<B: Bool> And<B> for False {
    type Output = False;
}

pub struct Config<V, L> {
    pub host: String,
    pub port: u16,
    _validated: PhantomData<V>,
    _logged: PhantomData<L>,
}

impl Config<False, False> {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Config {
            host: host.into(),
            port,
            _validated: PhantomData,
            _logged: PhantomData,
        }
    }
}

impl<L> Config<False, L> {
    pub fn validate(self) -> Config<True, L> {
        Config {
            host: self.host,
            port: self.port,
            _validated: PhantomData,
            _logged: PhantomData,
        }
    }
}

impl<V> Config<V, False> {
    pub fn enable_logging(self) -> Config<V, True> {
        Config {
            host: self.host,
            port: self.port,
            _validated: PhantomData,
            _logged: PhantomData,
        }
    }
}

impl Config<True, True> {
    pub fn execute(&self) -> String {
        format!("Executing on {}:{}", self.host, self.port)
    }
}

fn main() {
    println!("True::VALUE  = {}", True::VALUE);
    println!("False::VALUE = {}", False::VALUE);
    println!("NOT True     = {}", <True as Not>::Output::VALUE);
    println!("NOT False    = {}", <False as Not>::Output::VALUE);
    println!("True AND False = {}", <True as And<False>>::Output::VALUE);

    let result = Config::new("localhost", 9000)
        .validate()
        .enable_logging()
        .execute();
    println!("{}", result);

    // The following would NOT compile — execute() doesn't exist yet:
    // Config::new("host", 80).execute();
}

/* Output:
   True::VALUE  = true
   False::VALUE = false
   NOT True     = false
   NOT False    = true
   True AND False = false
   Executing on localhost:9000
*/
