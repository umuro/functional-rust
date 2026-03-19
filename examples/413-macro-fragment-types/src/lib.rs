//! Macro Fragment Types
//!
//! Different designators: expr, ident, ty, pat, literal, block, tt.

/// Debug expression and return its value.
#[macro_export]
macro_rules! dbg_expr {
    ($e:expr) => {{
        let val = $e;
        println!("{} = {:?}", stringify!($e), val);
        val
    }};
}

/// Generate a getter method.
#[macro_export]
macro_rules! make_getter {
    ($field:ident : $ty:ty) => {
        pub fn $field(&self) -> &$ty {
            &self.$field
        }
    };
}

/// Generate a setter method.
#[macro_export]
macro_rules! make_setter {
    ($field:ident : $ty:ty) => {
        paste::item! {
            pub fn [<set_ $field>](&mut self, value: $ty) {
                self.$field = value;
            }
        }
    };
}

/// Generate a default function for a type.
#[macro_export]
macro_rules! make_default_fn {
    ($name:ident -> $ret:ty) => {
        pub fn $name() -> $ret {
            Default::default()
        }
    };
}

/// Check if value matches pattern.
#[macro_export]
macro_rules! matches_pattern {
    ($val:expr, $pat:pat) => {
        matches!($val, $pat)
    };
}

/// Repeat a literal string.
#[macro_export]
macro_rules! repeat_lit {
    ($s:literal, $n:literal) => {
        $s.repeat($n)
    };
}

/// Time a block and return result.
#[macro_export]
macro_rules! timed {
    ($label:literal, $block:block) => {{
        let start = ::std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        (result, elapsed)
    }};
}

/// Execute statements with prefix/suffix.
#[macro_export]
macro_rules! with_setup {
    (setup: $setup:block, body: $body:block, teardown: $teardown:block) => {{
        $setup
        let result = $body;
        $teardown
        result
    }};
}

/// Create an enum from identifiers.
#[macro_export]
macro_rules! make_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $name {
            $($variant,)*
        }
    };
}

make_enum!(Color { Red, Green, Blue });
make_enum!(Status {
    Active,
    Inactive,
    Pending
});

/// A demo struct to show getter generation.
pub struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    make_getter!(name: String);
    make_getter!(age: u32);
}

make_default_fn!(default_string -> String);
make_default_fn!(default_vec -> Vec<i32>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbg_expr() {
        let v = dbg_expr!(2 + 3);
        assert_eq!(v, 5);
    }

    #[test]
    fn test_make_getter() {
        let p = Person::new("Alice", 30);
        assert_eq!(p.name(), "Alice");
        assert_eq!(*p.age(), 30);
    }

    #[test]
    fn test_make_default_fn() {
        assert_eq!(default_string(), "");
        assert_eq!(default_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_matches_pattern_some() {
        let opt: Option<i32> = Some(42);
        assert!(matches_pattern!(opt, Some(_)));
        assert!(!matches_pattern!(opt, None));
    }

    #[test]
    fn test_matches_pattern_none() {
        let opt: Option<i32> = None;
        assert!(matches_pattern!(opt, None));
        assert!(!matches_pattern!(opt, Some(_)));
    }

    #[test]
    fn test_repeat_lit() {
        assert_eq!(repeat_lit!("ab", 3), "ababab");
        assert_eq!(repeat_lit!("x", 5), "xxxxx");
    }

    #[test]
    fn test_timed() {
        let (result, _elapsed) = timed!("sum", { (1..=100).sum::<i32>() });
        assert_eq!(result, 5050);
    }

    #[test]
    fn test_make_enum() {
        assert_eq!(Color::Red, Color::Red);
        assert_ne!(Color::Red, Color::Blue);
        assert_eq!(Status::Active, Status::Active);
    }

    #[test]
    fn test_with_setup() {
        let mut counter = 0;
        let result = with_setup!(
            setup: { counter += 1; },
            body: { counter * 10 },
            teardown: { counter += 1; }
        );
        assert_eq!(result, 10);
        assert_eq!(counter, 2);
    }
}
