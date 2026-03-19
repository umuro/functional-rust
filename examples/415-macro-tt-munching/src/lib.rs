//! Token Tree Munching
//!
//! Parsing complex syntax by consuming token trees one at a time.

/// Parse a struct definition DSL with defaults.
#[macro_export]
macro_rules! define_config {
    // Done: emit the struct
    (@fields $name:ident {} -> { $($fields:tt)* } defaults: { $($defaults:tt)* }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($fields)*
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    $($defaults)*
                }
            }
        }
    };

    // Munch one field: name: Type = default,
    (@fields $name:ident {
        $field:ident : $ty:ty = $default:expr,
        $($rest:tt)*
    } -> { $($fields:tt)* } defaults: { $($defaults:tt)* }) => {
        define_config!(@fields $name { $($rest)* } -> {
            $($fields)*
            pub $field: $ty,
        } defaults: {
            $($defaults)*
            $field: $default,
        });
    };

    // Entry point
    (struct $name:ident { $($body:tt)* }) => {
        define_config!(@fields $name { $($body)* } -> {} defaults: {});
    };
}

/// Simple calculator DSL.
#[macro_export]
macro_rules! calc {
    // Base: single literal
    ($n:literal) => { $n };

    // Addition
    ($a:literal + $($rest:tt)+) => {
        $a + calc!($($rest)+)
    };

    // Subtraction
    ($a:literal - $($rest:tt)+) => {
        $a - calc!($($rest)+)
    };

    // Multiplication (simple case)
    ($a:literal * $b:literal) => {
        $a * $b
    };
}

/// Parse key=value pairs into a HashMap.
#[macro_export]
macro_rules! parse_pairs {
    // Done
    (@acc $map:ident) => {};

    // Munch one pair
    (@acc $map:ident $key:ident = $val:expr; $($rest:tt)*) => {
        $map.insert(stringify!($key).to_string(), $val.to_string());
        parse_pairs!(@acc $map $($rest)*);
    };

    // Entry
    ($($key:ident = $val:expr;)*) => {{
        let mut map = ::std::collections::HashMap::new();
        parse_pairs!(@acc map $($key = $val;)*);
        map
    }};
}

/// Process command-like DSL.
#[macro_export]
macro_rules! commands {
    // Done
    (@acc $results:ident) => {};

    // set command
    (@acc $results:ident set $var:ident = $val:expr; $($rest:tt)*) => {
        $results.push(format!("SET {} = {}", stringify!($var), $val));
        commands!(@acc $results $($rest)*);
    };

    // get command
    (@acc $results:ident get $var:ident; $($rest:tt)*) => {
        $results.push(format!("GET {}", stringify!($var)));
        commands!(@acc $results $($rest)*);
    };

    // delete command
    (@acc $results:ident delete $var:ident; $($rest:tt)*) => {
        $results.push(format!("DELETE {}", stringify!($var)));
        commands!(@acc $results $($rest)*);
    };

    // Entry
    ($($cmd:tt)*) => {{
        let mut results = Vec::new();
        commands!(@acc results $($cmd)*);
        results
    }};
}

/// Count specific tokens.
#[macro_export]
macro_rules! count_tokens {
    // Done
    (@counting $count:expr,) => { $count };

    // Skip commas, count everything else
    (@counting $count:expr, , $($rest:tt)*) => {
        count_tokens!(@counting $count, $($rest)*)
    };

    (@counting $count:expr, $head:tt $($rest:tt)*) => {
        count_tokens!(@counting $count + 1, $($rest)*)
    };

    // Entry
    ($($tokens:tt)*) => {
        count_tokens!(@counting 0usize, $($tokens)*)
    };
}

define_config!(
    struct ServerConfig {
    host: String = "localhost".to_string(),
    port: u16 = 8080,
    timeout_secs: u32 = 30,
}
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_config() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.host, "localhost");
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.timeout_secs, 30);
    }

    #[test]
    fn test_define_config_override() {
        let cfg = ServerConfig {
            port: 9090,
            ..Default::default()
        };
        assert_eq!(cfg.port, 9090);
        assert_eq!(cfg.host, "localhost");
    }

    #[test]
    fn test_calc_add() {
        assert_eq!(calc!(2 + 3), 5);
    }

    #[test]
    fn test_calc_sub() {
        assert_eq!(calc!(10 - 4), 6);
    }

    #[test]
    fn test_calc_mul() {
        assert_eq!(calc!(3 * 4), 12);
    }

    #[test]
    fn test_parse_pairs() {
        let pairs = parse_pairs! {
            name = "Alice";
            age = 30;
            city = "NYC";
        };
        assert_eq!(pairs["name"], "Alice");
        assert_eq!(pairs["age"], "30");
        assert_eq!(pairs["city"], "NYC");
    }

    #[test]
    fn test_commands() {
        let cmds = commands! {
            set x = 10;
            get y;
            delete z;
        };
        assert_eq!(cmds.len(), 3);
        assert!(cmds[0].contains("SET x"));
        assert!(cmds[1].contains("GET y"));
        assert!(cmds[2].contains("DELETE z"));
    }

    #[test]
    fn test_count_tokens() {
        assert_eq!(count_tokens!(a b c), 3);
        assert_eq!(count_tokens!(1 + 2 + 3), 5);
    }
}
