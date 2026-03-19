// Smart constructors: enforce invariants at the type level.
// The type is opaque — you can only create values through validated constructors.

// ── NonEmptyString ──────────────────────────────────────────────────────────

/// A string guaranteed to be non-empty.
/// The inner field is private; construction goes through `create`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn create(s: &str) -> Result<Self, String> {
        if !s.is_empty() {
            Ok(NonEmptyString(s.to_string()))
        } else {
            Err("string must be non-empty".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Concatenate two NonEmptyStrings — result is always non-empty.
    pub fn concat(&self, other: &NonEmptyString) -> NonEmptyString {
        NonEmptyString(format!("{}{}", self.0, other.0))
    }
}

impl std::fmt::Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── PositiveInt ─────────────────────────────────────────────────────────────

/// An integer guaranteed to be strictly positive (> 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositiveInt(i64);

impl PositiveInt {
    pub fn create(n: i64) -> Result<Self, String> {
        if n > 0 {
            Ok(PositiveInt(n))
        } else {
            Err(format!("{} is not positive", n))
        }
    }

    pub fn value(self) -> i64 {
        self.0
    }

    /// Addition of two PositiveInts — result is always positive.
    pub fn add(self, other: Self) -> Self {
        PositiveInt(self.0 + other.0)
    }

    /// Multiplication of two PositiveInts — result is always positive.
    pub fn mul(self, other: Self) -> Self {
        PositiveInt(self.0 * other.0)
    }
}

impl std::fmt::Display for PositiveInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── Validated accumulating error type ───────────────────────────────────────
// Goes beyond the OCaml example: a Validated<T> that collects ALL errors.

#[derive(Debug, PartialEq)]
pub enum Validated<T> {
    Ok(T),
    Err(Vec<String>),
}

impl<T> Validated<T> {
    pub fn ok(v: T) -> Self {
        Validated::Ok(v)
    }

    pub fn err(e: impl Into<String>) -> Self {
        Validated::Err(vec![e.into()])
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Validated::Ok(_))
    }

    /// Map over a successful value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U> {
        match self {
            Validated::Ok(v) => Validated::Ok(f(v)),
            Validated::Err(es) => Validated::Err(es),
        }
    }

    /// Combine two Validated values, collecting errors from both.
    pub fn and<U>(self, other: Validated<U>) -> Validated<(T, U)> {
        match (self, other) {
            (Validated::Ok(a), Validated::Ok(b)) => Validated::Ok((a, b)),
            (Validated::Err(mut e1), Validated::Err(e2)) => {
                e1.extend(e2);
                Validated::Err(e1)
            }
            (Validated::Err(e), _) | (_, Validated::Err(e)) => Validated::Err(e),
        }
    }

    pub fn errors(&self) -> Option<&[String]> {
        match self {
            Validated::Err(es) => Some(es),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_string_ok() {
        let s = NonEmptyString::create("hello").unwrap();
        assert_eq!(s.value(), "hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_non_empty_string_err() {
        assert!(NonEmptyString::create("").is_err());
    }

    #[test]
    fn test_positive_int_ok() {
        let n = PositiveInt::create(42).unwrap();
        assert_eq!(n.value(), 42);
    }

    #[test]
    fn test_positive_int_err() {
        assert!(PositiveInt::create(0).is_err());
        assert!(PositiveInt::create(-5).is_err());
    }

    #[test]
    fn test_positive_int_add() {
        let a = PositiveInt::create(3).unwrap();
        let b = PositiveInt::create(4).unwrap();
        assert_eq!(a.add(b).value(), 7);
    }

    #[test]
    fn test_validated_accumulates_errors() {
        let v1: Validated<i32> = Validated::err("error 1");
        let v2: Validated<i32> = Validated::err("error 2");
        let combined = v1.and(v2);
        assert_eq!(combined.errors().unwrap().len(), 2);
    }

    #[test]
    fn test_validated_ok() {
        let v1 = Validated::ok(1_i32);
        let v2 = Validated::ok(2_i32);
        assert_eq!(v1.and(v2), Validated::Ok((1, 2)));
    }
}
