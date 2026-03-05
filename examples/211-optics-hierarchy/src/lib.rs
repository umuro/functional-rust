// Example 211: Optics Hierarchy — Iso ⊂ Lens ⊂ Traversal, Iso ⊂ Prism ⊂ Traversal
//
// The complete hierarchy, from most specific to most general:
//
//         Iso           ← lossless bijection; exactly 1 focus
//        /   \
//      Lens  Prism      ← Lens: exactly 1 focus (products)
//        \   /          ← Prism: 0-or-1 focus (sums)
//       Traversal       ← 0-to-many focuses (most general)
//
// Every Iso is a Lens and a Prism.
// Every Lens is a Traversal. Every Prism is a Traversal.
//
// Generic functions written at the Traversal level accept any optic via upcasting,
// eliminating duplicated code across optic types.

// ---------------------------------------------------------------------------
// Approach 1: Struct-based optics with explicit `as_*` upcast methods
// ---------------------------------------------------------------------------

type GetFn<S, A> = Box<dyn Fn(&S) -> A>;
type SetFn<S, A> = Box<dyn Fn(A, &S) -> S>;
type PreviewFn<S, A> = Box<dyn Fn(&S) -> Option<A>>;
type ReviewFn<S, A> = Box<dyn Fn(A) -> S>;
type OverFn<S, A> = Box<dyn Fn(&dyn Fn(&A) -> A, &S) -> S>;
type ToListFn<S, A> = Box<dyn Fn(&S) -> Vec<A>>;

// -- Traversal (most general: 0-to-many focuses) ----------------------------

/// A Traversal focuses on zero or more values of type `A` inside `S`.
/// Every other optic can be upcast to a Traversal.
///
/// OCaml equivalent:
/// ```ocaml
/// type ('s, 'a) traversal = { over : ('a -> 'a) -> 's -> 's; to_list : 's -> 'a list }
/// ```
pub struct Traversal<S, A> {
    over_fn: OverFn<S, A>,
    to_list_fn: ToListFn<S, A>,
}

impl<S: 'static, A: 'static> Traversal<S, A> {
    pub fn new(
        over: impl Fn(&dyn Fn(&A) -> A, &S) -> S + 'static,
        to_list: impl Fn(&S) -> Vec<A> + 'static,
    ) -> Self {
        Traversal {
            over_fn: Box::new(over),
            to_list_fn: Box::new(to_list),
        }
    }

    /// Apply `f` to every focused value, returning the updated structure.
    pub fn over(&self, f: impl Fn(&A) -> A, s: &S) -> S {
        (self.over_fn)(&f, s)
    }

    /// Collect all focused values into a `Vec`.
    pub fn collect_all(&self, s: &S) -> Vec<A> {
        (self.to_list_fn)(s)
    }

    /// Count how many values are focused.
    pub fn length_of(&self, s: &S) -> usize {
        self.collect_all(s).len()
    }
}

// -- Lens (exactly 1 focus: product types) ----------------------------------

/// A Lens focuses on exactly one value of type `A` inside a product `S`.
///
/// OCaml equivalent:
/// ```ocaml
/// type ('s, 'a) lens = { get : 's -> 'a; set : 'a -> 's -> 's }
/// ```
pub struct Lens<S, A> {
    get_fn: GetFn<S, A>,
    set_fn: SetFn<S, A>,
}

impl<S: Clone + 'static, A: Clone + 'static> Lens<S, A> {
    pub fn new(get: impl Fn(&S) -> A + 'static, set: impl Fn(A, &S) -> S + 'static) -> Self {
        Lens {
            get_fn: Box::new(get),
            set_fn: Box::new(set),
        }
    }

    pub fn get(&self, s: &S) -> A {
        (self.get_fn)(s)
    }

    pub fn set(&self, a: A, s: &S) -> S {
        (self.set_fn)(a, s)
    }

