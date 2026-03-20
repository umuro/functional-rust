#![allow(clippy::all)]
// Example 207: Prism Laws — ReviewPreview and PreviewReview
//
// Two round-trip laws guarantee that a Prism's `preview` and `review` are
// consistent with each other:
//
//   Law 1 — ReviewPreview:  preview(review(a)) = Some(a)
//     "If I build an S with review, I can always get back the exact a I used."
//
//   Law 2 — PreviewReview:  if preview(s) = Some(a) then review(a) = s
//     "If extraction succeeds, re-injection gives back the original value."
//
// A Prism that violates either law compiles and passes naive tests but breaks
// silently when composed.  These law-checkers let you catch violations at
// test time.

// ---------------------------------------------------------------------------
// Core Prism struct
// ---------------------------------------------------------------------------

type PreviewFn<S, A> = Box<dyn Fn(&S) -> Option<A>>;
type ReviewFn<S, A> = Box<dyn Fn(A) -> S>;

/// A Prism: two functions (`preview` and `review`) that must satisfy the
/// ReviewPreview and PreviewReview laws to be well-behaved.
pub struct Prism<S, A> {
    preview: PreviewFn<S, A>,
    review: ReviewFn<S, A>,
}

impl<S: 'static, A: 'static> Prism<S, A> {
    pub fn new(
        preview: impl Fn(&S) -> Option<A> + 'static,
        review: impl Fn(A) -> S + 'static,
    ) -> Self {
        Prism {
            preview: Box::new(preview),
            review: Box::new(review),
        }
    }

    pub fn preview(&self, s: &S) -> Option<A> {
        (self.preview)(s)
    }

    pub fn review(&self, a: A) -> S {
        (self.review)(a)
    }
}

// ---------------------------------------------------------------------------
// Law checkers
// ---------------------------------------------------------------------------

/// **Law 1 — ReviewPreview**: `preview(review(a)) == Some(a)`
///
/// Build an `S` from `a`, then try to extract it back.  A lawful Prism must
/// always succeed and return exactly the `a` we started with.
pub fn check_review_preview<S, A>(prism: &Prism<S, A>, a: A) -> bool
where
    A: PartialEq + Clone + 'static,
    S: 'static,
{
    let s = prism.review(a.clone());
    prism.preview(&s) == Some(a)
}

/// **Law 2 — PreviewReview**: if `preview(s) == Some(a)` then `review(a) == s`
///
/// If we can extract an `a` from `s`, then re-injecting it must reproduce `s`
/// exactly.  Returns `true` when the precondition fails (the law is vacuously
/// satisfied when `preview` returns `None`).
pub fn check_preview_review<S, A>(prism: &Prism<S, A>, s: &S) -> bool
where
    S: PartialEq + 'static,
    A: 'static,
{
    match prism.preview(s) {
        None => true, // vacuously lawful — precondition not met
        Some(a) => prism.review(a) == *s,
    }
}

// ---------------------------------------------------------------------------
// Domain model: a simple JSON type
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    JString(String),
    JInt(i64),
    JBool(bool),
    JNull,
    JArray(Vec<Json>),
}

// ---------------------------------------------------------------------------
// Approach 1: Lawful prisms (both laws hold)
// ---------------------------------------------------------------------------

/// Prism focusing on `Json::JString`. Lawful: round-trips are exact.
pub fn jstring_prism() -> Prism<Json, String> {
    Prism::new(
        |j| match j {
            Json::JString(s) => Some(s.clone()),
            _ => None,
        },
        Json::JString,
    )
}

/// Prism focusing on `Json::JInt`. Lawful.
pub fn jint_prism() -> Prism<Json, i64> {
    Prism::new(
        |j| match j {
            Json::JInt(n) => Some(*n),
            _ => None,
        },
        Json::JInt,
    )
}

/// Prism focusing on `Json::JBool`. Lawful.
pub fn jbool_prism() -> Prism<Json, bool> {
    Prism::new(
        |j| match j {
            Json::JBool(b) => Some(*b),
            _ => None,
        },
        Json::JBool,
    )
}

// ---------------------------------------------------------------------------
// Approach 2: Unlawful prism — demonstrates what law violation looks like
// ---------------------------------------------------------------------------

/// An **unlawful** prism: `preview` uppercases the string it returns, but
/// `review` stores the original case.  This violates Law 1 (ReviewPreview):
///
/// ```text
/// review("hello") -> JString("hello")
/// preview(JString("hello")) -> Some("HELLO")   ← not Some("hello")!
/// ```
///
/// Code compiles.  Basic usage looks fine.  But round-trips corrupt data.
pub fn unlawful_uppercase_prism() -> Prism<Json, String> {
    Prism::new(
        |j| match j {
            // BUG: transforms the value during extraction
            Json::JString(s) => Some(s.to_uppercase()),
            _ => None,
        },
        Json::JString,
    )
}

