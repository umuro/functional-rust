//! # Example 212: Van Laarhoven Lenses
//!
//! The Van Laarhoven encoding collapses all lens operations into one function type:
//!   `type Lens s a = ∀f. Functor f ⇒ (a → f a) → s → f s`
//!
//! Choosing the functor selects the operation:
//! - `f = Identity` → `over` (modify the focused field)
//! - `f = Const r`  → `view` (extract the focused field)
//!
//! The deeper payoff: **composition is ordinary function composition**.
//! In Haskell, `lens_b_in_a . lens_a_in_s` is a valid composed lens.
//! Rust lacks rank-2 types, so we specialise to Identity and Const and
//! bundle them into a struct — but the composition logic is identical.

use std::marker::PhantomData;
use std::rc::Rc;

// ============================================================================
// Functors
// ============================================================================

/// Identity functor. `fmap f (Identity x) = Identity (f x)`.
///
/// Plugging Identity into a VL lens gives `over`: the lens applies `f` to
/// the focused element and returns the updated structure.
pub struct Identity<A>(pub A);

impl<A> Identity<A> {
    pub fn run(self) -> A {
        self.0
    }

    pub fn map<B>(self, f: impl FnOnce(A) -> B) -> Identity<B> {
        Identity(f(self.0))
    }
}

/// Const functor. `fmap _ (Const r) = Const r` — the payload is ignored.
///
/// Plugging `Const` into a VL lens gives `view`: the lens feeds the focused
/// element to `f` (which returns `Const r`), and the functor laws propagate
/// `r` out through the structure without touching it.
pub struct Const<R, B>(pub R, PhantomData<B>);

impl<R, B> Const<R, B> {
    pub fn new(r: R) -> Self {
        Const(r, PhantomData)
    }

    pub fn run(self) -> R {
        self.0
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn map<C>(self, _f: impl FnOnce(B) -> C) -> Const<R, C> {
        Const(self.0, PhantomData)
    }
}

// ============================================================================
// Van Laarhoven Lens
// ============================================================================

/// Lifts a modifier `a → a` to a structure modifier `s → s` (Identity functor).
/// Type alias to keep the struct definition readable.
type IdentityApp<S, A> = Rc<dyn Fn(Rc<dyn Fn(A) -> A>) -> Rc<dyn Fn(S) -> S>>;

/// Reads the focused value out of a structure (Const functor).
type ConstApp<S, A> = Rc<dyn Fn(&S) -> A>;

/// A Van Laarhoven lens focusing on a field of type `A` inside structure `S`.
///
/// In Haskell this is a single rank-2 polymorphic function:
/// ```text
/// type Lens s a = ∀f. Functor f ⇒ (a → f a) → s → f s
/// ```
///
/// Since Rust lacks `∀f`, we represent the two relevant specialisations
/// explicitly:
/// - `apply_identity` — the Identity functor application  → `over`/`set`
/// - `apply_const`    — the Const functor application     → `view`
///
/// ### Composition
/// The VL payoff: composing two lenses mirrors Haskell's `(.)`:
/// ```text
/// (outer . inner).apply(f) = outer.apply(inner.apply(f))
/// ```
/// One lens is simply passed as an argument to the other's `apply` —
/// no special `compose_lens` combinator is required.
pub struct VLLens<S: 'static, A: 'static> {
    /// Identity specialisation: lifts `a → a` to `s → s`.
    apply_identity: IdentityApp<S, A>,

