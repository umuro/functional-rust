//! Log Pattern Macros
//!
//! Logging and tracing utilities.

/// Log with level and location.
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        eprintln!(
            "[{}] {}:{}: {}",
            $level,
            file!(),
            line!(),
            format!($($arg)*)
        );
    };
}

/// Convenience macros.
#[macro_export]
macro_rules! info { ($($arg:tt)*) => { log!("INFO", $($arg)*); }; }

#[macro_export]
macro_rules! warn { ($($arg:tt)*) => { log!("WARN", $($arg)*); }; }

#[macro_export]
macro_rules! error { ($($arg:tt)*) => { log!("ERROR", $($arg)*); }; }

/// Trace function entry/exit.
#[macro_export]
macro_rules! trace_fn {
    ($name:expr, $body:block) => {{
        eprintln!("--> Entering {}", $name);
        let result = $body;
        eprintln!("<-- Exiting {}", $name);
        result
    }};
}

pub fn example_logged() -> i32 {
    // Would use: info!("Starting computation");
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_logged() {
        assert_eq!(example_logged(), 42);
    }

    #[test]
    fn test_trace_fn() {
        let result = trace_fn!("test", { 1 + 2 });
        assert_eq!(result, 3);
    }

    #[test]
    fn test_format_in_log() {
        // Just verify it compiles
        let x = 5;
        log!("DEBUG", "value = {}", x);
    }

    #[test]
    fn test_info_macro() {
        info!("test message");
    }

    #[test]
    fn test_nested_trace() {
        let result = trace_fn!("outer", { trace_fn!("inner", { 10 }) });
        assert_eq!(result, 10);
    }
}
