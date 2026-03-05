//! # Lens Pattern
//! Focus on a field within a struct.

#[derive(Clone, Debug)]
pub struct Address { pub city: String, pub zip: String }
#[derive(Clone, Debug)]
pub struct User { pub name: String, pub addr: Address }

pub fn get_city(u: &User) -> &str { &u.addr.city }
pub fn set_city(u: &User, city: String) -> User {
    User { addr: Address { city, ..u.addr.clone() }, ..u.clone() }
}

pub fn modify_city(u: &User, f: impl Fn(&str) -> String) -> User {
    set_city(u, f(get_city(u)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_modify() {
        let u = User { name: "Bob".into(), addr: Address { city: "NYC".into(), zip: "10001".into() }};
        let u2 = modify_city(&u, |c| c.to_lowercase());
        assert_eq!(u2.addr.city, "nyc");
    }
}
