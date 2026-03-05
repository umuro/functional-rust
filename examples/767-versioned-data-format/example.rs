// 767. Versioned Serialization with Migration
// Version tag in payload, migration chain

#[derive(Debug, PartialEq)]
pub struct UserV1 {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserV2 {
    pub name: String,
    pub age: u32,
    pub email: String,    // new in V2
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserV3 {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub active: bool,     // new in V3
}

// ── Migration chain ────────────────────────────────────────────────────────────

impl From<UserV1> for UserV2 {
    fn from(u: UserV1) -> Self {
        UserV2 {
            email: format!("{}@example.com", u.name.to_lowercase().replace(' ', ".")),
            name: u.name,
            age: u.age,
        }
    }
}

impl From<UserV2> for UserV3 {
    fn from(u: UserV2) -> Self {
        UserV3 {
            name: u.name,
            age: u.age,
            email: u.email,
            active: true, // sensible default for migrated records
        }
    }
}

impl From<UserV1> for UserV3 {
    fn from(u: UserV1) -> Self {
        UserV3::from(UserV2::from(u))
    }
}

// ── Versioned enum ─────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum VersionedUser {
    V1(UserV1),
    V2(UserV2),
    V3(UserV3),
}

impl VersionedUser {
    /// Always get the latest version (migrate if needed)
    pub fn into_current(self) -> UserV3 {
        match self {
            VersionedUser::V1(u) => UserV3::from(u),
            VersionedUser::V2(u) => UserV3::from(u),
            VersionedUser::V3(u) => u,
        }
    }
}

// ── Serialization ──────────────────────────────────────────────────────────────

pub fn serialize_v3(u: &UserV3) -> String {
    format!("version=3|name={}|age={}|email={}|active={}", u.name, u.age, u.email, u.active)
}
pub fn serialize_v1(u: &UserV1) -> String {
    format!("version=1|name={}|age={}", u.name, u.age)
}

fn fields(s: &str) -> std::collections::HashMap<&str, &str> {
    s.split('|').filter_map(|p| p.split_once('=')).collect()
}

// ── Deserialization ────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum DeError { MissingField(&'static str), UnsupportedVersion(String), ParseError(String) }

pub fn deserialize(s: &str) -> Result<VersionedUser, DeError> {
    let map = fields(s);
    match *map.get("version").ok_or(DeError::MissingField("version"))? {
        "1" => {
            let name = map.get("name").ok_or(DeError::MissingField("name"))?.to_string();
            let age  = map.get("age").ok_or(DeError::MissingField("age"))?
                .parse().map_err(|e: std::num::ParseIntError| DeError::ParseError(e.to_string()))?;
            Ok(VersionedUser::V1(UserV1 { name, age }))
        }
        "2" => {
            let name  = map.get("name").ok_or(DeError::MissingField("name"))?.to_string();
            let age   = map.get("age").ok_or(DeError::MissingField("age"))?
                .parse().map_err(|e: std::num::ParseIntError| DeError::ParseError(e.to_string()))?;
            let email = map.get("email").ok_or(DeError::MissingField("email"))?.to_string();
            Ok(VersionedUser::V2(UserV2 { name, age, email }))
        }
        "3" => {
            let name   = map.get("name").ok_or(DeError::MissingField("name"))?.to_string();
            let age    = map.get("age").ok_or(DeError::MissingField("age"))?
                .parse().map_err(|e: std::num::ParseIntError| DeError::ParseError(e.to_string()))?;
            let email  = map.get("email").ok_or(DeError::MissingField("email"))?.to_string();
            let active = map.get("active").map(|v| *v == "true").unwrap_or(true);
            Ok(VersionedUser::V3(UserV3 { name, age, email, active }))
        }
        v => Err(DeError::UnsupportedVersion(v.to_string())),
    }
}

fn main() {
    // Simulate reading old V1 data and migrating to V3
    let old = UserV1 { name: "Alice".into(), age: 30 };
    let wire = serialize_v1(&old);
    println!("Old wire: {wire}");

    let versioned = deserialize(&wire).expect("decode failed");
    let current = versioned.into_current();
    println!("Migrated to V3: {current:?}");

    // Current format round-trip
    let wire3 = serialize_v3(&current);
    println!("V3 wire: {wire3}");
    let back = deserialize(&wire3).expect("v3 decode").into_current();
    println!("V3 round-trip: {back:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v1_migrates_to_v3() {
        let u1 = UserV1 { name: "Bob".into(), age: 25 };
        let wire = serialize_v1(&u1);
        let v3 = deserialize(&wire).unwrap().into_current();
        assert_eq!(v3.name, "Bob");
        assert_eq!(v3.age, 25);
        assert!(v3.email.contains("bob"));
        assert!(v3.active);
    }

    #[test]
    fn unknown_version_errors() {
        let result = deserialize("version=99|name=X|age=1");
        assert!(matches!(result, Err(DeError::UnsupportedVersion(_))));
    }

    #[test]
    fn v3_round_trip() {
        let u = UserV3 { name: "Carol".into(), age: 35, email: "c@test.com".into(), active: false };
        let wire = serialize_v3(&u);
        let back = deserialize(&wire).unwrap().into_current();
        assert_eq!(back, u);
    }
}
