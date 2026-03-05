/// Example 206: Prism Basics — Optics for Enum Variants
///
/// A Prism is a lens-like abstraction that focuses on one variant of an enum.
/// Unlike a Lens (which always succeeds), a Prism's `preview` returns `Option`
/// because the focused variant might not be present.
///
/// Two operations define a Prism:
///   - `preview`: try to extract the inner value (`S -> Option<A>`)
///   - `review`:  construct the outer type from the inner value (`A -> S`)

// ---------------------------------------------------------------------------
// Approach 1: Struct-based Prism with boxed closures
// ---------------------------------------------------------------------------

/// A Prism focuses on one variant of a sum type `S`, exposing payload of type `A`.
pub struct Prism<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,
    review: Box<dyn Fn(A) -> S>,
}

impl<S: 'static, A: 'static> Prism<S, A> {
    /// Build a Prism from a `preview` function and a `review` function.
    pub fn new(
        preview: impl Fn(&S) -> Option<A> + 'static,
        review: impl Fn(A) -> S + 'static,
    ) -> Self {
        Prism {
            preview: Box::new(preview),
            review: Box::new(review),
        }
    }

    /// Try to extract the focused value. Returns `None` for the wrong variant.
    pub fn preview(&self, s: &S) -> Option<A> {
        (self.preview)(s)
    }

    /// Construct the outer type from the focused value.
    pub fn review(&self, a: A) -> S {
        (self.review)(a)
    }

    /// If the focused variant is present, apply `f` to its payload and
    /// re-wrap. Otherwise, return a clone of `s` unchanged.
    pub fn over(&self, s: &S, f: impl FnOnce(A) -> A) -> S
    where
        S: Clone,
    {
        match self.preview(s) {
            Some(a) => self.review(f(a)),
            None => s.clone(),
        }
    }

    /// Set the focused payload to `a` if the variant matches; otherwise clone.
    pub fn set(&self, s: &S, a: A) -> S
    where
        S: Clone,
    {
        self.over(s, |_| a)
    }

    /// `true` iff `s` is the focused variant.
    pub fn is_match(&self, s: &S) -> bool {
        self.preview(s).is_some()
    }
}

// ---------------------------------------------------------------------------
// Domain model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

/// Prism focusing on `Shape::Circle`.
pub fn circle_prism() -> Prism<Shape, f64> {
    Prism::new(
        |s| match s {
            Shape::Circle(r) => Some(*r),
            _ => None,
        },
        Shape::Circle,
    )
}

/// Prism focusing on `Shape::Rectangle`.
pub fn rectangle_prism() -> Prism<Shape, (f64, f64)> {
    Prism::new(
        |s| match s {
            Shape::Rectangle(w, h) => Some((*w, *h)),
            _ => None,
        },
        |(w, h)| Shape::Rectangle(w, h),
    )
}

/// Prism focusing on `Shape::Triangle`.
pub fn triangle_prism() -> Prism<Shape, (f64, f64, f64)> {
    Prism::new(
        |s| match s {
            Shape::Triangle(a, b, c) => Some((*a, *b, *c)),
            _ => None,
        },
        |(a, b, c)| Shape::Triangle(a, b, c),
    )
}

// ---------------------------------------------------------------------------
// Approach 2: Prism for `Option` itself (the canonical "some prism")
// ---------------------------------------------------------------------------

/// A Prism that focuses on the `Some` branch of `Option<A>`.
pub fn some_prism<A: Clone + 'static>() -> Prism<Option<A>, A> {
    Prism::new(
        |opt| opt.clone(),
        Some,
    )
}

// ---------------------------------------------------------------------------
// Approach 3: trait-based Prism (zero-allocation, compile-time dispatch)
// ---------------------------------------------------------------------------

/// Trait version: implement this to get a zero-cost prism with no boxing.
pub trait PrismTrait {
    type Source: Clone;
    type Focus;

    fn preview(s: &Self::Source) -> Option<Self::Focus>;
    fn review(a: Self::Focus) -> Self::Source;

    fn over(s: &Self::Source, f: impl FnOnce(Self::Focus) -> Self::Focus) -> Self::Source {
        match Self::preview(s) {
            Some(a) => Self::review(f(a)),
            None => s.clone(),
        }
    }

    fn is_match(s: &Self::Source) -> bool {
        Self::preview(s).is_some()
    }
}

/// Zero-cost prism for `Shape::Circle`.
pub struct CirclePrism;

impl PrismTrait for CirclePrism {
    type Source = Shape;
    type Focus = f64;

    fn preview(s: &Shape) -> Option<f64> {
        match s {
            Shape::Circle(r) => Some(*r),
            _ => None,
        }
    }

    fn review(r: f64) -> Shape {
        Shape::Circle(r)
    }
}

