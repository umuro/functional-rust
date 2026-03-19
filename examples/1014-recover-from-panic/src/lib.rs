// 1014: Recover from Panic
// std::panic::catch_unwind for recovering from panics

use std::panic;

// Approach 1: catch_unwind — converts panic to Result
fn safe_divide(a: i64, b: i64) -> Result<i64, String> {
    let result = panic::catch_unwind(|| {
        if b == 0 {
            panic!("division by zero");
        }
        a / b
    });

    result.map_err(|e| {
        if let Some(s) = e.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = e.downcast_ref::<String>() {
            s.clone()
        } else {
            "unknown panic".into()
        }
    })
}

// Approach 2: catch_unwind with AssertUnwindSafe
fn catch_with_state(data: &mut Vec<i64>) -> Result<i64, String> {
    // AssertUnwindSafe is needed for mutable references
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        data.push(42);
        if data.len() > 5 {
            panic!("too many elements");
        }
        data.iter().sum()
    }));

    result.map_err(|e| {
        e.downcast_ref::<&str>()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".into())
    })
}

// Approach 3: set_hook for custom panic handling
fn with_quiet_panic<F, R>(f: F) -> Result<R, String>
where
    F: FnOnce() -> R + panic::UnwindSafe,
{
    // Suppress default panic output
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // silent

    let result = panic::catch_unwind(f);

    panic::set_hook(prev_hook); // restore

    result.map_err(|e| {
        e.downcast_ref::<&str>()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown panic".into())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_success() {
        assert_eq!(safe_divide(10, 2), Ok(5));
    }

    #[test]
    fn test_catch_panic() {
        let result = safe_divide(10, 0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("division by zero"));
    }

    #[test]
    fn test_catch_with_state() {
        let mut data = vec![1, 2, 3];
        let result = catch_with_state(&mut data);
        assert!(result.is_ok());
        assert_eq!(data.len(), 4); // 42 was pushed
    }

    #[test]
    fn test_catch_state_overflow() {
        let mut data = vec![1, 2, 3, 4, 5];
        let result = catch_with_state(&mut data);
        assert!(result.is_err());
    }

    #[test]
    fn test_quiet_panic() {
        let result = with_quiet_panic(|| {
            panic!("silent failure");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_quiet_success() {
        let result = with_quiet_panic(|| 42);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_catch_unwind_basics() {
        // catch_unwind returns Result<T, Box<dyn Any>>
        let ok = std::panic::catch_unwind(|| 42);
        assert_eq!(ok.unwrap(), 42);

        let err = std::panic::catch_unwind(|| -> i64 { panic!("boom") });
        assert!(err.is_err());
    }
}
