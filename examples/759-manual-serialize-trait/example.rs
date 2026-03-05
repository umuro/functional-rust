// 759. Manual Serialize/Deserialize Trait Implementation
// std-only — no external crates

use std::collections::HashMap;
use std::fmt;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum SerError {
    InvalidFormat(String),
    MissingField(String),
    ParseError(String),
}

impl fmt::Display for SerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat(s) => write!(f, "InvalidFormat: {s}"),
            Self::MissingField(s) => write!(f, "MissingField: {s}"),
            Self::ParseError(s) => write!(f, "ParseError: {s}"),
        }
    }
}

// ── Traits ────────────────────────────────────────────────────────────────────

/// Something that can turn itself into a key=value string.
pub trait Serialize {
    fn serialize(&self) -> String;
}

/// Something that can reconstruct itself from a key=value string.
pub trait Deserialize: Sized {
    fn deserialize(input: &str) -> Result<Self, SerError>;
}

// ── Wire-format helpers ───────────────────────────────────────────────────────

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c == '|' || c == '\\' {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn parse_fields(input: &str) -> HashMap<&str, String> {
    let mut map = HashMap::new();
    for field in input.split('|') {
        if let Some(eq) = field.find('=') {
            let key = &field[..eq];
            let val = unescape(&field[eq + 1..]);
            map.insert(key, val);
        }
    }
    map
}

// ── Domain type ───────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub active: bool,
}

impl Serialize for Person {
    fn serialize(&self) -> String {
        format!(
            "name={}|age={}|active={}",
            escape(&self.name),
            self.age,
            self.active
        )
    }
}

impl Deserialize for Person {
    fn deserialize(input: &str) -> Result<Self, SerError> {
        let fields = parse_fields(input);

        let name = fields
            .get("name")
            .ok_or_else(|| SerError::MissingField("name".into()))?
            .clone();

        let age = fields
            .get("age")
            .ok_or_else(|| SerError::MissingField("age".into()))?
            .parse::<u32>()
            .map_err(|e| SerError::ParseError(e.to_string()))?;

        let active = fields
            .get("active")
            .ok_or_else(|| SerError::MissingField("active".into()))?
            .parse::<bool>()
            .map_err(|e| SerError::ParseError(e.to_string()))?;

        Ok(Person { name, age, active })
    }
}

// ── Generic round-trip helper ─────────────────────────────────────────────────

fn round_trip<T>(value: &T) -> Result<T, SerError>
where
    T: Serialize + Deserialize + std::fmt::Debug,
{
    let encoded = value.serialize();
    T::deserialize(&encoded)
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let alice = Person {
        name: "Alice|Wonder".to_string(),
        age: 30,
        active: true,
    };

    let encoded = alice.serialize();
    println!("Encoded : {encoded}");

    match Person::deserialize(&encoded) {
        Ok(p) => println!("Decoded : {p:?}"),
        Err(e) => println!("Error   : {e}"),
    }

    // Generic helper
    let bob = Person { name: "Bob".to_string(), age: 25, active: false };
    let bob2 = round_trip(&bob).expect("round-trip failed");
    println!("Round-trip Bob: {bob2:?}");
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_normal() {
        let p = Person { name: "Carol".to_string(), age: 42, active: true };
        let encoded = p.serialize();
        let decoded = Person::deserialize(&encoded).unwrap();
        assert_eq!(p, decoded);
    }

    #[test]
    fn round_trip_special_chars() {
        let p = Person { name: "Pi|pe".to_string(), age: 1, active: false };
        let encoded = p.serialize();
        let decoded = Person::deserialize(&encoded).unwrap();
        assert_eq!(p, decoded);
    }

    #[test]
    fn missing_field_error() {
        let result = Person::deserialize("name=Eve|active=true");
        assert!(matches!(result, Err(SerError::MissingField(_))));
    }

    #[test]
    fn bad_age_parse() {
        let result = Person::deserialize("name=Eve|age=notanumber|active=true");
        assert!(matches!(result, Err(SerError::ParseError(_))));
    }
}
