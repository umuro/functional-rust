//! include! and include_str! Macros
//!
//! Including files at compile time.

/// Include raw bytes from a file.
pub const README_BYTES: &[u8] = include_bytes!("../Cargo.toml");

/// Example SQL query.
pub fn example_query() -> &'static str {
    "SELECT * FROM users"
}

/// Include at compile time.
#[macro_export]
macro_rules! include_sql {
    ($name:literal) => {
        concat!("-- Query: ", $name, "\n", "SELECT * FROM ", $name)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_include_bytes() {
        assert!(!README_BYTES.is_empty());
    }

    #[test]
    fn test_include_sql() {
        let sql = include_sql!("users");
        assert!(sql.contains("users"));
    }

    #[test]
    fn test_example_query() {
        assert!(example_query().contains("SELECT"));
    }

    #[test]
    fn test_cargo_toml_content() {
        let s = std::str::from_utf8(README_BYTES).unwrap();
        assert!(s.contains("[package]"));
    }

    #[test]
    fn test_file_macro() {
        let f = file!();
        assert!(f.contains("lib.rs"));
    }
}
