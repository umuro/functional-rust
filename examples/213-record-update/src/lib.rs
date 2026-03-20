#![allow(clippy::all)]
//! # Example 213: Practical Lens — Deeply Nested Config Update
//!
//! Lenses solve a real problem: immutably updating one field deep inside a
//! nested struct without writing boilerplate at every level.
//!
//! A `Lens<S, A>` is a pair of `(get: S → A, set: A × S → S)`.
//! Composing two lenses gives a new lens that skips the intermediate level.
//! `over(lens, f, config)` applies `f` to the focused field and rebuilds
//! every ancestor — all the clone-and-update work disappears into the lens.

use std::rc::Rc;

// ============================================================================
// Lens type — simple get/set pair
// ============================================================================

type GetFn<S, A> = Rc<dyn Fn(&S) -> A>;
type SetFn<S, A> = Rc<dyn Fn(A, &S) -> S>;

/// A lens focusing on a field of type `A` inside a structure of type `S`.
///
/// Both closures are reference-counted so a single lens can be composed into
/// multiple derived lenses without needing to copy the underlying function.
pub struct Lens<S, A> {
    get: GetFn<S, A>,
    set: SetFn<S, A>,
}

impl<S: 'static, A: 'static> Lens<S, A> {
    /// Build a lens from a getter and a setter.
    pub fn new(get: impl Fn(&S) -> A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            get: Rc::new(get),
            set: Rc::new(set),
        }
    }

    /// Read the focused value out of `s`.
    pub fn view(&self, s: &S) -> A {
        (self.get)(s)
    }

    /// Replace the focused value with `a`.
    pub fn set(&self, a: A, s: &S) -> S {
        (self.set)(a, s)
    }

    /// Apply a function to the focused value and return the updated structure.
    ///
    /// `over(lens, f, s) = lens.set(f(lens.get(s)), s)`
    ///
    /// OCaml: `let over l f s = l.set (f (l.get s)) s`
    pub fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        let a = (self.get)(s);
        (self.set)(f(a), s)
    }

    /// Compose `self` (focusing `S → A`) with `inner` (focusing `A → B`).
    ///
    /// The resulting lens focuses `S → B`.
    ///
    /// OCaml:
    /// ```ocaml
    /// let compose outer inner = {
    ///   get = (fun s -> inner.get (outer.get s));
    ///   set = (fun b s -> outer.set (inner.set b (outer.get s)) s);
    /// }
    /// ```
    ///
    /// `Rc` allows `outer_get` to be shared between the `get` and `set`
    /// closures of the composed lens without cloning the closure itself.
    pub fn compose<B: 'static>(self, inner: Lens<A, B>) -> Lens<S, B>
    where
        A: Clone,
    {
        // Share outer_get between the two composed closures via Rc.
        let outer_get = Rc::clone(&self.get);
        let outer_get2 = Rc::clone(&self.get);
        let outer_set = Rc::clone(&self.set);
        let inner_get = Rc::clone(&inner.get);
        let inner_set = Rc::clone(&inner.set);

        Lens {
            get: Rc::new(move |s| inner_get(&outer_get(s))),
            set: Rc::new(move |b, s| {
                let a = outer_get2(s);
                let new_a = inner_set(b, &a);
                outer_set(new_a, s)
            }),
        }
    }
}

