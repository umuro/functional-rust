#![allow(clippy::all)]
//! syn and quote Basics
//!
//! Understanding the crates used in proc macros.

/// syn parses Rust tokens into AST.
/// quote generates Rust tokens from templates.
/// This example shows the concepts.

/// A field descriptor (what syn might parse).
pub struct FieldInfo {
    pub name: String,
    pub ty: String,
}

/// Generate code string (what quote does).
pub fn generate_getter(field: &FieldInfo) -> String {
    format!(
        "pub fn {}(&self) -> &{} {{ &self.{} }}",
        field.name, field.ty, field.name
    )
}

/// Generate setter.
pub fn generate_setter(field: &FieldInfo) -> String {
    format!(
        "pub fn set_{}(&mut self, value: {}) {{ self.{} = value; }}",
        field.name, field.ty, field.name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_getter() {
        let f = FieldInfo {
            name: "x".into(),
            ty: "i32".into(),
        };
        let code = generate_getter(&f);
        assert!(code.contains("fn x(&self)"));
        assert!(code.contains("&i32"));
    }

    #[test]
    fn test_generate_setter() {
        let f = FieldInfo {
            name: "y".into(),
            ty: "String".into(),
        };
        let code = generate_setter(&f);
        assert!(code.contains("set_y"));
        assert!(code.contains("value: String"));
    }

    #[test]
    fn test_field_info() {
        let f = FieldInfo {
            name: "age".into(),
            ty: "u32".into(),
        };
        assert_eq!(f.name, "age");
        assert_eq!(f.ty, "u32");
    }

    #[test]
    fn test_getter_contains_self() {
        let f = FieldInfo {
            name: "data".into(),
            ty: "Vec<u8>".into(),
        };
        let code = generate_getter(&f);
        assert!(code.contains("&self"));
    }

    #[test]
    fn test_setter_contains_mut() {
        let f = FieldInfo {
            name: "count".into(),
            ty: "usize".into(),
        };
        let code = generate_setter(&f);
        assert!(code.contains("&mut self"));
    }
}
