// 773. Serde Attributes: rename, skip, flatten — Manual Implementation
// Shows what #[serde(rename="id")] etc. expand into

use std::collections::HashMap;

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub struct Address {
    pub street: String,
    pub city:   String,
}

/// Fields have conceptual serde attributes (applied manually below)
#[derive(Debug, PartialEq)]
pub struct User {
    pub user_id:  u32,    // #[serde(rename = "id")]
    pub name:     String,
    pub password: String, // #[serde(skip)]
    pub address:  Address,// #[serde(flatten)]
}

// ── Traits ────────────────────────────────────────────────────────────────────

pub trait Serialize {
    fn serialize_to(&self, out: &mut HashMap<String, String>);
}

pub trait Deserialize: Sized {
    fn deserialize_from(map: &HashMap<String, String>) -> Option<Self>;
}

// ── Address: normal serialize ──────────────────────────────────────────────────

impl Serialize for Address {
    fn serialize_to(&self, out: &mut HashMap<String, String>) {
        out.insert("street".into(), self.street.clone());
        out.insert("city".into(),   self.city.clone());
    }
}

impl Deserialize for Address {
    fn deserialize_from(map: &HashMap<String, String>) -> Option<Self> {
        Some(Address {
            street: map.get("street")?.clone(),
            city:   map.get("city")?.clone(),
        })
    }
}

// ── User: applies rename, skip, flatten ───────────────────────────────────────

impl Serialize for User {
    fn serialize_to(&self, out: &mut HashMap<String, String>) {
        // rename: user_id → "id"
        out.insert("id".into(), self.user_id.to_string());

        // normal
        out.insert("name".into(), self.name.clone());

        // skip: password NOT inserted

        // flatten: merge Address fields directly into this map
        self.address.serialize_to(out);
    }
}

impl Deserialize for User {
    fn deserialize_from(map: &HashMap<String, String>) -> Option<Self> {
        // rename: "id" → user_id
        let user_id = map.get("id")?.parse().ok()?;

        let name = map.get("name")?.clone();

        // skip: default for password
        let password = String::new(); // #[serde(default)]

        // flatten: Address reads from the same map
        let address = Address::deserialize_from(map)?;

        Some(User { user_id, name, password, address })
    }
}

// ── Wire format helpers ────────────────────────────────────────────────────────

fn to_wire<T: Serialize>(v: &T) -> String {
    let mut map = HashMap::new();
    v.serialize_to(&mut map);
    let mut pairs: Vec<_> = map.iter().map(|(k, v)| format!("{k}={v}")).collect();
    pairs.sort();
    pairs.join("|")
}

fn from_wire<T: Deserialize>(s: &str) -> Option<T> {
    let map: HashMap<String, String> = s.split('|')
        .filter_map(|p| p.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    T::deserialize_from(&map)
}

fn main() {
    let user = User {
        user_id:  42,
        name:     "Alice".into(),
        password: "s3cr3t".into(),  // will be skipped!
        address:  Address { street: "Main St 1".into(), city: "Berlin".into() },
    };

    let wire = to_wire(&user);
    println!("Wire: {wire}");
    println!("(notice: no 'password' field — it was skipped)");

    let user2: User = from_wire(&wire).expect("deserialize failed");
    println!("\nDeserialized:");
    println!("  user_id  = {} (was renamed from 'id')", user2.user_id);
    println!("  name     = {}", user2.name);
    println!("  password = {:?} (was skipped → default empty)", user2.password);
    println!("  address  = {:?} (was flattened)", user2.address);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> User {
        User {
            user_id: 1,
            name: "Bob".into(),
            password: "secret".into(),
            address: Address { street: "Baker St".into(), city: "London".into() },
        }
    }

    #[test]
    fn password_not_in_wire() {
        let wire = to_wire(&sample());
        assert!(!wire.contains("password"), "password should be skipped");
        assert!(!wire.contains("secret"),   "password value should not appear");
    }

    #[test]
    fn rename_id_in_wire() {
        let wire = to_wire(&sample());
        assert!(wire.contains("id="), "should use renamed key 'id'");
        assert!(!wire.contains("user_id="), "original key name should not appear");
    }

    #[test]
    fn flatten_address_in_wire() {
        let wire = to_wire(&sample());
        assert!(wire.contains("city=London"));
        assert!(wire.contains("street=Baker"));
    }

    #[test]
    fn round_trip_without_password() {
        let u = sample();
        let wire = to_wire(&u);
        let u2: User = from_wire(&wire).unwrap();
        assert_eq!(u2.user_id, u.user_id);
        assert_eq!(u2.name, u.name);
        assert_eq!(u2.password, ""); // skipped → default
        assert_eq!(u2.address, u.address);
    }
}