// ---------------------------------------------------------------------------
// Approach 3: Trait-based law verification (compile-time dispatch)
// ---------------------------------------------------------------------------

/// Implement this trait to get zero-cost Prism dispatch with built-in law checks.
pub trait LawfulPrism {
    type Source: Clone + PartialEq;
    type Focus: Clone + PartialEq + 'static;

    fn preview(s: &Self::Source) -> Option<Self::Focus>;
    fn review(a: Self::Focus) -> Self::Source;

    /// Checks Law 1 for a given focus value.
    fn law_review_preview(a: Self::Focus) -> bool {
        let s = Self::review(a.clone());
        Self::preview(&s) == Some(a)
    }

    /// Checks Law 2 for a given source value.
    fn law_preview_review(s: &Self::Source) -> bool {
        match Self::preview(s) {
            None => true,
            Some(a) => Self::review(a) == *s,
        }
    }
}

/// Zero-cost lawful prism for `Json::JString` via the trait approach.
pub struct JStringPrism;

impl LawfulPrism for JStringPrism {
    type Source = Json;
    type Focus = String;

    fn preview(j: &Json) -> Option<String> {
        match j {
            Json::JString(s) => Some(s.clone()),
            _ => None,
        }
    }

    fn review(s: String) -> Json {
        Json::JString(s)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Law 1: ReviewPreview ---

    #[test]
    fn test_law1_jstring_review_then_preview_returns_original() {
        let p = jstring_prism();
        assert!(check_review_preview(&p, "hello".to_string()));
    }

    #[test]
    fn test_law1_jint_review_then_preview_returns_original() {
        let p = jint_prism();
        assert!(check_review_preview(&p, 42_i64));
    }

    #[test]
    fn test_law1_jbool_review_then_preview_returns_original() {
        let p = jbool_prism();
        assert!(check_review_preview(&p, true));
        assert!(check_review_preview(&p, false));
    }

    #[test]
    fn test_law1_unlawful_prism_violates_review_preview() {
        // The unlawful prism transforms during preview, so Law 1 must fail.
        let p = unlawful_uppercase_prism();
        assert!(!check_review_preview(&p, "hello".to_string()));
    }

    // --- Law 2: PreviewReview ---

    #[test]
    fn test_law2_jstring_preview_then_review_gives_original() {
        let p = jstring_prism();
        let s = Json::JString("world".to_string());
        assert!(check_preview_review(&p, &s));
    }

    #[test]
    fn test_law2_jstring_wrong_variant_vacuously_true() {
        // preview returns None for JInt — law is vacuously satisfied.
        let p = jstring_prism();
        let s = Json::JInt(99);
        assert!(check_preview_review(&p, &s));
    }

    #[test]
    fn test_law2_jint_preview_then_review_gives_original() {
        let p = jint_prism();
        let s = Json::JInt(-7);
        assert!(check_preview_review(&p, &s));
    }

    #[test]
    fn test_law2_jnull_vacuously_true_for_jstring_prism() {
        let p = jstring_prism();
        assert!(check_preview_review(&p, &Json::JNull));
    }

    // --- Trait-based law checks ---

    #[test]
    fn test_trait_prism_law1_jstring() {
        assert!(JStringPrism::law_review_preview("rust".to_string()));
    }

    #[test]
    fn test_trait_prism_law2_jstring_matching_variant() {
        let s = Json::JString("optics".to_string());
        assert!(JStringPrism::law_preview_review(&s));
    }

    #[test]
    fn test_trait_prism_law2_jstring_non_matching_variant() {
        assert!(JStringPrism::law_preview_review(&Json::JBool(true)));
    }

    // --- Concrete behavior ---

    #[test]
    fn test_jstring_prism_preview_matching() {
        let p = jstring_prism();
        let j = Json::JString("abc".to_string());
        assert_eq!(p.preview(&j), Some("abc".to_string()));
    }

    #[test]
    fn test_jstring_prism_preview_non_matching() {
        let p = jstring_prism();
        assert_eq!(p.preview(&Json::JNull), None);
        assert_eq!(p.preview(&Json::JInt(1)), None);
    }

    #[test]
    fn test_jint_prism_review_then_preview() {
        let p = jint_prism();
        let constructed = p.review(100);
        assert_eq!(constructed, Json::JInt(100));
        assert_eq!(p.preview(&constructed), Some(100));
    }
}
