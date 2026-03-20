#![allow(clippy::all)]
// Example 209: Affine Traversal — At Most One Focus
//
// An affine traversal focuses on at most one value: `preview` returns
// `Option<A>`, and `over`/`set` are no-ops when the target is absent.
//
// It combines the "might not exist" of a Prism with the "exactly one"
// of a Lens.  Typical use cases: optional record fields, HashMap lookups,
// the head of a possibly-empty Vec.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Core Affine struct
// ---------------------------------------------------------------------------

type PreviewFn<S, A> = Box<dyn Fn(&S) -> Option<A>>;
type SetFn<S, A> = Box<dyn Fn(A, &S) -> S>;

/// An affine traversal: `preview` extracts at most one value; `set` replaces
/// it when present and is a structural no-op otherwise.
pub struct Affine<S, A> {
    preview: PreviewFn<S, A>,
    set: SetFn<S, A>,
}

impl<S: Clone + 'static, A: 'static> Affine<S, A> {
    pub fn new(
        preview: impl Fn(&S) -> Option<A> + 'static,
        set: impl Fn(A, &S) -> S + 'static,
    ) -> Self {
        Affine {
            preview: Box::new(preview),
            set: Box::new(set),
        }
    }

    /// Extract the focus value, returning `None` if absent.
    pub fn preview(&self, s: &S) -> Option<A> {
        (self.preview)(s)
    }

    /// Replace the focus value; if absent, returns a clone of `s` unchanged.
    pub fn set(&self, a: A, s: &S) -> S {
        (self.set)(a, s)
    }

