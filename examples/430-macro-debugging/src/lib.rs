//! Macro Debugging
//!
//! Tools for debugging macros.

/// Use cargo expand to see macro expansion.
/// Use trace_macros! for step-by-step.

#[macro_export]
macro_rules! debug_sum {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        eprintln!("debug_sum: {} + {} = {}", a, b, a + b);
        a + b
    }};
}

/// Macro that shows its expansion.
#[macro_export]
macro_rules! show_expansion {
    ($($t:tt)*) => {
        compile_error!(concat!("Tokens: ", stringify!($($t)*)));
    };
}

/// Helper to stringify macro args.
#[macro_export]
macro_rules! stringify_args {
    ($($arg:expr),*) => {
        vec![$(stringify!($arg)),*]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug_sum() {
        let result = debug_sum!(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_stringify_args() {
        let args = stringify_args!(x, y + z, foo());
        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "x");
    }

    #[test]
    fn test_nested_debug() {
        let result = debug_sum!(debug_sum!(1, 2), 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_stringify_preserves() {
        let args = stringify_args!(1 + 2, 3 * 4);
        assert!(args[0].contains("+"));
        assert!(args[1].contains("*"));
    }

    #[test]
    fn test_empty_stringify() {
        let args: Vec<&str> = stringify_args!();
        assert!(args.is_empty());
    }
}