/// Zero-cost prism for `Shape::Rectangle`.
pub struct RectanglePrism;

impl PrismTrait for RectanglePrism {
    type Source = Shape;
    type Focus = (f64, f64);

    fn preview(s: &Shape) -> Option<(f64, f64)> {
        match s {
            Shape::Rectangle(w, h) => Some((*w, *h)),
            _ => None,
        }
    }

    fn review((w, h): (f64, f64)) -> Shape {
        Shape::Rectangle(w, h)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- closure-based Prism ---

    #[test]
    fn test_preview_matches_correct_variant() {
        let prism = circle_prism();
        let shape = Shape::Circle(3.0);
        assert_eq!(prism.preview(&shape), Some(3.0));
    }

    #[test]
    fn test_preview_returns_none_for_wrong_variant() {
        let prism = circle_prism();
        let shape = Shape::Rectangle(4.0, 5.0);
        assert_eq!(prism.preview(&shape), None);
    }

    #[test]
    fn test_review_constructs_correct_variant() {
        let prism = circle_prism();
        assert_eq!(prism.review(7.0), Shape::Circle(7.0));
    }

    #[test]
    fn test_over_modifies_matching_variant() {
        let prism = circle_prism();
        let shape = Shape::Circle(3.0);
        let result = prism.over(&shape, |r| r * 2.0);
        assert_eq!(result, Shape::Circle(6.0));
    }

    #[test]
    fn test_over_leaves_non_matching_variant_unchanged() {
        let prism = circle_prism();
        let shape = Shape::Rectangle(4.0, 5.0);
        let result = prism.over(&shape, |r| r * 2.0);
        assert_eq!(result, Shape::Rectangle(4.0, 5.0));
    }

    #[test]
    fn test_set_replaces_payload_of_matching_variant() {
        let prism = circle_prism();
        let shape = Shape::Circle(3.0);
        assert_eq!(prism.set(&shape, 10.0), Shape::Circle(10.0));
    }

    #[test]
    fn test_set_leaves_non_matching_variant_unchanged() {
        let prism = circle_prism();
        let shape = Shape::Triangle(1.0, 2.0, 3.0);
        assert_eq!(prism.set(&shape, 10.0), Shape::Triangle(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_is_match_true_for_correct_variant() {
        let prism = circle_prism();
        assert!(prism.is_match(&Shape::Circle(1.0)));
    }

    #[test]
    fn test_is_match_false_for_wrong_variant() {
        let prism = circle_prism();
        assert!(!prism.is_match(&Shape::Rectangle(2.0, 3.0)));
    }

    #[test]
    fn test_rectangle_prism_preview() {
        let prism = rectangle_prism();
        let shape = Shape::Rectangle(4.0, 5.0);
        assert_eq!(prism.preview(&shape), Some((4.0, 5.0)));
    }

    #[test]
    fn test_rectangle_prism_over_scales_width() {
        let prism = rectangle_prism();
        let shape = Shape::Rectangle(4.0, 5.0);
        let result = prism.over(&shape, |(w, h)| (w * 2.0, h));
        assert_eq!(result, Shape::Rectangle(8.0, 5.0));
    }

    // --- some_prism ---

    #[test]
    fn test_some_prism_preview_some() {
        let prism = some_prism::<i32>();
        assert_eq!(prism.preview(&Some(42)), Some(42));
    }

    #[test]
    fn test_some_prism_preview_none() {
        let prism = some_prism::<i32>();
        assert_eq!(prism.preview(&None), None);
    }

    #[test]
    fn test_some_prism_over_doubles_value() {
        let prism = some_prism::<i32>();
        assert_eq!(prism.over(&Some(5), |x| x * 2), Some(10));
    }

    #[test]
    fn test_some_prism_over_none_stays_none() {
        let prism = some_prism::<i32>();
        assert_eq!(prism.over(&None, |x| x * 2), None);
    }

    // --- trait-based Prism ---

    #[test]
    fn test_trait_prism_preview_circle() {
        assert_eq!(CirclePrism::preview(&Shape::Circle(2.5)), Some(2.5));
        assert_eq!(CirclePrism::preview(&Shape::Rectangle(1.0, 2.0)), None);
    }

    #[test]
    fn test_trait_prism_review_circle() {
        assert_eq!(CirclePrism::review(4.0), Shape::Circle(4.0));
    }

    #[test]
    fn test_trait_prism_over_circle() {
        let shape = Shape::Circle(3.0);
        let result = CirclePrism::over(&shape, |r| r + 1.0);
        assert_eq!(result, Shape::Circle(4.0));
    }

    #[test]
    fn test_trait_prism_is_match_rectangle() {
        assert!(RectanglePrism::is_match(&Shape::Rectangle(2.0, 3.0)));
        assert!(!RectanglePrism::is_match(&Shape::Circle(1.0)));
    }
}
