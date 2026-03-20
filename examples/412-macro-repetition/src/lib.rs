#![allow(clippy::all)]
//! Macro Repetition Patterns
//!
//! Using $(...),* and $(...),+ for variadic macros.

/// Sum any number of values.
#[macro_export]
macro_rules! sum {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => {
        $first $(+ $rest)*
    };
}

/// Product of any number of values.
#[macro_export]
macro_rules! product {
    () => { 1 };
    ($first:expr $(, $rest:expr)*) => {
        $first $(* $rest)*
    };
}

/// All values greater than threshold.
#[macro_export]
macro_rules! all_gt {
    ($threshold:expr; $($val:expr),+ $(,)?) => {
        true $(&& ($val > $threshold))+
    };
}

/// Any value equals target.
#[macro_export]
macro_rules! any_eq {
    ($target:expr; $($val:expr),+ $(,)?) => {
        false $(|| ($val == $target))+
    };
}

/// Create a HashMap from key-value pairs.
#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $value);)+
            map
        }
    };
}

/// Create a struct with fields from macro.
#[macro_export]
macro_rules! define_struct {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: $ty,)*
        }
    };
}

define_struct!(Config {
    host: String,
    port: u16,
    timeout: u32,
});

/// Print a table of key-value pairs.
#[macro_export]
macro_rules! print_table {
    ($($key:expr => $val:expr),* $(,)?) => {
        $(println!("{:>20}: {}", $key, $val);)*
    };
}

/// Count the number of arguments.
#[macro_export]
macro_rules! count_args {
    () => { 0usize };
    ($first:expr $(, $rest:expr)*) => {
        1usize $(+ { let _ = $rest; 1usize })*
    };
}

/// Create a Vec with a transformation applied.
#[macro_export]
macro_rules! vec_transform {
    ($f:expr; $($x:expr),* $(,)?) => {
        vec![$($f($x)),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum!(), 0);
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(sum!(42), 42);
    }

    #[test]
    fn test_sum_multiple() {
        assert_eq!(sum!(1, 2, 3, 4, 5), 15);
    }

    #[test]
    fn test_product_empty() {
        assert_eq!(product!(), 1);
    }

    #[test]
    fn test_product_multiple() {
        assert_eq!(product!(2, 3, 4), 24);
    }

    #[test]
    fn test_all_gt_true() {
        assert!(all_gt!(0; 1, 2, 3, 4, 5));
    }

    #[test]
    fn test_all_gt_false() {
        assert!(!all_gt!(2; 1, 2, 3, 4, 5));
    }

    #[test]
    fn test_any_eq_true() {
        assert!(any_eq!(3; 1, 2, 3, 4, 5));
    }

    #[test]
    fn test_any_eq_false() {
        assert!(!any_eq!(10; 1, 2, 3, 4, 5));
    }

    #[test]
    fn test_hashmap_empty() {
        let m: std::collections::HashMap<&str, i32> = hashmap!();
        assert!(m.is_empty());
    }

    #[test]
    fn test_hashmap_entries() {
        let m = hashmap! {
            "a" => 1,
            "b" => 2,
        };
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
    }

    #[test]
    fn test_define_struct() {
        let cfg = Config {
            host: "localhost".to_string(),
            port: 8080,
            timeout: 30,
        };
        assert_eq!(cfg.port, 8080);
    }

    #[test]
    fn test_count_args() {
        assert_eq!(count_args!(), 0);
        assert_eq!(count_args!(1), 1);
        assert_eq!(count_args!(1, 2, 3), 3);
        assert_eq!(count_args!(1, 2, 3, 4, 5), 5);
    }

    #[test]
    fn test_vec_transform() {
        let v = vec_transform!(|x| x * 2; 1, 2, 3);
        assert_eq!(v, vec![2, 4, 6]);
    }
}