    /// Const specialisation: reads the focused value out of `s`.
    apply_const: ConstApp<S, A>,
}

impl<S: 'static, A: 'static> VLLens<S, A> {
    /// Build a VL lens from a `getter` and a consuming `setter`.
    pub fn new(getter: impl Fn(&S) -> A + 'static, setter: impl Fn(S, A) -> S + 'static) -> Self {
        let getter = Rc::new(getter);
        let setter = Rc::new(setter);

        let getter_id = Rc::clone(&getter);
        let setter_id = Rc::clone(&setter);

        VLLens {
            // Identity application: given modifier `f: a → a`, produce `s → s`
            // In Haskell: `λf s → run_identity (l (Identity . f) s)`
            apply_identity: Rc::new(move |f: Rc<dyn Fn(A) -> A>| {
                let g = Rc::clone(&getter_id);
                let s = Rc::clone(&setter_id);
                let f = Rc::clone(&f);
                Rc::new(move |structure: S| {
                    let a = g(&structure);
                    let a2 = f(a);
                    s(structure, a2)
                }) as Rc<dyn Fn(S) -> S>
            }),
            // Const application: given `s`, extract the focused `a`
            // In Haskell: `λs → get_const (l Const s)`
            apply_const: Rc::new(move |structure: &S| getter(structure)),
        }
    }

    /// Extract the focused value from `s`.
    ///
    /// Corresponds to the Const functor application:
    /// `view l s = get_const (l Const s)`
    pub fn view(&self, s: &S) -> A {
        (self.apply_const)(s)
    }

    /// Apply a pure function to the focused value and return the updated structure.
    ///
    /// Corresponds to the Identity functor application:
    /// `over l f s = run_identity (l (Identity . f) s)`
    pub fn over(&self, s: S, f: impl Fn(A) -> A + 'static) -> S {
        let modifier = (self.apply_identity)(Rc::new(f));
        modifier(s)
    }

    /// Replace the focused value with `a`.
    pub fn set(&self, s: S, a: A) -> S
    where
        A: Clone,
    {
        self.over(s, move |_| a.clone())
    }

    /// Compose two VL lenses.
    ///
    /// `self` focuses `S → A`; `inner` focuses `A → B`.
    /// Result focuses `S → B`.
    ///
    /// The implementation mirrors Haskell's `(.)`:
    /// ```text
    /// (outer . inner).apply(f) = outer.apply(inner.apply(f))
    /// ```
    /// This is plain function application — the same formula as `(.)` in Haskell.
    pub fn compose<B: 'static>(self, inner: VLLens<A, B>) -> VLLens<S, B> {
        let outer_id = self.apply_identity;
        let inner_id = inner.apply_identity;
        let outer_c = self.apply_const;
        let inner_c = inner.apply_const;

        VLLens {
            // Composition = outer.apply(inner.apply(f))
            // This is function composition: outer ∘ inner
            apply_identity: Rc::new(move |f: Rc<dyn Fn(B) -> B>| {
                let inner_lifted: Rc<dyn Fn(A) -> A> = (inner_id)(f);
                (outer_id)(inner_lifted)
            }),
            apply_const: Rc::new(move |s: &S| {
                let a = (outer_c)(s);
                (inner_c)(&a)
            }),
        }
    }
}

// ============================================================================
// Domain types
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct Address {
    pub street: String,
    pub city: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub address: Address,
}

// ============================================================================
// Lenses for the domain
// ============================================================================

pub fn person_name() -> VLLens<Person, String> {
    VLLens::new(|p: &Person| p.name.clone(), |p, name| Person { name, ..p })
}

pub fn person_age() -> VLLens<Person, u32> {
    VLLens::new(|p: &Person| p.age, |p, age| Person { age, ..p })
}

pub fn person_address() -> VLLens<Person, Address> {
    VLLens::new(
        |p: &Person| p.address.clone(),
        |p, address| Person { address, ..p },
    )
}