// ============================================================================
// Config domain — realistic 4-level nesting
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct SslConfig {
    pub enabled: bool,
    pub cert_path: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolConfig {
    pub min_size: u32,
    pub max_size: u32,
    pub timeout_ms: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub pool: PoolConfig,
    pub ssl: SslConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheConfig {
    pub host: String,
    pub port: u16,
    pub ttl_seconds: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub db: DbConfig,
    pub cache: CacheConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppConfig {
    pub name: String,
    pub debug: bool,
    pub server: ServerConfig,
}

// ============================================================================
// Atomic lenses — one per struct field
// ============================================================================

pub fn app_server() -> Lens<AppConfig, ServerConfig> {
    Lens::new(
        |a: &AppConfig| a.server.clone(),
        |server, a| AppConfig {
            server,
            ..a.clone()
        },
    )
}

pub fn app_debug() -> Lens<AppConfig, bool> {
    Lens::new(
        |a: &AppConfig| a.debug,
        |debug, a| AppConfig { debug, ..a.clone() },
    )
}

pub fn server_db() -> Lens<ServerConfig, DbConfig> {
    Lens::new(
        |s: &ServerConfig| s.db.clone(),
        |db, s| ServerConfig { db, ..s.clone() },
    )
}

pub fn server_cache() -> Lens<ServerConfig, CacheConfig> {
    Lens::new(
        |s: &ServerConfig| s.cache.clone(),
        |cache, s| ServerConfig { cache, ..s.clone() },
    )
}

pub fn db_pool() -> Lens<DbConfig, PoolConfig> {
    Lens::new(
        |d: &DbConfig| d.pool.clone(),
        |pool, d| DbConfig { pool, ..d.clone() },
    )
}

pub fn db_ssl() -> Lens<DbConfig, SslConfig> {
    Lens::new(
        |d: &DbConfig| d.ssl.clone(),
        |ssl, d| DbConfig { ssl, ..d.clone() },
    )
}

pub fn pool_max_size() -> Lens<PoolConfig, u32> {
    Lens::new(
        |p: &PoolConfig| p.max_size,
        |max_size, p| PoolConfig {
            max_size,
            ..p.clone()
        },
    )
}

pub fn pool_min_size() -> Lens<PoolConfig, u32> {
    Lens::new(
        |p: &PoolConfig| p.min_size,
        |min_size, p| PoolConfig {
            min_size,
            ..p.clone()
        },
    )
}

pub fn pool_timeout_ms() -> Lens<PoolConfig, u64> {
    Lens::new(
        |p: &PoolConfig| p.timeout_ms,
        |timeout_ms, p| PoolConfig {
            timeout_ms,
            ..p.clone()
        },
    )
}

pub fn ssl_enabled() -> Lens<SslConfig, bool> {
    Lens::new(
        |s: &SslConfig| s.enabled,
        |enabled, s| SslConfig {
            enabled,
            ..s.clone()
        },
    )
}

pub fn cache_ttl() -> Lens<CacheConfig, u64> {
    Lens::new(
        |c: &CacheConfig| c.ttl_seconds,
        |ttl_seconds, c| CacheConfig {
            ttl_seconds,
            ..c.clone()
        },
    )
}

// ============================================================================
// Composed lenses — App → deeply nested fields
// ============================================================================

/// App → u32 (pool max_size — 4 levels deep)
pub fn app_pool_max_size() -> Lens<AppConfig, u32> {
    app_server()
        .compose(server_db())
        .compose(db_pool())
        .compose(pool_max_size())
}

/// App → u32 (pool min_size)
pub fn app_pool_min_size() -> Lens<AppConfig, u32> {
    app_server()
        .compose(server_db())
        .compose(db_pool())
        .compose(pool_min_size())
}

/// App → u64 (pool timeout)
pub fn app_pool_timeout() -> Lens<AppConfig, u64> {
    app_server()
        .compose(server_db())
        .compose(db_pool())
        .compose(pool_timeout_ms())
}

/// App → bool (ssl enabled)
pub fn app_ssl_enabled() -> Lens<AppConfig, bool> {
    app_server()
        .compose(server_db())
        .compose(db_ssl())
        .compose(ssl_enabled())
}

/// App → u64 (cache ttl)
pub fn app_cache_ttl() -> Lens<AppConfig, u64> {
    app_server().compose(server_cache()).compose(cache_ttl())
}

// ============================================================================
// Production configurator — the motivating use case
// ============================================================================

/// Apply all production settings in one pass.
///
/// Without lenses this would require rebuilding every ancestor struct for each
/// of the six changed fields — four levels × six changes = ~24 lines of
/// boilerplate. With lenses each change is one call; composition handles
/// the rebuilding automatically.
///
/// OCaml parallel: `let configure_for_production cfg = ...`
/// using `( %~ )` operator chaining.
pub fn configure_for_production(config: &AppConfig) -> AppConfig {
    let c = app_pool_max_size().over(|n| n * 2, config);
    let c = app_pool_min_size().over(|n| n * 2, &c);
    let c = app_pool_timeout().set(30_000, &c);
    let c = app_ssl_enabled().set(true, &c);
    let c = app_cache_ttl().set(300, &c);
    app_debug().set(false, &c)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn dev_config() -> AppConfig {
        AppConfig {
            name: "myapp".into(),
            debug: true,
            server: ServerConfig {
                host: "localhost".into(),
                port: 8080,
                db: DbConfig {
                    host: "localhost".into(),
                    port: 5432,
                    pool: PoolConfig {
                        min_size: 2,
                        max_size: 10,
                        timeout_ms: 5_000,
                    },
                    ssl: SslConfig {
                        enabled: false,
                        cert_path: "".into(),
                    },
                },
                cache: CacheConfig {
                    host: "localhost".into(),
                    port: 6379,
                    ttl_seconds: 60,
                },
            },
        }
    }

    #[test]
    fn test_view_top_level_field() {
        assert!(app_debug().view(&dev_config()));
    }

    #[test]
    fn test_view_4_levels_deep() {
        assert_eq!(app_pool_max_size().view(&dev_config()), 10);
    }

    #[test]
    fn test_set_top_level_does_not_disturb_siblings() {
        let cfg = dev_config();
        let updated = app_debug().set(false, &cfg);
        assert!(!updated.debug);
        assert_eq!(updated.name, "myapp");
        assert_eq!(updated.server.db.pool.max_size, 10);
    }

    #[test]
    fn test_set_deeply_nested_pool_max_size() {
        let cfg = dev_config();
        let updated = app_pool_max_size().set(50, &cfg);
        assert_eq!(updated.server.db.pool.max_size, 50);
        assert_eq!(updated.server.db.pool.min_size, 2);
        assert_eq!(updated.server.db.pool.timeout_ms, 5_000);
        assert_eq!(updated.server.db.host, "localhost");
        assert_eq!(updated.name, "myapp");
    }

    #[test]
    fn test_over_doubles_pool_max() {
        let cfg = dev_config();
        let updated = app_pool_max_size().over(|n| n * 2, &cfg);
        assert_eq!(updated.server.db.pool.max_size, 20);
        assert_eq!(updated.server.db.pool.min_size, 2);
    }

    #[test]
    fn test_over_cache_ttl() {
        let cfg = dev_config();
        let updated = app_cache_ttl().over(|_| 300, &cfg);
        assert_eq!(updated.server.cache.ttl_seconds, 300);
        assert_eq!(updated.server.cache.host, "localhost");
        assert_eq!(updated.server.db.pool.max_size, 10);
    }

    #[test]
    fn test_original_not_mutated() {
        let cfg = dev_config();
        let _updated = app_pool_max_size().set(999, &cfg);
        assert_eq!(cfg.server.db.pool.max_size, 10);
    }

    #[test]
    fn test_configure_for_production() {
        let prod = configure_for_production(&dev_config());
        assert!(!prod.debug);
        assert!(prod.server.db.ssl.enabled);
        assert_eq!(prod.server.db.pool.max_size, 20);
        assert_eq!(prod.server.db.pool.min_size, 4);
        assert_eq!(prod.server.db.pool.timeout_ms, 30_000);
        assert_eq!(prod.server.cache.ttl_seconds, 300);
        assert_eq!(prod.name, "myapp");
        assert_eq!(prod.server.db.host, "localhost");
    }

    #[test]
    fn test_lens_law_get_after_set() {
        // get(set(a, s)) = a
        let cfg = dev_config();
        let updated = app_pool_max_size().set(42, &cfg);
        assert_eq!(app_pool_max_size().view(&updated), 42);
    }

    #[test]
    fn test_lens_law_set_after_get() {
        // set(get(s), s) = s
        let cfg = dev_config();
        let a = app_pool_max_size().view(&cfg);
        let roundtrip = app_pool_max_size().set(a, &cfg);
        assert_eq!(roundtrip, cfg);
    }

    #[test]
    fn test_lens_law_set_set() {
        // set(b, set(a, s)) = set(b, s)
        let cfg = dev_config();
        let after_two = app_pool_max_size().set(99, &app_pool_max_size().set(1, &cfg));
        let after_one = app_pool_max_size().set(99, &cfg);
        assert_eq!(after_two, after_one);
    }
}
