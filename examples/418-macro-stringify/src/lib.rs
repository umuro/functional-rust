#![allow(clippy::all)]
//! stringify! and concat! Macros
//!
//! Converting tokens to strings at compile time.

/// Debug print with variable name.
#[macro_export]
macro_rules! dbg_named {
    ($val:expr) => {{
        let v = $val;
        println!("{} = {:?}", stringify!($val), v);
        v
    }};
}

/// Assert with expression stringification.
#[macro_export]
macro_rules! assert_dbg {
    ($cond:expr) => {
        if !$cond {
            panic!("Assertion failed: {}", stringify!($cond));
        }
    };
}

/// Create an enum with string conversion.
#[macro_export]
macro_rules! string_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)*
                }
            }
        }
    };
}

string_enum!(Color { Red, Green, Blue });
string_enum!(Status {
    Active,
    Inactive,
    Pending
});

/// Build a path-like string from segments.
#[macro_export]
macro_rules! path_str {
    ($($segment:expr),+ $(,)?) => {
        concat!($($segment, "/"),+).trim_end_matches('/')
    };
}

/// Get function name for logging.
#[macro_export]
macro_rules! fn_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

/// Log with automatic location info.
#[macro_export]
macro_rules! log_here {
    ($msg:expr) => {
        println!("[{}:{}] {}", file!(), line!(), $msg);
    };
}

/// Version string from Cargo.toml.
#[macro_export]
macro_rules! version_str {
    () => {
        concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify_basic() {
        let s = stringify!(x + y * z);
        assert_eq!(s, "x + y * z");
    }

    #[test]
    fn test_concat_basic() {
        let s = concat!("hello", " ", "world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_dbg_named() {
        let x = 42;
        let result = dbg_named!(x + 1);
        assert_eq!(result, 43);
    }

    #[test]
    fn test_assert_dbg_pass() {
        assert_dbg!(2 + 2 == 4);
    }

    #[test]
    #[should_panic(expected = "2 + 2 == 5")]
    fn test_assert_dbg_fail() {
        assert_dbg!(2 + 2 == 5);
    }

    #[test]
    fn test_string_enum() {
        assert_eq!(Color::Red.as_str(), "Red");
        assert_eq!(Color::Blue.as_str(), "Blue");
        assert_eq!(Status::Active.as_str(), "Active");
    }

    #[test]
    fn test_file_line() {
        let f = file!();
        let l = line!();
        assert!(f.contains("lib.rs"));
        assert!(l > 0);
    }
}
