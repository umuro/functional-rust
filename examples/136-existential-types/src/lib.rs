//! Example 136: Existential Types
//!
//! Rust encodes existential types in two complementary ways:
//!   - `impl Trait` in return position: opaque, zero-cost, single concrete type per call-site
//!   - `Box<dyn Trait>` / `dyn Trait`: runtime dispatch, heterogeneous collections
//!
//! OCaml uses first-class modules and GADTs to achieve the same hiding of the
//! concrete type behind an interface.

// ---------------------------------------------------------------------------
// Shared trait — the "interface" callers know about
// ---------------------------------------------------------------------------

/// Something that can produce a human-readable string.
pub trait Showable {
    fn show(&self) -> String;
}

// ---------------------------------------------------------------------------
// Concrete implementations (hidden from callers behind the existential)
// ---------------------------------------------------------------------------

struct Counter(u32);

impl Showable for Counter {
    fn show(&self) -> String {
        format!("counter({})", self.0)
    }
}

struct Label(String);

impl Showable for Label {
    fn show(&self) -> String {
        format!("label({:?})", self.0)
    }
}

impl Showable for i32 {
    fn show(&self) -> String {
        format!("i32({})", self)
    }
}

impl Showable for f64 {
    fn show(&self) -> String {
        format!("f64({:.2})", self)
    }
}

// ---------------------------------------------------------------------------
// Approach 1: `impl Trait` — opaque return type (static existential)
//
// The caller knows *some* Showable is returned; the concrete type is erased.
// All branches must return the *same* concrete type — chosen at compile time.
// ---------------------------------------------------------------------------

/// Returns a `Counter` wrapped as an opaque `impl Showable`.
/// Caller sees only the `Showable` interface; `Counter` is hidden.
pub fn make_counter(n: u32) -> impl Showable {
    Counter(n)
}

/// Returns a `Label` wrapped as an opaque `impl Showable`.
pub fn make_label(s: &str) -> impl Showable {
    Label(s.to_owned())
}

// ---------------------------------------------------------------------------
// Approach 2: `Box<dyn Trait>` — dynamic existential
//
// Analogous to OCaml's `(module SHOWABLE)` packing: the concrete type is
// erased at runtime. Enables heterogeneous collections and runtime dispatch.
// ---------------------------------------------------------------------------

/// Pack any `Showable + 'static` into an erased `Box<dyn Showable>`.
/// This mirrors OCaml's `pack_showable`: both hide the concrete type `T`.
pub fn pack(value: impl Showable + 'static) -> Box<dyn Showable> {
    Box::new(value)
}

/// Show every item in a heterogeneous collection of erased showables.
/// Mirrors OCaml's `show_any_list` over `any_list` (GADT existential).
pub fn show_all(items: &[Box<dyn Showable>]) -> Vec<String> {
    items.iter().map(|item| item.show()).collect()
}

// ---------------------------------------------------------------------------
// Approach 3: closure-based erasure (OCaml `{ show : unit -> string }`)
//
// Store only the *behaviour*, not the value. The concrete type disappears
// the moment the closure captures it — identical to OCaml's record approach.
// ---------------------------------------------------------------------------

/// An erased "showable" that owns just the behaviour, not the value.
pub struct ShowClosure {
    show_fn: Box<dyn Fn() -> String>,
}

impl ShowClosure {
    /// Capture `value` and `show_fn` into a closure; the type `T` is erased.
    pub fn new<T>(value: T, show_fn: impl Fn(&T) -> String + 'static) -> Self
    where
        T: 'static,
    {
        ShowClosure {
            show_fn: Box::new(move || show_fn(&value)),
        }
    }

    pub fn show(&self) -> String {
        (self.show_fn)()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_trait_counter() {
        let s = make_counter(7);
        assert_eq!(s.show(), "counter(7)");
    }

    #[test]
    fn test_impl_trait_label() {
        let s = make_label("hello");
        assert_eq!(s.show(), "label(\"hello\")");
    }

    #[test]
    fn test_dyn_trait_heterogeneous_collection() {
        // A Vec that holds i32, f64, Counter, Label — all erased to Box<dyn Showable>
        let items: Vec<Box<dyn Showable>> = vec![
            pack(42i32),
            pack(3.14f64),
            pack(Counter(99)),
            pack(Label("rust".to_owned())),
        ];

        let shown = show_all(&items);
        assert_eq!(shown[0], "i32(42)");
        assert_eq!(shown[1], "f64(3.14)");
        assert_eq!(shown[2], "counter(99)");
        assert_eq!(shown[3], "label(\"rust\")");
    }

    #[test]
    fn test_closure_erasure_hides_type() {
        // The integer `42` is captured; the type `i32` is completely hidden.
        let s = ShowClosure::new(42i32, |n| format!("the answer is {}", n));
        assert_eq!(s.show(), "the answer is 42");
    }

    #[test]
    fn test_closure_erasure_with_struct() {
        let s = ShowClosure::new(Counter(5), |c| c.show());
        assert_eq!(s.show(), "counter(5)");
    }

    #[test]
    fn test_show_all_empty() {
        let items: Vec<Box<dyn Showable>> = vec![];
        assert_eq!(show_all(&items), Vec::<String>::new());
    }
}
