#![allow(clippy::all)]
//! Macro Scoping
//!
//! How macros are imported and exported.

/// #[macro_export] makes macro public.
/// #[macro_use] imports macros from crate.

#[macro_export]
macro_rules! public_macro {
    () => {
        "public"
    };
}

/// Not exported - only usable in this crate.
macro_rules! private_macro {
    () => {
        "private"
    };
}

/// Use private macro.
pub fn use_private() -> &'static str {
    private_macro!()
}

/// Use public macro.
pub fn use_public() -> &'static str {
    public_macro!()
}

/// Module with local macro.
mod inner {
    macro_rules! local_macro {
        () => {
            "local"
        };
    }

    pub fn use_local() -> &'static str {
        local_macro!()
    }
}

pub use inner::use_local;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_macro() {
        assert_eq!(public_macro!(), "public");
    }

    #[test]
    fn test_use_private() {
        assert_eq!(use_private(), "private");
    }

    #[test]
    fn test_use_public() {
        assert_eq!(use_public(), "public");
    }

    #[test]
    fn test_use_local() {
        assert_eq!(use_local(), "local");
    }

    #[test]
    fn test_exported_available() {
        let s = public_macro!();
        assert!(!s.is_empty());
    }
}