    pub fn over(&self, f: impl FnOnce(A) -> A, s: &S) -> S {
        self.set(f(self.get(s)), s)
    }

    /// Every Lens is a Traversal.
    ///
    /// The single focus becomes a singleton list; `over` applies to exactly
    /// one value. This is the "upcast" that lets a Lens be used wherever a
    /// Traversal is expected.
    pub fn as_traversal(self) -> Traversal<S, A> {
        use std::rc::Rc;
        // Share get_fn between the two closures via Rc — same pattern as
        // lens composition in example 204.
        let get_fn = Rc::new(self.get_fn);
        let get_fn2 = Rc::clone(&get_fn);
        let set_fn = self.set_fn;
        Traversal::new(
            move |f, s| {
                let a = get_fn(s);
                set_fn(f(&a), s)
            },
            move |s| vec![get_fn2(s)],
        )
    }
}

// -- Prism (0-or-1 focus: sum types) ----------------------------------------

/// A Prism focuses on 0 or 1 values of type `A` inside a sum `S`.
///
/// OCaml equivalent:
/// ```ocaml
/// type ('s, 'a) prism = { preview : 's -> 'a option; review : 'a -> 's }
/// ```
pub struct Prism<S, A> {
    preview_fn: PreviewFn<S, A>,
    review_fn: ReviewFn<S, A>,
}

impl<S: Clone + 'static, A: Clone + 'static> Prism<S, A> {
    pub fn new(
        preview: impl Fn(&S) -> Option<A> + 'static,
        review: impl Fn(A) -> S + 'static,
    ) -> Self {
        Prism {
            preview_fn: Box::new(preview),
            review_fn: Box::new(review),
        }
    }

    pub fn preview(&self, s: &S) -> Option<A> {
        (self.preview_fn)(s)
    }

    pub fn review(&self, a: A) -> S {
        (self.review_fn)(a)
    }

    /// Every Prism is a Traversal.
    ///
    /// When `preview` succeeds there is exactly one focus; otherwise zero.
    /// `over` is a no-op when the variant doesn't match — structural identity
    /// preserved via `Clone`.
    pub fn as_traversal(self) -> Traversal<S, A> {
        use std::rc::Rc;
        let preview_fn = Rc::new(self.preview_fn);
        let preview_fn2 = Rc::clone(&preview_fn);
        let review_fn = self.review_fn;
        Traversal::new(
            move |f, s| match preview_fn(s) {
                Some(a) => review_fn(f(&a)),
                None => s.clone(),
            },
            move |s| match preview_fn2(s) {
                Some(a) => vec![a],
                None => vec![],
            },
        )
    }
}

// -- Iso (lossless bijection) -----------------------------------------------

/// An Iso is a lossless two-way bijection between `S` and `A`.
/// It is simultaneously a Lens and a Prism — the most specific optic.
///
/// OCaml equivalent:
/// ```ocaml
/// type ('s, 'a) iso = { get : 's -> 'a; reverse_get : 'a -> 's }
/// ```
pub struct Iso<S, A> {
    get_fn: GetFn<S, A>,
    reverse_get_fn: ReviewFn<S, A>,
}

impl<S: Clone + 'static, A: Clone + 'static> Iso<S, A> {
    pub fn new(get: impl Fn(&S) -> A + 'static, reverse_get: impl Fn(A) -> S + 'static) -> Self {
        Iso {
            get_fn: Box::new(get),
            reverse_get_fn: Box::new(reverse_get),
        }
    }

    pub fn get(&self, s: &S) -> A {
        (self.get_fn)(s)
    }

    pub fn reverse_get(&self, a: A) -> S {
        (self.reverse_get_fn)(a)
    }

    /// An Iso is a Lens.
    ///
    /// `set(a, _s)` discards `_s` entirely — valid *only* for a lossless Iso,
    /// where `a` alone determines the full structure via `reverse_get`.
    /// A regular Lens cannot do this because `S` may carry other fields.
    pub fn as_lens(self) -> Lens<S, A> {
        let rev = self.reverse_get_fn;
        Lens::new(self.get_fn, move |a, _| rev(a))
    }

