#![allow(clippy::all)]
//! Lifetime Coercion and Subtyping
//!
//! Longer lifetimes can be used where shorter ones are required.

/// Accepts a reference valid for at least 'short duration.
pub fn use_briefly<'short>(s: &'short str) -> usize {
    s.len()
}

/// Demonstrate implicit coercion: longer lifetime used as shorter.
pub fn coercion_demo() -> usize {
    // 'static is longer than any local lifetime
    let static_str: &'static str = "I live forever";

    // 'static coerces to 'short when passed to use_briefly
    use_briefly(static_str)
}

/// Store in a Vec with shorter lifetime requirement.
pub fn store_with_coercion<'short>(storage: &mut Vec<&'short str>, item: &'static str) {
    // 'static can be stored where 'short is expected
    storage.push(item);
}

/// Function demonstrating variance.
pub fn demonstrate_variance<'long: 'short, 'short>(
    long_ref: &'long str,
    _short_ref: &'short str,
) -> &'short str {
    // 'long can be used as 'short (covariance)
    long_ref
}

/// Reborrowing: creating a shorter borrow from a longer one.
pub fn reborrow_demo() {
    let owned = String::from("hello");
    let long_borrow: &str = &owned;

    // Reborrow with shorter lifetime
    let short_borrow: &str = long_borrow;
    assert_eq!(short_borrow, "hello");
}

/// Reference to reference coercion.
pub fn ref_ref_coercion<'a, 'b: 'a>(r: &'a &'b str) -> &'a str {
    *r
}

/// Struct that accepts shorter lifetime.
pub struct Holder<'a> {
    pub data: &'a str,
}

impl<'a> Holder<'a> {
    /// Can accept 'static (longer) for 'a.
    pub fn from_static(s: &'static str) -> Holder<'static> {
        Holder { data: s }
    }

    /// General constructor.
    pub fn new(data: &'a str) -> Self {
        Holder { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_to_short() {
        let result = coercion_demo();
        assert_eq!(result, 14); // "I live forever".len()
    }

    #[test]
    fn test_store_with_coercion() {
        let owned = String::from("local");
        let mut storage: Vec<&str> = vec![&owned];
        store_with_coercion(&mut storage, "static");
        assert_eq!(storage.len(), 2);
    }

    #[test]
    fn test_demonstrate_variance() {
        let long = String::from("long lived");
        {
            let short = String::from("short");
            let result = demonstrate_variance(&long, &short);
            assert_eq!(result, "long lived");
        }
    }

    #[test]
    fn test_reborrow() {
        let owned = String::from("test");
        let r1: &str = &owned;
        let r2: &str = r1; // reborrow
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_ref_ref() {
        let s = "hello";
        let r: &str = &s;
        let result = ref_ref_coercion(&r);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_holder_from_static() {
        let holder = Holder::from_static("static string");
        assert_eq!(holder.data, "static string");
    }

    #[test]
    fn test_holder_from_local() {
        let local = String::from("local");
        let holder = Holder::new(&local);
        assert_eq!(holder.data, "local");
    }
}
