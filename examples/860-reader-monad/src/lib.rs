#![allow(clippy::all)]
// Example 061: Reader Monad
// Dependency injection via implicit environment passing

// Approach 1: Reader as a wrapper struct
struct Reader<'a, R, A> {
    run: Box<dyn FnOnce(&R) -> A + 'a>,
}

impl<'a, R: 'a, A: 'a> Reader<'a, R, A> {
    fn new(f: impl FnOnce(&R) -> A + 'a) -> Self {
        Reader { run: Box::new(f) }
    }

    fn run(self, env: &R) -> A {
        (self.run)(env)
    }

    fn map<B: 'a>(self, f: impl FnOnce(A) -> B + 'a) -> Reader<'a, R, B> {
        Reader::new(move |env| f(self.run(env)))
    }

    fn and_then<B: 'a>(self, f: impl FnOnce(A) -> Reader<'a, R, B> + 'a) -> Reader<'a, R, B> {
        Reader::new(move |env: &R| {
            // We need to read env twice, so we use a pointer trick
            let env_ptr = env as *const R;
            let a = self.run(env);
            f(a).run(unsafe { &*env_ptr })
        })
    }
}

fn ask<'a, R: 'a + Clone>() -> Reader<'a, R, R> {
    Reader::new(|env: &R| env.clone())
}

fn asks<'a, R: 'a, A: 'a>(f: impl FnOnce(&R) -> A + 'a) -> Reader<'a, R, A> {
    Reader::new(f)
}

// Approach 2: Closures as readers (idiomatic Rust)
struct Config {
    db_host: String,
    db_port: u16,
    debug: bool,
}

fn get_connection_string(config: &Config) -> String {
    format!("{}:{}", config.db_host, config.db_port)
}

fn get_log_prefix(config: &Config) -> &str {
    if config.debug {
        "[DEBUG] "
    } else {
        "[INFO] "
    }
}

fn format_message(msg: &str, config: &Config) -> String {
    format!(
        "{}{} (connected to {})",
        get_log_prefix(config),
        msg,
        get_connection_string(config)
    )
}

// Approach 3: Trait-based dependency injection (most idiomatic Rust)
trait HasDb {
    fn db_url(&self) -> String;
}

trait HasLogger {
    fn log_prefix(&self) -> &str;
}

impl HasDb for Config {
    fn db_url(&self) -> String {
        format!("{}:{}", self.db_host, self.db_port)
    }
}

impl HasLogger for Config {
    fn log_prefix(&self) -> &str {
        if self.debug {
            "[DEBUG] "
        } else {
            "[INFO] "
        }
    }
}

fn format_msg_generic<E: HasDb + HasLogger>(msg: &str, env: &E) -> String {
    format!(
        "{}{} (connected to {})",
        env.log_prefix(),
        msg,
        env.db_url()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(debug: bool) -> Config {
        Config {
            db_host: "localhost".into(),
            db_port: 5432,
            debug,
        }
    }

    #[test]
    fn test_format_message_debug() {
        let cfg = test_config(true);
        assert_eq!(
            format_message("Starting", &cfg),
            "[DEBUG] Starting (connected to localhost:5432)"
        );
    }

    #[test]
    fn test_format_message_info() {
        let cfg = test_config(false);
        assert_eq!(
            format_message("Starting", &cfg),
            "[INFO] Starting (connected to localhost:5432)"
        );
    }

    #[test]
    fn test_trait_di() {
        let cfg = test_config(true);
        assert_eq!(
            format_msg_generic("Starting", &cfg),
            "[DEBUG] Starting (connected to localhost:5432)"
        );
    }

    #[test]
    fn test_reader_asks() {
        let cfg = test_config(true);
        let r = asks(|c: &Config| c.debug);
        assert_eq!(r.run(&cfg), true);
    }

    #[test]
    fn test_reader_map() {
        let cfg = test_config(false);
        let r = asks(|c: &Config| c.db_port).map(|p| p * 2);
        assert_eq!(r.run(&cfg), 10864);
    }
}
