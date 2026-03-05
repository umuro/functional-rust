//! 709 — Unions in Rust: C-style Tagged Unions
//!
//! Raw `union` + enum tag = safe tagged union.
//! This is exactly what OCaml's algebraic data types are at the hardware level,
//! except OCaml hides the tag and dispatch from you. Here we write it explicitly.

// ---------------------------------------------------------------------------
// Raw union — all fields overlap at the same memory address.
// Only usable inside `unsafe` blocks.
// ---------------------------------------------------------------------------

/// Untagged union: all fields share the same memory location.
/// Reading the wrong field after writing another is undefined behaviour.
#[repr(C)]
union RawValue {
    int_val: i64,
    float_val: f64,
    bool_val: u8,
}

// ---------------------------------------------------------------------------
// Tag enum — tracks which field of the union is currently valid.
// ---------------------------------------------------------------------------

/// Discriminant tracking which field is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Int,
    Float,
    Bool,
}

// ---------------------------------------------------------------------------
// Safe tagged union — pairs the raw union with its discriminant.
// All unsafe access is confined to these methods.
// ---------------------------------------------------------------------------

/// Safe tagged union: an enum tag guards all reads of the raw union.
pub struct Value {
    tag: Tag,
    data: RawValue,
}

impl Value {
    /// Construct a `Value` holding an integer.
    pub fn int(n: i64) -> Self {
        Value {
            tag: Tag::Int,
            data: RawValue { int_val: n },
        }
    }

    /// Construct a `Value` holding a float.
    pub fn float(f: f64) -> Self {
        Value {
            tag: Tag::Float,
            data: RawValue { float_val: f },
        }
    }

    /// Construct a `Value` holding a boolean.
    pub fn bool(b: bool) -> Self {
        Value {
            tag: Tag::Bool,
            data: RawValue { bool_val: b as u8 },
        }
    }

    /// Return the integer if the tag is `Int`, otherwise `None`.
    pub fn as_int(&self) -> Option<i64> {
        if self.tag == Tag::Int {
            // SAFETY: we just checked the tag is Int, so int_val was the last
            // field written and its bits are valid for i64.
            Some(unsafe { self.data.int_val })
        } else {
            None
        }
    }

    /// Return the float if the tag is `Float`, otherwise `None`.
    pub fn as_float(&self) -> Option<f64> {
        if self.tag == Tag::Float {
            // SAFETY: tag is Float, so float_val is the active field.
            Some(unsafe { self.data.float_val })
        } else {
            None
        }
    }

    /// Return the bool if the tag is `Bool`, otherwise `None`.
    pub fn as_bool(&self) -> Option<bool> {
        if self.tag == Tag::Bool {
            // SAFETY: tag is Bool; u8 non-zero → true, zero → false.
            Some(unsafe { self.data.bool_val != 0 })
        } else {
            None
        }
    }

    /// The active tag for this value.
    pub fn tag(&self) -> Tag {
        self.tag
    }

    /// Human-readable description — mirrors the OCaml `describe` function.
    pub fn describe(&self) -> String {
        match self.tag {
            Tag::Int => format!("Int({})", unsafe { self.data.int_val }),
            Tag::Float => format!("Float({})", unsafe { self.data.float_val }),
            Tag::Bool => format!("Bool({})", unsafe { self.data.bool_val != 0 }),
        }
    }

    /// Size in bytes of the stored value — mirrors OCaml `size_of_value`.
    pub fn size_of_stored(&self) -> usize {
        match self.tag {
            Tag::Int => 8,
            Tag::Float => 8,
            Tag::Bool => 1,
        }
    }
}

// ---------------------------------------------------------------------------
// Idiomatic Rust equivalent: just use an enum.
// In most Rust code you would never touch a raw union directly.
// ---------------------------------------------------------------------------

/// Idiomatic Rust: the compiler generates the tag and dispatch for you.
#[derive(Debug, Clone, PartialEq)]
pub enum ValueEnum {
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl ValueEnum {
    pub fn describe(&self) -> String {
        match self {
            ValueEnum::Int(n) => format!("Int({n})"),
            ValueEnum::Float(f) => format!("Float({f})"),
            ValueEnum::Bool(b) => format!("Bool({b})"),
        }
    }

    pub fn size_of_stored(&self) -> usize {
        match self {
            ValueEnum::Int(_) => 8,
            ValueEnum::Float(_) => 8,
            ValueEnum::Bool(_) => 1,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tagged-union (manual) tests ---

    #[test]
    fn test_int_value_round_trip() {
        let v = Value::int(42);
        assert_eq!(v.tag(), Tag::Int);
        assert_eq!(v.as_int(), Some(42));
        assert_eq!(v.as_float(), None);
        assert_eq!(v.as_bool(), None);
    }

    #[test]
    fn test_float_value_round_trip() {
        let v = Value::float(3.14);
        assert_eq!(v.tag(), Tag::Float);
        assert!(v.as_float().is_some());
        assert!((v.as_float().unwrap() - 3.14).abs() < f64::EPSILON);
        assert_eq!(v.as_int(), None);
        assert_eq!(v.as_bool(), None);
    }

    #[test]
    fn test_bool_value_round_trip() {
        let t = Value::bool(true);
        assert_eq!(t.tag(), Tag::Bool);
        assert_eq!(t.as_bool(), Some(true));

        let f = Value::bool(false);
        assert_eq!(f.as_bool(), Some(false));
        assert_eq!(f.as_int(), None);
    }

    #[test]
    fn test_negative_int() {
        let v = Value::int(-7);
        assert_eq!(v.as_int(), Some(-7));
        assert_eq!(v.describe(), "Int(-7)");
    }

    #[test]
    fn test_describe_and_size() {
        let vals = [Value::int(42), Value::float(3.14), Value::bool(true)];
        let descriptions: Vec<String> = vals.iter().map(|v| v.describe()).collect();
        assert_eq!(descriptions[0], "Int(42)");
        assert!(descriptions[1].starts_with("Float("));
        assert_eq!(descriptions[2], "Bool(true)");

        assert_eq!(vals[0].size_of_stored(), 8);
        assert_eq!(vals[1].size_of_stored(), 8);
        assert_eq!(vals[2].size_of_stored(), 1);
    }

    #[test]
    fn test_cross_field_isolation() {
        // Writing int then reading float must return None (tag guard prevents it).
        let v = Value::int(100);
        assert_eq!(v.as_float(), None);
        assert_eq!(v.as_bool(), None);
    }

    // --- Idiomatic enum tests ---

    #[test]
    fn test_enum_describe() {
        assert_eq!(ValueEnum::Int(42).describe(), "Int(42)");
        assert_eq!(ValueEnum::Bool(false).describe(), "Bool(false)");
    }

    #[test]
    fn test_enum_size_of_stored() {
        assert_eq!(ValueEnum::Int(0).size_of_stored(), 8);
        assert_eq!(ValueEnum::Float(0.0).size_of_stored(), 8);
        assert_eq!(ValueEnum::Bool(true).size_of_stored(), 1);
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(ValueEnum::Int(1), ValueEnum::Int(1));
        assert_ne!(ValueEnum::Int(1), ValueEnum::Int(2));
    }
}
