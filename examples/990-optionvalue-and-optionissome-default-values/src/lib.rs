/// Return the value inside an Option, or a default if None.
/// Mirrors OCaml's `Option.value ~default:x opt`.
pub fn option_value<T>(opt: Option<T>, default: T) -> T {
    opt.unwrap_or(default)
}

/// Return true if the Option holds a value.
/// Mirrors OCaml's `Option.is_some opt`.
pub fn option_is_some<T>(opt: &Option<T>) -> bool {
    opt.is_some()
}

/// Functional-style: extract with a lazy default (closure).
/// Mirrors the pattern of `Option.value` but avoids eager evaluation.
pub fn option_value_lazy<T, F: FnOnce() -> T>(opt: Option<T>, default_fn: F) -> T {
    opt.unwrap_or_else(default_fn)
}

/// Map an Option to a concrete value, falling back to a default.
/// Demonstrates `.map().unwrap_or()` — common idiomatic pattern.
pub fn describe_port(port: Option<u16>) -> String {
    port.map(|p| format!("port {p}"))
        .unwrap_or_else(|| "default port".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // option_value tests

    #[test]
    fn test_option_value_none_returns_default() {
        assert_eq!(option_value(None::<i32>, 42), 42);
    }

    #[test]
    fn test_option_value_some_returns_inner() {
        assert_eq!(option_value(Some(99), 42), 99);
    }

    #[test]
    fn test_option_value_string_default() {
        assert_eq!(option_value(None::<&str>, "0.0.0.0"), "0.0.0.0");
    }

    #[test]
    fn test_option_value_string_some() {
        assert_eq!(option_value(Some("localhost"), "0.0.0.0"), "localhost");
    }

    // option_is_some tests

    #[test]
    fn test_option_is_some_none() {
        assert!(!option_is_some(&None::<i32>));
    }

    #[test]
    fn test_option_is_some_some() {
        assert!(option_is_some(&Some(1)));
    }

    // option_value_lazy tests

    #[test]
    fn test_option_value_lazy_none_calls_closure() {
        let result = option_value_lazy(None::<u16>, || 8080);
        assert_eq!(result, 8080);
    }

    #[test]
    fn test_option_value_lazy_some_skips_closure() {
        let mut called = false;
        let result = option_value_lazy(Some(3000_u16), || {
            called = true;
            8080
        });
        assert_eq!(result, 3000);
        assert!(!called);
    }

    // describe_port tests

    #[test]
    fn test_describe_port_none() {
        assert_eq!(describe_port(None), "default port");
    }

    #[test]
    fn test_describe_port_some() {
        assert_eq!(describe_port(Some(8080)), "port 8080");
    }

    // Config scenario: mirrors the OCaml example directly

    #[test]
    fn test_config_scenario() {
        let config_port: Option<u16> = None;
        let config_host: Option<&str> = Some("localhost");

        let port = option_value(config_port, 8080);
        let host = option_value(config_host, "0.0.0.0");

        assert_eq!(port, 8080);
        assert_eq!(host, "localhost");
        assert!(!option_is_some(&config_port));
        assert!(option_is_some(&config_host));
    }
}
