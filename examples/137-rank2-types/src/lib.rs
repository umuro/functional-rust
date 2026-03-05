//! # Rank-2 Types Simulation in Rust
//!
//! Rust lacks native rank-2 polymorphism, but traits with generic methods
//! serve as an equivalent: a trait bound forces the caller to supply something
//! that works for *every* type the trait method is called with.
//!
//! Crucially, traits with generic methods are **not** dyn-compatible in Rust
//! (they can't be used with `dyn Trait`). Rank-2 is therefore encoded via
//! *static dispatch* — `F: IdFn` — not dynamic dispatch. This is actually
//! the more faithful analogue to OCaml's record-polymorphism and first-class
//! modules.

// ---------------------------------------------------------------------------
// Approach 1: Trait-based rank-2 identity
// ---------------------------------------------------------------------------
// `trait IdFn` is the "for all T" contract. Any implementor must handle every T.
// `apply_id` can therefore call `f.apply` with an i32 *and* a String in one go.
// The caller cannot specialize `apply` to a single type — the callee owns that.

pub trait IdFn {
    fn apply<T>(&self, x: T) -> T;
}

pub struct Identity;

impl IdFn for Identity {
    fn apply<T>(&self, x: T) -> T {
        x
    }
}

/// Apply a rank-2 identity function to two different types in a single call.
/// Note: `F: IdFn` (static dispatch) — not `dyn IdFn`, which is not allowed
/// because generic methods are not dyn-compatible.
pub fn apply_id<F: IdFn>(f: &F) -> (i32, String) {
    let x = f.apply(42_i32);
    let y = f.apply("hello".to_string());
    (x, y)
}

// ---------------------------------------------------------------------------
// Approach 2: Rank-2 "for-all" callback with Debug bound
// ---------------------------------------------------------------------------
// The trait carries a generic method, so the implementor must work for any
// `T: fmt::Debug`. This lets `apply_forall` call it with heterogeneous types.

pub trait ForAll {
    fn call<T: std::fmt::Debug>(&self, val: T) -> String;
}

pub struct ShowDebug;

impl ForAll for ShowDebug {
    fn call<T: std::fmt::Debug>(&self, val: T) -> String {
        format!("{val:?}")
    }
}

/// Apply a rank-2 "format as debug" function to multiple distinct types.
pub fn apply_forall<F: ForAll>(f: &F) -> Vec<String> {
    vec![f.call(42_i32), f.call("world"), f.call(vec![1_u8, 2, 3])]
}

// ---------------------------------------------------------------------------
// Approach 3: Rank-2 via higher-ranked trait bounds (HRTBs)
// ---------------------------------------------------------------------------
// Rust's `for<'a>` syntax is the closest native form of rank-2: a closure
// that must be valid for *every* lifetime, not just one chosen by the caller.
// This is rank-2 over lifetimes. We use it to apply a function to references
// of different-scoped data within a single call.

/// Accept any function that works for *any* lifetime `'a`, apply it to two
/// independently-scoped references, and return both results.
pub fn apply_twice_hrtb<F>(f: F) -> (usize, usize)
where
    F: for<'a> Fn(&'a str) -> usize,
{
    let result1 = {
        let s1 = String::from("hello world");
        f(&s1)
    };
    let result2 = {
        let s2 = String::from("rank-2");
        f(&s2)
    };
    (result1, result2)
}

// ---------------------------------------------------------------------------
// Approach 4: ST-monad style — rank-2 prevents state from leaking
// ---------------------------------------------------------------------------
// In Haskell/OCaml the phantom type `s` is existentially quantified at the
// boundary so no `STRef s a` can escape `runST`. In Rust we encode the same
// discipline with a lifetime brand: `s` is an invariant lifetime that the
// closure must be generic over, which the compiler enforces.
//
// The simplified version: a trait that represents a "pure state action"
// whose result type is fixed, so no internal state references can leak.

pub trait StAction {
    /// Run the state action and return a result that does not carry `s`.
    fn run(&self) -> i32;
}

pub struct CountRef {
    pub initial: i32,
}

impl StAction for CountRef {
    fn run(&self) -> i32 {
        // Simulate: new_ref(initial), increment, read — result escapes, ref doesn't.
        let mut val = self.initial;
        val += 1;
        val
    }
}

/// Accept any `StAction` — the result type is fixed (i32), so no internal
/// state can "leak" through the return type.
pub fn run_st<A: StAction>(action: &A) -> i32 {
    action.run()
}

// ---------------------------------------------------------------------------
// Approach 5: Apply a polymorphic transformation to a heterogeneous pair
// ---------------------------------------------------------------------------

pub struct HetPair<A, B>(pub A, pub B);

pub trait MapPair<A, B> {
    fn map_pair(&self, pair: HetPair<A, B>) -> HetPair<A, B>;
}

pub struct ClonePair;

impl<A: Clone, B: Clone> MapPair<A, B> for ClonePair {
    fn map_pair(&self, pair: HetPair<A, B>) -> HetPair<A, B> {
        HetPair(pair.0.clone(), pair.1.clone())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_applies_to_two_types() {
        let id = Identity;
        let (n, s) = apply_id(&id);
        assert_eq!(n, 42);
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_identity_preserves_value_for_multiple_types() {
        let id = Identity;
        assert_eq!(id.apply(99_u64), 99_u64);
        assert_eq!(id.apply("rust"), "rust");
        assert_eq!(id.apply(3.14_f64), 3.14_f64);
    }

    #[test]
    fn test_forall_formats_different_types() {
        let show = ShowDebug;
        let results = apply_forall(&show);
        assert_eq!(results[0], "42");
        assert_eq!(results[1], "\"world\"");
        assert_eq!(results[2], "[1, 2, 3]");
    }

    #[test]
    fn test_forall_call_directly() {
        let show = ShowDebug;
        assert_eq!(show.call(true), "true");
        assert_eq!(show.call(0_i32), "0");
    }

    #[test]
    fn test_hrtb_applies_to_different_scopes() {
        let (a, b) = apply_twice_hrtb(|s| s.len());
        assert_eq!(a, 11); // "hello world"
        assert_eq!(b, 6); // "rank-2"
    }

    #[test]
    fn test_hrtb_with_word_count() {
        let count_words = |s: &str| s.split_whitespace().count();
        let (a, b) = apply_twice_hrtb(count_words);
        assert_eq!(a, 2); // "hello world" → 2 words
        assert_eq!(b, 1); // "rank-2" → 1 word
    }

    #[test]
    fn test_st_action_result_escapes_state_does_not() {
        let action = CountRef { initial: 5 };
        assert_eq!(run_st(&action), 6);
    }

    #[test]
    fn test_st_action_pure_runs_consistently() {
        let action = CountRef { initial: 10 };
        // Running twice should return the same pure result each time.
        assert_eq!(run_st(&action), run_st(&action));
    }

    #[test]
    fn test_het_pair_clone() {
        let mapper = ClonePair;
        let pair = HetPair(42_i32, "hello".to_string());
        let result = mapper.map_pair(pair);
        assert_eq!(result.0, 42);
        assert_eq!(result.1, "hello");
    }
}
