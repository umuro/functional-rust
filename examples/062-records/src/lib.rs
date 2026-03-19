#![allow(clippy::all)]
// 062: Records (Structs)
// Named fields, creation, update syntax, pattern matching

// Approach 1: Basic struct
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// Approach 2: Struct update syntax
#[derive(Debug, Clone)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
    timeout: u32,
}

impl Config {
    fn default_config() -> Self {
        Config {
            host: "localhost".to_string(),
            port: 8080,
            debug: false,
            timeout: 30,
        }
    }
}

fn dev_config() -> Config {
    Config {
        debug: true,
        port: 3000,
        ..Config::default_config()
    }
}

fn prod_config() -> Config {
    Config {
        host: "prod.example.com".to_string(),
        timeout: 60,
        ..Config::default_config()
    }
}

// Approach 3: Destructuring
fn describe_config(config: &Config) -> String {
    let Config {
        host, port, debug, ..
    } = config;
    format!("{}:{}{}", host, port, if *debug { " [DEBUG]" } else { "" })
}

fn is_local(config: &Config) -> bool {
    config.host == "localhost" || config.host == "127.0.0.1"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let o = Point::origin();
        let p = Point { x: 3.0, y: 4.0 };
        assert!((o.distance(&p) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_struct_update() {
        let dev = dev_config();
        assert!(dev.debug);
        assert_eq!(dev.port, 3000);
        assert_eq!(dev.host, "localhost");
    }

    #[test]
    fn test_prod_config() {
        let prod = prod_config();
        assert_eq!(prod.timeout, 60);
        assert_eq!(prod.host, "prod.example.com");
    }

    #[test]
    fn test_describe() {
        assert_eq!(describe_config(&dev_config()), "localhost:3000 [DEBUG]");
    }

    #[test]
    fn test_is_local() {
        assert!(is_local(&Config::default_config()));
        assert!(!is_local(&prod_config()));
    }
}
