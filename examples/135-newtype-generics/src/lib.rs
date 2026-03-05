//! Example 135: Generic Newtype Patterns
//!
//! Wrap primitives and collections in named types to prevent mix-ups, add
//! invariants, and give behaviour to types you don't own.
//!
//! # Approaches
//!
//! 1. **Validated newtypes** — construction validates an invariant; the type
//!    proves validity to every caller.
//! 2. **Generic validated wrapper** — a single `Validated<T, V>` struct
//!    parameterised by a *validator* trait, mirroring OCaml's functor pattern.
//! 3. **Transparent newtypes via `Deref`** — expose the inner API without
//!    boilerplate while still keeping the distinct type.

use std::fmt;
use std::marker::PhantomData;
use std::ops::Deref;

// ── Approach 1: Validated domain newtypes ─────────────────────────────────────

/// A validated e-mail address.  Can only be constructed through `Email::new`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Returns `Ok(Email)` when `s` contains `'@'`, otherwise `Err`.
    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.contains('@') {
            Ok(Email(s.to_owned()))
        } else {
            Err("invalid email: missing '@'")
        }
    }

    /// Borrow the inner string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A validated username (>= 3 characters).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    /// Returns `Ok(Username)` when `s` has at least 3 characters, otherwise `Err`.
    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.len() >= 3 {
            Ok(Username(s.to_owned()))
        } else {
            Err("username too short (< 3 chars)")
        }
    }
}

/// Transparent access to `str` methods without boilerplate forwarding.
impl Deref for Username {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// Typed IDs -- same underlying type, distinct at compile time.

/// A user's unique identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(pub u64);

/// A product's unique identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(pub u64);

/// Accept only a `UserId`, never a raw `u64` or a `ProductId`.
pub fn find_user(id: UserId) -> String {
    format!("user:{}", id.0)
}

// ── Approach 2: Generic validated wrapper (OCaml functor parallel) ────────────

/// A validation strategy.  Implement this for a zero-sized marker type.
pub trait Validator<T> {
    type Error: fmt::Debug + fmt::Display;
    fn validate(value: &T) -> Result<(), Self::Error>;
}

/// A value that has been checked by `V`.
///
/// Construction is the only path to a `Validated<T, V>` -- no public field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validated<T, V>(T, PhantomData<V>);

impl<T, V: Validator<T>> Validated<T, V> {
    /// Run the validator; return the wrapped value on success.
    pub fn new(value: T) -> Result<Self, V::Error> {
        V::validate(&value)?;
        Ok(Validated(value, PhantomData))
    }

    /// Borrow the inner value.
    pub fn inner(&self) -> &T {
        &self.0
    }

    /// Consume and return the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: fmt::Display, V> fmt::Display for Validated<T, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Validator: integer must be positive.
pub struct Positive;

impl Validator<i64> for Positive {
    type Error = String;
    fn validate(value: &i64) -> Result<(), String> {
        if *value > 0 {
            Ok(())
        } else {
            Err(format!("{value} is not positive"))
        }
    }
}

/// Validator: string must be non-empty.
pub struct NonEmpty;

impl Validator<String> for NonEmpty {
    type Error = &'static str;
    fn validate(value: &String) -> Result<(), &'static str> {
        if value.is_empty() {
            Err("value must not be empty")
        } else {
            Ok(())
        }
    }
}

/// A positive integer, guaranteed by the type.
pub type PositiveInt = Validated<i64, Positive>;

/// A non-empty string, guaranteed by the type.
pub type NonEmptyStr = Validated<String, NonEmpty>;

// ── Approach 3: Newtype over a collection ─────────────────────────────────────

/// An ordered list of scores.  Exposes read-only slice access via `Deref` but
/// controls mutation so invariants (e.g. sorted order) can be enforced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreList(Vec<u32>);

impl ScoreList {
    pub fn new(scores: Vec<u32>) -> Self {
        ScoreList(scores)
    }

