//! Sealed Trait Pattern

mod private { pub trait Sealed {} }

pub trait Token: private::Sealed {
    fn value(&self) -> String;
    fn token_type(&self) -> &'static str;
}

pub struct Identifier(pub String);
pub struct Number(pub i64);

impl private::Sealed for Identifier {}
impl private::Sealed for Number {}

impl Token for Identifier {
    fn value(&self) -> String { self.0.clone() }
    fn token_type(&self) -> &'static str { "identifier" }
}

impl Token for Number {
    fn value(&self) -> String { self.0.to_string() }
    fn token_type(&self) -> &'static str { "number" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_identifier() {
        let id = Identifier("foo".into());
        assert_eq!(id.value(), "foo");
        assert_eq!(id.token_type(), "identifier");
    }
    #[test] fn test_number() {
        let n = Number(42);
        assert_eq!(n.value(), "42");
        assert_eq!(n.token_type(), "number");
    }
    #[test] fn test_trait_object() {
        let tokens: Vec<Box<dyn Token>> = vec![
            Box::new(Identifier("x".into())),
            Box::new(Number(1)),
        ];
        assert_eq!(tokens.len(), 2);
    }
}