pub fn address_city() -> VLLens<Address, String> {
    VLLens::new(
        |a: &Address| a.city.clone(),
        |a, city| Address { city, ..a },
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn alice() -> Person {
        Person {
            name: "Alice".to_string(),
            age: 30,
            address: Address {
                street: "1 Elm St".to_string(),
                city: "Springfield".to_string(),
            },
        }
    }

    // --- view ---

    #[test]
    fn test_view_simple_field() {
        let p = alice();
        assert_eq!(person_age().view(&p), 30);
    }

    #[test]
    fn test_view_string_field() {
        let p = alice();
        assert_eq!(person_name().view(&p), "Alice");
    }

    #[test]
    fn test_view_nested_via_compose() {
        // Compose person_address and address_city to reach Person → city
        let person_city = person_address().compose(address_city());
        let p = alice();
        assert_eq!(person_city.view(&p), "Springfield");
    }

    // --- over ---

    #[test]
    fn test_over_increments_age() {
        let p = alice();
        let updated = person_age().over(p, |age| age + 1);
        assert_eq!(updated.age, 31);
        assert_eq!(updated.name, "Alice"); // other fields unchanged
    }

    #[test]
    fn test_over_uppercases_name() {
        let p = alice();
        let updated = person_name().over(p, |n| n.to_uppercase());
        assert_eq!(updated.name, "ALICE");
        assert_eq!(updated.age, 30);
    }

    #[test]
    fn test_over_composed_lens_modifies_nested_field() {
        // Compose to get Person → city, then over to upper-case it
        let person_city = person_address().compose(address_city());
        let p = alice();
        let updated = person_city.over(p, |c| c.to_uppercase());
        assert_eq!(updated.address.city, "SPRINGFIELD");
        assert_eq!(updated.address.street, "1 Elm St"); // sibling unchanged
        assert_eq!(updated.name, "Alice"); // parent unchanged
    }

    // --- set ---

    #[test]
    fn test_set_replaces_age() {
        let p = alice();
        let updated = person_age().set(p, 99);
        assert_eq!(updated.age, 99);
    }

    #[test]
    fn test_set_composed_lens_replaces_city() {
        let person_city = person_address().compose(address_city());
        let p = alice();
        let updated = person_city.set(p, "Shelbyville".to_string());
        assert_eq!(updated.address.city, "Shelbyville");
        assert_eq!(updated.address.street, "1 Elm St");
        assert_eq!(updated.age, 30);
    }

    // --- identity / immutability laws ---

    #[test]
    fn test_original_not_mutated() {
        let p = alice();
        let _updated = person_age().over(p.clone(), |a| a + 100);
        // p.clone() is updated; original p is still 30
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_over_with_identity_function_preserves_structure() {
        let p = alice();
        let updated = person_age().over(p.clone(), |a| a);
        assert_eq!(updated, p);
    }

    // --- VL insight: composition = function composition ---

    #[test]
    fn test_composition_is_function_application() {
        // The VL composition formula:
        //   (outer.compose(inner)).apply(f) == outer.apply(inner.apply(f))
        //
        // We verify both paths produce the same result.
        let f: Rc<dyn Fn(String) -> String> = Rc::new(|c: String| c.to_uppercase());

        // Path A: compose lenses, then apply
        let person_city = person_address().compose(address_city());
        let result_a = person_city.over(alice(), |c| c.to_uppercase());

        // Path B: apply inner lens first, then outer lens (manual function composition)
        let inner_lifted = address_city().apply_identity.clone();
        let inner_fn: Rc<dyn Fn(Address) -> Address> = (inner_lifted)(f.clone());
        let outer_lifted = person_address().apply_identity.clone();
        let outer_fn: Rc<dyn Fn(Person) -> Person> = (outer_lifted)(inner_fn);
        let result_b = outer_fn(alice());

        assert_eq!(result_a, result_b);
    }

    // --- functor types ---

    #[test]
    fn test_identity_functor_map() {
        let id = Identity(5i32);
        let mapped = id.map(|x| x * 2);
        assert_eq!(mapped.run(), 10);
    }

    #[test]
    fn test_const_functor_map_ignores_function() {
        let c: Const<i32, String> = Const::new(42);
        let mapped: Const<i32, bool> = c.map(|_s: String| true);
        assert_eq!(mapped.run(), 42); // payload preserved, phantom type changed
    }
}
