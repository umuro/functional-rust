// 762. Custom Deserialization with Visitor Pattern
// Implements the core serde Visitor mechanism from scratch

// ── Error ──────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DeError {
    InvalidType { got: &'static str, expected: &'static str },
    MissingField(&'static str),
    ParseError(String),
    Custom(String),
}

impl std::fmt::Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidType { got, expected } => write!(f, "expected {expected}, got {got}"),
            Self::MissingField(n) => write!(f, "missing field `{n}`"),
            Self::ParseError(s) => write!(f, "parse error: {s}"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}

// ── Token (what the deserializer produces) ────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Str(&'a str),
    Int(i64),
    Float(f64),
    Bool(bool),
    Map(Vec<(&'a str, &'a str)>),
    Seq(Vec<&'a str>),
}

// ── Visitor trait ──────────────────────────────────────────────────────────────

pub trait Visitor<'de>: Sized {
    type Value;
    fn expecting(&self) -> &'static str;

    fn visit_str(self, v: &'de str) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "str", expected: self.expecting() })
    }
    fn visit_i64(self, v: i64) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "i64", expected: self.expecting() })
    }
    fn visit_f64(self, v: f64) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "f64", expected: self.expecting() })
    }
    fn visit_bool(self, v: bool) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "bool", expected: self.expecting() })
    }
    fn visit_map(self, m: Vec<(&'de str, &'de str)>) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "map", expected: self.expecting() })
    }
}

// ── Deserializer ──────────────────────────────────────────────────────────────

pub struct SimpleDeserializer<'de>(&'de str);

impl<'de> SimpleDeserializer<'de> {
    pub fn new(s: &'de str) -> Self { Self(s) }

    pub fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, DeError> {
        let s = self.0;
        if let Some(rest) = s.strip_prefix("str:") {
            visitor.visit_str(rest)
        } else if let Some(rest) = s.strip_prefix("int:") {
            let n = rest.parse::<i64>().map_err(|e| DeError::ParseError(e.to_string()))?;
            visitor.visit_i64(n)
        } else if let Some(rest) = s.strip_prefix("float:") {
            let f = rest.parse::<f64>().map_err(|e| DeError::ParseError(e.to_string()))?;
            visitor.visit_f64(f)
        } else if s == "true" {
            visitor.visit_bool(true)
        } else if s == "false" {
            visitor.visit_bool(false)
        } else if let Some(rest) = s.strip_prefix("map:") {
            let pairs: Vec<(&'de str, &'de str)> = rest
                .split(',')
                .filter_map(|p| {
                    let mut it = p.splitn(2, '=');
                    Some((it.next()?, it.next()?))
                })
                .collect();
            visitor.visit_map(pairs)
        } else {
            visitor.visit_str(s) // fallback
        }
    }
}

// ── Domain type and its Visitor ───────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub struct PersonVisitor;

impl<'de> Visitor<'de> for PersonVisitor {
    type Value = Person;
    fn expecting(&self) -> &'static str { "a map with name and age" }

    fn visit_map(self, m: Vec<(&'de str, &'de str)>) -> Result<Person, DeError> {
        let name = m.iter().find(|(k, _)| *k == "name")
            .map(|(_, v)| v.to_string())
            .ok_or(DeError::MissingField("name"))?;
        let age_str = m.iter().find(|(k, _)| *k == "age")
            .map(|(_, v)| *v)
            .ok_or(DeError::MissingField("age"))?;
        let age = age_str.parse::<u32>().map_err(|e| DeError::ParseError(e.to_string()))?;
        Ok(Person { name, age })
    }
}

pub trait Deserialize<'de>: Sized {
    fn deserialize(de: SimpleDeserializer<'de>) -> Result<Self, DeError>;
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize(de: SimpleDeserializer<'de>) -> Result<Self, DeError> {
        de.deserialize_any(PersonVisitor)
    }
}

fn main() {
    let wire = "map:name=Alice,age=30";
    let de = SimpleDeserializer::new(wire);
    match Person::deserialize(de) {
        Ok(p)  => println!("Got: {p:?}"),
        Err(e) => println!("Error: {e}"),
    }

    // Wrong type — should error with helpful message
    let wrong = SimpleDeserializer::new("str:hello");
    match Person::deserialize(wrong) {
        Ok(_)  => println!("unexpected ok"),
        Err(e) => println!("Expected error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_person_from_map() {
        let de = SimpleDeserializer::new("map:name=Bob,age=25");
        let p = Person::deserialize(de).unwrap();
        assert_eq!(p, Person { name: "Bob".into(), age: 25 });
    }

    #[test]
    fn wrong_type_returns_error() {
        let de = SimpleDeserializer::new("str:notamap");
        let result = Person::deserialize(de);
        assert!(matches!(result, Err(DeError::InvalidType { .. })));
    }

    #[test]
    fn missing_age_returns_error() {
        let de = SimpleDeserializer::new("map:name=Alice");
        let result = Person::deserialize(de);
        assert!(matches!(result, Err(DeError::MissingField("age"))));
    }
}