    /// An Iso is a Prism.
    ///
    /// `preview` always succeeds (wraps in `Some`) because the bijection is
    /// total — there is no "wrong variant". `review` = `reverse_get`.
    pub fn as_prism(self) -> Prism<S, A> {
        let get_fn = self.get_fn;
        Prism::new(move |s| Some(get_fn(s)), self.reverse_get_fn)
    }

    /// An Iso is a Traversal (via its Lens representation).
    pub fn as_traversal(self) -> Traversal<S, A> {
        self.as_lens().as_traversal()
    }
}

// ---------------------------------------------------------------------------
// Domain model
// ---------------------------------------------------------------------------

/// Temperature in Celsius — demonstrates Iso (`Celsius` ↔ `f64` is lossless).
#[derive(Debug, Clone, PartialEq)]
pub struct Celsius(pub f64);

/// A 2D point — demonstrates Lens (focus on one field of a product).
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A geometric shape — demonstrates Prism (focus on one variant of a sum).
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
}

/// Iso: `Celsius` ↔ `f64` — lossless because `Celsius(x)` and `x` are the same value.
pub fn celsius_iso() -> Iso<Celsius, f64> {
    Iso::new(|c: &Celsius| c.0, Celsius)
}

/// Lens: focus on `Point::x` — `y` is preserved unchanged on every `set`.
pub fn point_x_lens() -> Lens<Point, f64> {
    Lens::new(|p: &Point| p.x, |x, p: &Point| Point { x, y: p.y })
}

/// Prism: focus on `Shape::Circle`'s radius; `Shape::Rect` is a miss.
pub fn circle_radius_prism() -> Prism<Shape, f64> {
    Prism::new(
        |s: &Shape| match s {
            Shape::Circle { radius } => Some(*radius),
            Shape::Rect { .. } => None,
        },
        |r| Shape::Circle { radius: r },
    )
}

// ---------------------------------------------------------------------------
// Approach 2: Trait-based hierarchy (zero-cost compile-time dispatch)
// ---------------------------------------------------------------------------
//
// Each trait is a supertype of the next: IsoOptic ⊂ LensOptic ⊂ OpticBase
//                                         IsoOptic ⊂ PrismOptic ⊂ OpticBase
//
// Generic functions written against `OpticBase` work for any optic type.

/// Base trait for all optics. Provides `collect` (the universal read) and
/// `over_t` (the universal write). Generic code accepting `&dyn OpticBase`
/// works for Lens, Prism, and Iso without knowing which one it has.
pub trait OpticBase<S: Clone, A: Clone> {
    fn collect(&self, s: &S) -> Vec<A>;
    fn over_t(&self, f: &dyn Fn(&A) -> A, s: &S) -> S;

    fn length(&self, s: &S) -> usize {
        self.collect(s).len()
    }
}

/// A Lens optic: always exactly one focus. Extends `OpticBase` with `view`
/// (always-succeeding get) and `set`.
pub trait LensOptic<S: Clone, A: Clone>: OpticBase<S, A> {
    fn view(&self, s: &S) -> A;
    fn set(&self, a: A, s: &S) -> S;
}

/// A Prism optic: 0-or-1 focus. Extends `OpticBase` with `preview` and `review`.
pub trait PrismOptic<S: Clone, A: Clone>: OpticBase<S, A> {
    fn preview(&self, s: &S) -> Option<A>;
    fn review(&self, a: A) -> S;
}

/// An Iso optic: both a Lens and a Prism. `reverse_get` makes it explicit that
/// the mapping is lossless in both directions.
pub trait IsoOptic<S: Clone, A: Clone>: LensOptic<S, A> + PrismOptic<S, A> {
    fn reverse_get(&self, a: A) -> S;
}

// -- Concrete: PointXLens ---------------------------------------------------

