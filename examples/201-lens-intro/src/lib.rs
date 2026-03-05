// Example 201: The Nested Update Problem — Why Lenses Exist

// === The Problem: Deeply Nested Struct Updates === //

#[derive(Debug, Clone, PartialEq)]
struct DbConfig {
    host: String,
    port: u16,
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ServerConfig {
    db: DbConfig,
    max_connections: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct AppConfig {
    server: ServerConfig,
    debug: bool,
    version: String,
}

// Approach 1: Manual nested update — clone everything by hand
fn update_db_port_manual(config: &AppConfig, new_port: u16) -> AppConfig {
    AppConfig {
        server: ServerConfig {
            db: DbConfig {
                port: new_port,
                ..config.server.db.clone()
            },
            ..config.server.clone()
        },
        ..config.clone()
    }
}

// Approach 2: Helper functions — map at each level
fn map_server(f: impl FnOnce(ServerConfig) -> ServerConfig, config: &AppConfig) -> AppConfig {
    AppConfig {
        server: f(config.server.clone()),
        ..config.clone()
    }
}

fn map_db(f: impl FnOnce(DbConfig) -> DbConfig, server: ServerConfig) -> ServerConfig {
    ServerConfig {
        db: f(server.db.clone()),
        ..server
    }
}

fn set_port(port: u16, db: DbConfig) -> DbConfig {
    DbConfig { port, ..db }
}

fn update_db_port_helpers(config: &AppConfig, new_port: u16) -> AppConfig {
    map_server(|s| map_db(|d| set_port(new_port, d), s), config)
}

// Approach 3: Lenses — composable getters and setters
struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    fn new(
        get: impl Fn(&S) -> A + 'static,
        set: impl Fn(A, &S) -> S + 'static,
    ) -> Self {
        Lens {
            get: Box::new(get),
            set: Box::new(set),
        }
    }

    fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
        S: Clone,
    {
        let outer_get = self.get;
        let outer_set = self.set;
        let inner_get = inner.get;
        let inner_set = inner.set;
        Lens {
            get: Box::new(move |s| (inner_get)(&(outer_get)(s))),
            set: Box::new(move |b, s| {
                let a = (outer_get)(s);
                let new_a = (inner_set)(b, &a);
                (outer_set)(new_a, s)
            }),
        }
    }
}

fn server_lens() -> Lens<AppConfig, ServerConfig> {
    Lens::new(
        |c| c.server.clone(),
        |s, c| AppConfig { server: s, ..c.clone() },
    )
}

fn db_lens() -> Lens<ServerConfig, DbConfig> {
    Lens::new(
        |s| s.db.clone(),
        |d, s| ServerConfig { db: d, ..s.clone() },
    )
}

fn port_lens() -> Lens<DbConfig, u16> {
    Lens::new(
        |d| d.port,
        |p, d| DbConfig { port: p, ..d.clone() },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> AppConfig {
        AppConfig {
            server: ServerConfig {
                db: DbConfig {
                    host: "localhost".into(),
                    port: 5432,
                    name: "mydb".into(),
                },
                max_connections: 100,
            },
            debug: false,
            version: "1.0".into(),
        }
    }

    #[test]
    fn test_manual_update() {
        let c = update_db_port_manual(&sample_config(), 5433);
        assert_eq!(c.server.db.port, 5433);
        assert_eq!(c.server.max_connections, 100);
    }

    #[test]
    fn test_helper_update() {
        let c = update_db_port_helpers(&sample_config(), 5433);
        assert_eq!(c.server.db.port, 5433);
    }

    #[test]
    fn test_lens_update() {
        let lens = server_lens().compose(db_lens()).compose(port_lens());
        let c = (lens.set)(5433, &sample_config());
        assert_eq!((lens.get)(&c), 5433);
        assert_eq!(c.server.max_connections, 100);
    }

    #[test]
    fn test_all_equivalent() {
        let cfg = sample_config();
        let c1 = update_db_port_manual(&cfg, 9999);
        let c2 = update_db_port_helpers(&cfg, 9999);
        let lens = server_lens().compose(db_lens()).compose(port_lens());
        let c3 = (lens.set)(9999, &cfg);
        assert_eq!(c1, c2);
        assert_eq!(c2, c3);
    }
}
