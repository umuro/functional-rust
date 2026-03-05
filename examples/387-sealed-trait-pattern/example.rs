// Sealed trait pattern in Rust

// Private module to hold the seal
mod private {
    pub trait Sealed {}
}

// Public trait requires the private Sealed supertrait
// External code cannot implement this trait because they can't implement Sealed
pub trait Token: private::Sealed {
    fn value(&self) -> String;
    fn token_type(&self) -> &'static str;
}

pub struct Identifier(pub String);
pub struct Number(pub i64);
pub struct Punctuation(pub char);

// Only types in our crate can implement Sealed
impl private::Sealed for Identifier {}
impl private::Sealed for Number {}
impl private::Sealed for Punctuation {}

impl Token for Identifier {
    fn value(&self) -> String { self.0.clone() }
    fn token_type(&self) -> &'static str { "identifier" }
}

impl Token for Number {
    fn value(&self) -> String { self.0.to_string() }
    fn token_type(&self) -> &'static str { "number" }
}

impl Token for Punctuation {
    fn value(&self) -> String { self.0.to_string() }
    fn token_type(&self) -> &'static str { "punctuation" }
}

fn describe(token: &dyn Token) {
    println!("{}: {}", token.token_type(), token.value());
}

fn main() {
    let tokens: Vec<Box<dyn Token>> = vec![
        Box::new(Identifier("variable".to_string())),
        Box::new(Number(42)),
        Box::new(Punctuation(';')),
    ];

    for tok in &tokens {
        describe(tok.as_ref());
    }

    // Cannot implement Token for an external type:
    // struct MyToken;
    // impl private::Sealed for MyToken {} // ERROR: private
    // impl Token for MyToken {}            // ERROR: Sealed not impl'd
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sealed_token() {
        let id = Identifier("foo".to_string());
        assert_eq!(id.token_type(), "identifier");
        assert_eq!(id.value(), "foo");
    }

    #[test]
    fn test_number_token() {
        let n = Number(-99);
        assert_eq!(n.token_type(), "number");
        assert_eq!(n.value(), "-99");
    }
}
