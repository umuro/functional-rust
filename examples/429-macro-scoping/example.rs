// Macro scoping and #[macro_export] in Rust

// Macros follow textual scoping — must define before use in file

// Local macro (visible only in this module and below)
macro_rules! add {
    ($a:expr, $b:expr) => { $a + $b };
}

// Exported macro — available at crate root
#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $tolerance:expr) => {
        {
            let diff = ($a - $b).abs();
            assert!(diff < $tolerance,
                "assert_approx_eq failed: |{} - {}| = {} >= {}",
                $a, $b, diff, $tolerance);
        }
    };
    ($a:expr, $b:expr) => {
        assert_approx_eq!($a, $b, 1e-9f64);
    };
}

// Using $crate:: for correct cross-crate references
#[macro_export]
macro_rules! my_debug {
    ($val:expr) => {
        // $crate:: ensures we refer to this crate's items
        // even when macro is used from another crate
        println!("[{}:{}] {} = {:?}",
            file!(), line!(), stringify!($val), $val)
    };
}

mod inner {
    // Macro defined here is only visible within this module
    macro_rules! inner_add {
        ($a:expr, $b:expr) => { $a + $b };
    }

    pub fn compute(x: i32, y: i32) -> i32 {
        inner_add!(x, y) // valid: same module
    }

    // #[macro_export] would hoist to crate root:
    #[macro_export]
    macro_rules! inner_exported {
        ($x:expr) => { $x * 2 };
    }
}

// Using #[macro_use] for compatibility with older macro pattern
// mod compat {
//     #[macro_use]
//     mod macros {
//         macro_rules! old_style { ($x:expr) => { $x }; }
//     }
//     fn use_it() { let _ = old_style!(42); }
// }

fn main() {
    // Local macro
    println!("add!(3, 4) = {}", add!(3, 4));

    // Exported macros (available at crate root)
    assert_approx_eq!(1.0f64, 1.0 + 1e-12, 1e-9);
    println!("approx_eq test passed");

    my_debug!(42);
    my_debug!(vec![1, 2, 3]);
    my_debug!("hello");

    // Macro from inner module (exported)
    println!("inner_exported!(5) = {}", inner_exported!(5));

    // Function using inner macro
    println!("inner compute: {}", inner::compute(3, 7));
}

#[cfg(test)]
mod tests {
    // Exported macros are available here via #[macro_export]

    #[test]
    fn test_assert_approx_eq() {
        assert_approx_eq!(1.0f64, 1.0 + 1e-12);
        assert_approx_eq!(3.14f64, 3.14000001, 1e-5);
    }

    #[test]
    fn test_inner_exported() {
        assert_eq!(inner_exported!(6), 12);
    }
}