    /// Apply `f` to the focus value; if absent, returns a clone of `s`.
    pub fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        match (self.preview)(s) {
            Some(v) => (self.set)(f(v), s),
            None => s.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Approach 1: Affine for optional record fields
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// Affine traversal targeting `User::email`.
pub fn email_affine() -> Affine<User, String> {
    Affine::new(
        |u: &User| u.email.clone(),
        |e, u: &User| User {
            email: Some(e),
            ..u.clone()
        },
    )
}

/// Affine traversal targeting `User::phone`.
pub fn phone_affine() -> Affine<User, String> {
    Affine::new(
        |u: &User| u.phone.clone(),
        |p, u: &User| User {
            phone: Some(p),
            ..u.clone()
        },
    )
}

// ---------------------------------------------------------------------------
// Approach 2: Affine for HashMap key lookups
// ---------------------------------------------------------------------------

/// Build an affine traversal that focuses on a specific key in a `HashMap`.
///
/// The key is captured at construction time, matching the OCaml approach of
/// returning a record with closures that close over the key.
pub fn map_key_affine(key: &str) -> Affine<HashMap<String, String>, String> {
    let k_preview = key.to_string();
    let k_set = key.to_string();
    Affine::new(
        move |m: &HashMap<String, String>| m.get(&k_preview).cloned(),
        move |v, m: &HashMap<String, String>| {
            let mut out = m.clone();
            out.insert(k_set.clone(), v);
            out
        },
    )
}

// ---------------------------------------------------------------------------
// Approach 3: Affine for the head of a Vec
// ---------------------------------------------------------------------------

/// Affine traversal targeting the first element of a `Vec<i32>`.
/// `over` on an empty vec is a no-op.
pub fn vec_head_affine() -> Affine<Vec<i32>, i32> {
    Affine::new(
        |v: &Vec<i32>| v.first().copied(),
        |x, v: &Vec<i32>| {
            if v.is_empty() {
                v.clone()
            } else {
                let mut out = v.clone();
                out[0] = x;
                out
            }
        },
    )
}

// ---------------------------------------------------------------------------
// Approach 4: Trait-based zero-cost affine (no heap allocation)
// ---------------------------------------------------------------------------

/// Implement to get compile-time dispatch affine traversal behaviour.
pub trait AffineTraversal {
    type Source: Clone;
    type Focus;

    fn preview(s: &Self::Source) -> Option<Self::Focus>;
    fn set(a: Self::Focus, s: &Self::Source) -> Self::Source;

    fn over(f: impl FnOnce(Self::Focus) -> Self::Focus, s: &Self::Source) -> Self::Source {
        match Self::preview(s) {
            Some(v) => Self::set(f(v), s),
            None => s.clone(),
        }
    }
}

/// Zero-cost affine for `User::email` via the trait.
pub struct UserEmailAffine;

impl AffineTraversal for UserEmailAffine {
    type Source = User;
    type Focus = String;

    fn preview(u: &User) -> Option<String> {
        u.email.clone()
    }

    fn set(e: String, u: &User) -> User {
        User {
            email: Some(e),
            ..u.clone()
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Approach 1: optional record fields ---

    #[test]
    fn test_email_preview_present() {
        let u = User {
            name: "Alice".to_string(),
            email: Some("alice@example.com".to_string()),
            phone: None,
        };
        let aff = email_affine();
        assert_eq!(aff.preview(&u), Some("alice@example.com".to_string()));
    }

    #[test]
    fn test_email_preview_absent() {
        let u = User {
            name: "Bob".to_string(),
            email: None,
            phone: None,
        };
        let aff = email_affine();
        assert_eq!(aff.preview(&u), None);
    }

    #[test]
    fn test_email_set_when_present() {
        let u = User {
            name: "Alice".to_string(),
            email: Some("old@example.com".to_string()),
            phone: None,
        };
        let aff = email_affine();
        let updated = aff.set("new@example.com".to_string(), &u);
        assert_eq!(updated.email, Some("new@example.com".to_string()));
        assert_eq!(updated.name, "Alice");
    }

    #[test]
    fn test_email_set_when_absent_installs_value() {
        // set always writes; the affine does not guard on prior existence
        let u = User {
            name: "Bob".to_string(),
            email: None,
            phone: None,
        };
        let aff = email_affine();
        let updated = aff.set("bob@example.com".to_string(), &u);
        assert_eq!(updated.email, Some("bob@example.com".to_string()));
    }

    #[test]
    fn test_over_uppercases_present_email() {
        let u = User {
            name: "Alice".to_string(),
            email: Some("alice@example.com".to_string()),
            phone: None,
        };
        let aff = email_affine();
        let updated = aff.over(|e| e.to_uppercase(), &u);
        assert_eq!(updated.email, Some("ALICE@EXAMPLE.COM".to_string()));
        // Other fields untouched
        assert_eq!(updated.name, "Alice");
        assert_eq!(updated.phone, None);
    }

    #[test]
    fn test_over_is_noop_when_email_absent() {
        let u = User {
            name: "Bob".to_string(),
            email: None,
            phone: None,
        };
        let aff = email_affine();
        let updated = aff.over(|e| e.to_uppercase(), &u);
        assert_eq!(updated, u); // structurally unchanged
    }

    #[test]
    fn test_phone_affine_independent_of_email() {
        let u = User {
            name: "Carol".to_string(),
            email: Some("carol@example.com".to_string()),
            phone: Some("555-1234".to_string()),
        };
        let phone = phone_affine();
        let updated = phone.over(|p| p.replace('-', ""), &u);
        assert_eq!(updated.phone, Some("5551234".to_string()));
        assert_eq!(updated.email, Some("carol@example.com".to_string()));
    }

    // --- Approach 2: HashMap key ---

    #[test]
    fn test_map_key_preview_present() {
        let mut m = HashMap::new();
        m.insert("lang".to_string(), "rust".to_string());
        let aff = map_key_affine("lang");
        assert_eq!(aff.preview(&m), Some("rust".to_string()));
    }

    #[test]
    fn test_map_key_preview_absent() {
        let m: HashMap<String, String> = HashMap::new();
        let aff = map_key_affine("missing");
        assert_eq!(aff.preview(&m), None);
    }

    #[test]
    fn test_map_key_over_transforms_existing_value() {
        let mut m = HashMap::new();
        m.insert("greeting".to_string(), "hello".to_string());
        let aff = map_key_affine("greeting");
        let updated = aff.over(|v| v.to_uppercase(), &m);
        assert_eq!(updated.get("greeting"), Some(&"HELLO".to_string()));
    }

    #[test]
    fn test_map_key_over_noop_on_missing_key() {
        let m: HashMap<String, String> = HashMap::new();
        let aff = map_key_affine("nope");
        let updated = aff.over(|v| v.to_uppercase(), &m);
        assert!(updated.is_empty());
    }

    // --- Approach 3: Vec head ---

    #[test]
    fn test_vec_head_preview_nonempty() {
        let v = vec![10, 20, 30];
        let aff = vec_head_affine();
        assert_eq!(aff.preview(&v), Some(10));
    }

    #[test]
    fn test_vec_head_preview_empty() {
        let v: Vec<i32> = vec![];
        let aff = vec_head_affine();
        assert_eq!(aff.preview(&v), None);
    }

    #[test]
    fn test_vec_head_over_doubles_head() {
        let v = vec![3, 4, 5];
        let aff = vec_head_affine();
        let updated = aff.over(|x| x * 2, &v);
        assert_eq!(updated, vec![6, 4, 5]);
    }

    #[test]
    fn test_vec_head_over_noop_on_empty() {
        let v: Vec<i32> = vec![];
        let aff = vec_head_affine();
        let updated = aff.over(|x| x * 2, &v);
        assert_eq!(updated, v);
    }

    // --- Approach 4: trait-based ---

    #[test]
    fn test_trait_affine_preview_present() {
        let u = User {
            name: "Dana".to_string(),
            email: Some("dana@example.com".to_string()),
            phone: None,
        };
        assert_eq!(
            UserEmailAffine::preview(&u),
            Some("dana@example.com".to_string())
        );
    }

    #[test]
    fn test_trait_affine_preview_absent() {
        let u = User {
            name: "Eve".to_string(),
            email: None,
            phone: None,
        };
        assert_eq!(UserEmailAffine::preview(&u), None);
    }

    #[test]
    fn test_trait_affine_over_present() {
        let u = User {
            name: "Dana".to_string(),
            email: Some("dana@example.com".to_string()),
            phone: None,
        };
        let updated = UserEmailAffine::over(|e| e.to_uppercase(), &u);
        assert_eq!(updated.email, Some("DANA@EXAMPLE.COM".to_string()));
    }

    #[test]
    fn test_trait_affine_over_absent_noop() {
        let u = User {
            name: "Eve".to_string(),
            email: None,
            phone: None,
        };
        let updated = UserEmailAffine::over(|e| e.to_uppercase(), &u);
        assert_eq!(updated, u);
    }
}