/// Marker type for the `Point::x` lens (zero-cost, no heap allocation).
pub struct PointXLens;

impl OpticBase<Point, f64> for PointXLens {
    fn collect(&self, s: &Point) -> Vec<f64> {
        vec![s.x]
    }

    fn over_t(&self, f: &dyn Fn(&f64) -> f64, s: &Point) -> Point {
        Point { x: f(&s.x), y: s.y }
    }
}

impl LensOptic<Point, f64> for PointXLens {
    fn view(&self, s: &Point) -> f64 {
        s.x
    }

    fn set(&self, a: f64, s: &Point) -> Point {
        Point { x: a, y: s.y }
    }
}

// -- Concrete: CircleRadiusPrism --------------------------------------------

/// Marker type for the `Shape::Circle` radius prism.
pub struct CircleRadiusPrism;

impl OpticBase<Shape, f64> for CircleRadiusPrism {
    fn collect(&self, s: &Shape) -> Vec<f64> {
        match s {
            Shape::Circle { radius } => vec![*radius],
            Shape::Rect { .. } => vec![],
        }
    }

    fn over_t(&self, f: &dyn Fn(&f64) -> f64, s: &Shape) -> Shape {
        match s {
            Shape::Circle { radius } => Shape::Circle { radius: f(radius) },
            Shape::Rect { .. } => s.clone(),
        }
    }
}

impl PrismOptic<Shape, f64> for CircleRadiusPrism {
    fn preview(&self, s: &Shape) -> Option<f64> {
        match s {
            Shape::Circle { radius } => Some(*radius),
            Shape::Rect { .. } => None,
        }
    }

    fn review(&self, a: f64) -> Shape {
        Shape::Circle { radius: a }
    }
}

// -- Concrete: CelsiusIso ---------------------------------------------------

/// Marker type for the `Celsius` ↔ `f64` iso.
/// Implements `OpticBase`, `LensOptic`, `PrismOptic`, and `IsoOptic`,
/// demonstrating that an Iso satisfies the full hierarchy.
pub struct CelsiusIso;

impl OpticBase<Celsius, f64> for CelsiusIso {
    fn collect(&self, s: &Celsius) -> Vec<f64> {
        vec![s.0]
    }

    fn over_t(&self, f: &dyn Fn(&f64) -> f64, s: &Celsius) -> Celsius {
        Celsius(f(&s.0))
    }
}

impl LensOptic<Celsius, f64> for CelsiusIso {
    fn view(&self, s: &Celsius) -> f64 {
        s.0
    }

    /// `set` discards the old `Celsius` — valid because an Iso is lossless
    /// and `a` completely determines the result.
    fn set(&self, a: f64, _s: &Celsius) -> Celsius {
        Celsius(a)
    }
}

impl PrismOptic<Celsius, f64> for CelsiusIso {
    /// `preview` always returns `Some` for an Iso — no variant can be absent.
    fn preview(&self, s: &Celsius) -> Option<f64> {
        Some(s.0)
    }

    fn review(&self, a: f64) -> Celsius {
        Celsius(a)
    }
}

impl IsoOptic<Celsius, f64> for CelsiusIso {
    fn reverse_get(&self, a: f64) -> Celsius {
        Celsius(a)
    }
}

// ---------------------------------------------------------------------------
// Generic functions that accept any optic via `OpticBase`
// ---------------------------------------------------------------------------

/// Count the number of focuses any optic has on `s`.
/// - Lens: always 1
/// - Prism: 0 (miss) or 1 (hit)
/// - Iso: always 1
pub fn count_focuses<S: Clone, A: Clone>(optic: &dyn OpticBase<S, A>, s: &S) -> usize {
    optic.length(s)
}