    /// Push a score and keep the list sorted.
    pub fn insert_sorted(&mut self, score: u32) {
        let pos = self.0.partition_point(|&s| s <= score);
        self.0.insert(pos, score);
    }

    /// Arithmetic mean, or `None` when the list is empty.
    pub fn mean(&self) -> Option<f64> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.iter().map(|&s| s as f64).sum::<f64>() / self.0.len() as f64)
        }
    }
}

impl Deref for ScoreList {
    type Target = [u32];
    fn deref(&self) -> &[u32] {
        &self.0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // -- Email -----------------------------------------------------------------

    #[test]
    fn email_valid_accepted() {
        let e = Email::new("alice@example.com").unwrap();
        assert_eq!(e.as_str(), "alice@example.com");
    }

    #[test]
    fn email_missing_at_rejected() {
        assert!(Email::new("notanemail").is_err());
    }

    #[test]
    fn email_display_shows_address() {
        let e = Email::new("x@y.com").unwrap();
        assert_eq!(e.to_string(), "x@y.com");
    }

    // -- Username --------------------------------------------------------------

    #[test]
    fn username_valid_accepted() {
        let u = Username::new("bob").unwrap();
        // Deref lets us call str methods directly.
        assert_eq!(u.to_uppercase(), "BOB");
    }

    #[test]
    fn username_too_short_rejected() {
        assert!(Username::new("ab").is_err());
        assert!(Username::new("").is_err());
    }

    // -- Typed IDs -- no accidental swap ---------------------------------------

    #[test]
    fn typed_ids_are_distinct_in_find_user() {
        let uid = UserId(42);
        // find_user(ProductId(42)) would be a compile error -- types differ.
        assert_eq!(find_user(uid), "user:42");
    }

    #[test]
    fn typed_ids_ordering() {
        assert!(UserId(1) < UserId(2));
        assert!(ProductId(10) > ProductId(5));
    }

    // -- Generic Validated wrapper ---------------------------------------------

    #[test]
    fn positive_int_accepts_positive() {
        let n = PositiveInt::new(7).unwrap();
        assert_eq!(*n.inner(), 7);
    }

    #[test]
    fn positive_int_rejects_zero_and_negative() {
        assert!(PositiveInt::new(0).is_err());
        assert!(PositiveInt::new(-3).is_err());
    }

    #[test]
    fn positive_int_into_inner() {
        let n = PositiveInt::new(99).unwrap();
        assert_eq!(n.into_inner(), 99_i64);
    }

    #[test]
    fn non_empty_str_accepts_content() {
        let s = NonEmptyStr::new("hello".to_owned()).unwrap();
        assert_eq!(s.inner(), "hello");
    }

    #[test]
    fn non_empty_str_rejects_empty() {
        assert!(NonEmptyStr::new(String::new()).is_err());
    }

    // -- ScoreList -------------------------------------------------------------

    #[test]
    fn score_list_mean_empty_is_none() {
        let sl = ScoreList::new(vec![]);
        assert_eq!(sl.mean(), None);
    }

    #[test]
    fn score_list_mean_computed_correctly() {
        let sl = ScoreList::new(vec![10, 20, 30]);
        assert!((sl.mean().unwrap() - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn score_list_insert_sorted_maintains_order() {
        let mut sl = ScoreList::new(vec![10, 30, 50]);
        sl.insert_sorted(25);
        sl.insert_sorted(5);
        assert_eq!(&*sl, &[5, 10, 25, 30, 50]);
    }

    #[test]
    fn score_list_deref_gives_slice_access() {
        let sl = ScoreList::new(vec![1, 2, 3]);
        // `.len()` and `.iter()` come from `Deref<Target = [u32]>`.
        assert_eq!(sl.len(), 3);
        let doubled: Vec<u32> = sl.iter().map(|&s| s * 2).collect();
        assert_eq!(doubled, [2, 4, 6]);
    }
}
