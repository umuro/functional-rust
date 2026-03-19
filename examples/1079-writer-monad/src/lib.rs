#![allow(clippy::all)]
//! Writer Monad — Logging Computation
//!
//! The Writer monad accumulates a log alongside a computation.
//! In OCaml, this is a record `{ value: 'a; log: string list }`.
//! In Rust, we use a generic struct and implement monadic operations.

// ── Solution 1: Idiomatic Rust — struct with method chaining ──

/// A Writer that carries a value and a log of messages.
/// OCaml: `type 'a writer = { value: 'a; log: string list }`
#[derive(Debug, Clone, PartialEq)]
pub struct Writer<A> {
    pub value: A,
    pub log: Vec<String>,
}

impl<A> Writer<A> {
    /// Wrap a value with an empty log (monadic return/pure).
    /// OCaml: `let return x = { value = x; log = [] }`
    pub fn new(value: A) -> Self {
        Writer {
            value,
            log: Vec::new(),
        }
    }

    /// Monadic bind: apply a function to the value, combining logs.
    /// OCaml: `let bind w f = let w' = f w.value in { value = w'.value; log = w.log @ w'.log }`
    pub fn bind<B, F>(self, f: F) -> Writer<B>
    where
        F: FnOnce(A) -> Writer<B>,
    {
        let mut result = f(self.value);
        let mut combined_log = self.log;
        combined_log.append(&mut result.log);
        Writer {
            value: result.value,
            log: combined_log,
        }
    }

    /// Map a function over the value without adding to the log.
    /// This is the functor `fmap` operation.
    pub fn map<B, F>(self, f: F) -> Writer<B>
    where
        F: FnOnce(A) -> B,
    {
        Writer {
            value: f(self.value),
            log: self.log,
        }
    }
}

/// Add a message to the log without changing the value.
/// OCaml: `let tell msg = { value = (); log = [msg] }`
pub fn tell(msg: impl Into<String>) -> Writer<()> {
    Writer {
        value: (),
        log: vec![msg.into()],
    }
}

/// Half a number, logging the operation.
/// OCaml: `let half x = { value = x / 2; log = [Printf.sprintf "halved %d to %d" x (x / 2)] }`
pub fn half(x: i64) -> Writer<i64> {
    let result = x / 2;
    Writer {
        value: result,
        log: vec![format!("halved {x} to {result}")],
    }
}

/// The composed computation from the OCaml example.
/// OCaml: `let compute x = return x >>= fun n -> half n >>= fun n -> tell ... >>= fun () -> return n`
pub fn compute(x: i64) -> Writer<i64> {
    Writer::new(x)
        .bind(half)
        .bind(|n| tell(format!("result is {n}")).map(|()| n))
}

// ── Solution 2: Generic Writer with any monoid log ──
//
// OCaml's Writer uses `string list` but conceptually any monoid works.

/// A generic writer where the log type is any type that supports append.
#[derive(Debug, Clone, PartialEq)]
pub struct GenericWriter<W, A> {
    pub value: A,
    pub log: W,
}

/// Trait for monoid-like types (identity + combine).
pub trait Monoid: Default {
    fn combine(self, other: Self) -> Self;
}

impl Monoid for Vec<String> {
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

impl Monoid for String {
    fn combine(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

impl<W: Monoid, A> GenericWriter<W, A> {
    pub fn pure(value: A) -> Self {
        GenericWriter {
            value,
            log: W::default(),
        }
    }

    pub fn bind<B, F>(self, f: F) -> GenericWriter<W, B>
    where
        F: FnOnce(A) -> GenericWriter<W, B>,
    {
        let result = f(self.value);
        GenericWriter {
            value: result.value,
            log: self.log.combine(result.log),
        }
    }
}

// ── Solution 3: Functional composition with closures ──

/// A logged computation is just a function that returns a Writer.
/// We can compose them with `and_then`.
pub fn and_then<A, B>(first: Writer<A>, f: impl FnOnce(A) -> Writer<B>) -> Writer<B> {
    first.bind(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_has_empty_log() {
        let w: Writer<i64> = Writer::new(42);
        assert_eq!(w.value, 42);
        assert!(w.log.is_empty());
    }

    #[test]
    fn test_tell_adds_message() {
        let w = tell("hello");
        assert_eq!(w.value, ());
        assert_eq!(w.log, vec!["hello"]);
    }

    #[test]
    fn test_half_logs() {
        let w = half(100);
        assert_eq!(w.value, 50);
        assert_eq!(w.log, vec!["halved 100 to 50"]);
    }

    #[test]
    fn test_compute_full_pipeline() {
        let result = compute(100);
        assert_eq!(result.value, 50);
        assert_eq!(result.log, vec!["halved 100 to 50", "result is 50"]);
    }

    #[test]
    fn test_bind_combines_logs() {
        let w = Writer::new(10).bind(half).bind(half);
        assert_eq!(w.value, 2);
        assert_eq!(w.log, vec!["halved 10 to 5", "halved 5 to 2"]);
    }

    #[test]
    fn test_map_preserves_log() {
        let w = half(10).map(|n| n * 3);
        assert_eq!(w.value, 15);
        assert_eq!(w.log, vec!["halved 10 to 5"]);
    }

    #[test]
    fn test_generic_writer_string_monoid() {
        let w: GenericWriter<String, i32> = GenericWriter::pure(42);
        let result = w.bind(|n| GenericWriter {
            value: n + 1,
            log: format!("incremented {n}; "),
        });
        assert_eq!(result.value, 43);
        assert_eq!(result.log, "incremented 42; ");
    }

    #[test]
    fn test_and_then_composition() {
        let result = and_then(Writer::new(20), half);
        assert_eq!(result.value, 10);
        assert_eq!(result.log, vec!["halved 20 to 10"]);
    }
}