/// Return the first focused value, if any. Uniform across all optic types.
pub fn first_focus<S: Clone, A: Clone>(optic: &dyn OpticBase<S, A>, s: &S) -> Option<A> {
    optic.collect(s).into_iter().next()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Approach 1: struct-based upcasting -----------------------------------

    #[test]
    fn test_iso_as_lens_get_and_set() {
        let lens = celsius_iso().as_lens();
        let c = Celsius(100.0);
        assert_eq!(lens.get(&c), 100.0);
        // set discards the original Celsius — valid because the Iso is lossless
        assert_eq!(lens.set(37.0, &c), Celsius(37.0));
    }

    #[test]
    fn test_iso_as_prism_preview_always_succeeds() {
        let prism = celsius_iso().as_prism();
        // An Iso's preview always returns Some — no variant can be absent
        assert_eq!(prism.preview(&Celsius(25.5)), Some(25.5));
        assert_eq!(prism.preview(&Celsius(-10.0)), Some(-10.0));
    }

    #[test]
    fn test_iso_as_prism_review_roundtrips() {
        let prism = celsius_iso().as_prism();
        assert_eq!(prism.review(100.0), Celsius(100.0));
        assert_eq!(prism.review(0.0), Celsius(0.0));
    }

    #[test]
    fn test_iso_as_traversal_collect_gives_singleton() {
        let trav = celsius_iso().as_traversal();
        // An Iso always has exactly 1 focus — collect yields a singleton
        assert_eq!(trav.collect_all(&Celsius(42.0)), vec![42.0]);
        assert_eq!(trav.length_of(&Celsius(42.0)), 1);
    }

    #[test]
    fn test_iso_as_traversal_over_transforms_value() {
        let trav = celsius_iso().as_traversal();
        // Celsius → Kelvin offset: add 273.15
        let result = trav.over(|f| f + 273.15, &Celsius(100.0));
        assert!((result.0 - 373.15).abs() < 1e-10);
    }

    #[test]
    fn test_lens_as_traversal_collect_gives_singleton() {
        let trav = point_x_lens().as_traversal();
        let p = Point { x: 3.0, y: 4.0 };
        // A Lens always has exactly 1 focus — same as Iso at the Traversal level
        assert_eq!(trav.collect_all(&p), vec![3.0]);
        assert_eq!(trav.length_of(&p), 1);
    }

    #[test]
    fn test_lens_as_traversal_over_modifies_only_x() {
        let trav = point_x_lens().as_traversal();
        let p = Point { x: 2.0, y: 5.0 };
        let result = trav.over(|x| x * 10.0, &p);
        // x is scaled; y is preserved — Traversal has the same semantics as Lens
        assert_eq!(result, Point { x: 20.0, y: 5.0 });
    }

    #[test]
    fn test_prism_as_traversal_collect_hit() {
        let trav = circle_radius_prism().as_traversal();
        // Circle matches → exactly 1 focus
        assert_eq!(trav.collect_all(&Shape::Circle { radius: 7.0 }), vec![7.0]);
        assert_eq!(trav.length_of(&Shape::Circle { radius: 7.0 }), 1);
    }

    #[test]
    fn test_prism_as_traversal_collect_miss() {
        let trav = circle_radius_prism().as_traversal();
        // Rect doesn't match → 0 focuses
        let rect = Shape::Rect {
            width: 4.0,
            height: 3.0,
        };
        assert_eq!(trav.collect_all(&rect), vec![]);
        assert_eq!(trav.length_of(&rect), 0);
    }

    #[test]
    fn test_prism_as_traversal_over_hit_scales_radius() {
        let trav = circle_radius_prism().as_traversal();
        let result = trav.over(|r| r * 2.0, &Shape::Circle { radius: 5.0 });
        assert_eq!(result, Shape::Circle { radius: 10.0 });
    }

    #[test]
    fn test_prism_as_traversal_over_miss_is_noop() {
        let trav = circle_radius_prism().as_traversal();
        let rect = Shape::Rect {
            width: 4.0,
            height: 3.0,
        };
        // over on a miss returns a structural clone — no modification
        assert_eq!(trav.over(|r| r * 2.0, &rect), rect);
    }

    // -- Approach 2: trait-based hierarchy ------------------------------------

    #[test]
    fn test_trait_lens_count_is_always_one() {
        let lens = PointXLens;
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(count_focuses(&lens, &p), 1);
    }

    #[test]
    fn test_trait_prism_count_hit_and_miss() {
        let prism = CircleRadiusPrism;
        assert_eq!(count_focuses(&prism, &Shape::Circle { radius: 3.0 }), 1);
        assert_eq!(
            count_focuses(
                &prism,
                &Shape::Rect {
                    width: 1.0,
                    height: 1.0
                }
            ),
            0
        );
    }

    #[test]
    fn test_trait_iso_count_is_always_one() {
        let iso = CelsiusIso;
        // Iso is both Lens and Prism, so count is always 1
        assert_eq!(count_focuses(&iso, &Celsius(0.0)), 1);
        assert_eq!(count_focuses(&iso, &Celsius(-273.15)), 1);
    }

    #[test]
    fn test_trait_lens_view_set_and_over_t() {
        let lens = PointXLens;
        let p = Point { x: 1.5, y: 9.0 };
        assert_eq!(lens.view(&p), 1.5);
        assert_eq!(lens.set(42.0, &p), Point { x: 42.0, y: 9.0 });
        assert_eq!(lens.over_t(&|x| x + 1.0, &p), Point { x: 2.5, y: 9.0 });
    }

    #[test]
    fn test_trait_prism_preview_review_and_over_t() {
        let prism = CircleRadiusPrism;
        let circle = Shape::Circle { radius: 4.0 };
        let rect = Shape::Rect {
            width: 2.0,
            height: 3.0,
        };
        assert_eq!(prism.preview(&circle), Some(4.0));
        assert_eq!(prism.preview(&rect), None);
        assert_eq!(prism.review(8.0), Shape::Circle { radius: 8.0 });
        // over_t on a miss returns the original structure unchanged
        assert_eq!(prism.over_t(&|r| r * 2.0, &rect), rect);
    }

    #[test]
    fn test_trait_iso_acts_as_both_lens_and_prism() {
        let iso = CelsiusIso;
        let c = Celsius(20.0);
        // As LensOptic
        assert_eq!(iso.view(&c), 20.0);
        assert_eq!(iso.set(37.0, &c), Celsius(37.0));
        // As PrismOptic — preview always Some for an Iso
        assert_eq!(iso.preview(&c), Some(20.0));
        assert_eq!(iso.review(100.0), Celsius(100.0));
        // As IsoOptic
        assert_eq!(iso.reverse_get(0.0), Celsius(0.0));
    }

    #[test]
    fn test_first_focus_generic_across_all_optic_types() {
        let lens = PointXLens;
        let prism = CircleRadiusPrism;
        let iso = CelsiusIso;
        // Lens: always Some
        assert_eq!(first_focus(&lens, &Point { x: 7.0, y: 0.0 }), Some(7.0));
        // Prism: Some on hit, None on miss
        assert_eq!(
            first_focus(&prism, &Shape::Circle { radius: 3.0 }),
            Some(3.0)
        );
        assert_eq!(
            first_focus(
                &prism,
                &Shape::Rect {
                    width: 1.0,
                    height: 2.0
                }
            ),
            None
        );
        // Iso: always Some
        assert_eq!(first_focus(&iso, &Celsius(99.0)), Some(99.0));
    }

    #[test]
    fn test_hierarchy_upcasting_same_semantics_lens_and_traversal() {
        // Using a Lens directly and via Traversal upcast must give the same result
        let lens = point_x_lens();
        let trav = point_x_lens().as_traversal();
        let p = Point { x: 5.0, y: 3.0 };
        let via_lens = lens.over(|x| x + 1.0, &p);
        let via_trav = trav.over(|x| x + 1.0, &p);
        assert_eq!(via_lens, via_trav);
    }
}
