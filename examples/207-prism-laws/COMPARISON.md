# OCaml vs Rust: Prism Laws — ReviewPreview and PreviewReview

## Side-by-Side Code

### OCaml

```ocaml
type ('s, 'a) prism = {
  preview : 's -> 'a option;
  review  : 'a -> 's;
}

(* Law 1 — ReviewPreview: preview (review a) = Some a *)
let check_review_preview prism a =
  prism.preview (prism.review a) = Some a

(* Law 2 — PreviewReview: if preview s = Some a then review a = s *)
let check_preview_review prism s =
  match prism.preview s with
  | None   -> true   (* vacuously lawful *)
  | Some a -> prism.review a = s

(* Lawful prism for JString *)
let jstring_prism = {
  preview = (function JString s -> Some s | _ -> None);
  review  = (fun s -> JString s);
}

(* Unlawful prism — transforms value during preview *)
let unlawful_uppercase = {
  preview = (function JString s -> Some (String.uppercase_ascii s) | _ -> None);
  review  = (fun s -> JString s);
}
```

### Rust (idiomatic — struct with boxed closures)

```rust
pub struct Prism<S, A> {
    preview: Box<dyn Fn(&S) -> Option<A>>,
    review:  Box<dyn Fn(A) -> S>,
}

/// Law 1 — ReviewPreview: preview(review(a)) == Some(a)
pub fn check_review_preview<S, A>(prism: &Prism<S, A>, a: A) -> bool
where
    A: PartialEq + Clone + 'static,
    S: 'static,
{
    let s = prism.review(a.clone());
    prism.preview(&s) == Some(a)
}

/// Law 2 — PreviewReview: if preview(s) == Some(a) then review(a) == s
pub fn check_preview_review<S, A>(prism: &Prism<S, A>, s: &S) -> bool
where
    S: PartialEq + 'static,
    A: 'static,
{
    match prism.preview(s) {
        None    => true,
        Some(a) => prism.review(a) == *s,
    }
}

pub fn jstring_prism() -> Prism<Json, String> {
    Prism::new(
        |j| match j { Json::JString(s) => Some(s.clone()), _ => None },
        Json::JString,
    )
}

pub fn unlawful_uppercase_prism() -> Prism<Json, String> {
    Prism::new(
        |j| match j { Json::JString(s) => Some(s.to_uppercase()), _ => None },
        Json::JString,
    )
}
```

### Rust (functional — zero-cost trait dispatch)

```rust
pub trait LawfulPrism {
    type Source: Clone + PartialEq;
    type Focus: Clone + PartialEq + 'static;

    fn preview(s: &Self::Source) -> Option<Self::Focus>;
    fn review(a: Self::Focus) -> Self::Source;

    fn law_review_preview(a: Self::Focus) -> bool {
        let s = Self::review(a.clone());
        Self::preview(&s) == Some(a)
    }

    fn law_preview_review(s: &Self::Source) -> bool {
        match Self::preview(s) {
            None    => true,
            Some(a) => Self::review(a) == *s,
        }
    }
}

pub struct JStringPrism;

impl LawfulPrism for JStringPrism {
    type Source = Json;
    type Focus  = String;
    fn preview(j: &Json) -> Option<String> {
        match j { Json::JString(s) => Some(s.clone()), _ => None }
    }
    fn review(s: String) -> Json { Json::JString(s) }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Prism type | `('s, 'a) prism` record | `Prism<S, A>` struct |
| preview | `'s -> 'a option` | `Fn(&S) -> Option<A>` |
| review | `'a -> 's` | `Fn(A) -> S` |
| Law 1 checker | `'a -> bool` | `fn(prism, A) -> bool` where `A: PartialEq + Clone` |
| Law 2 checker | `'s -> bool` | `fn(prism, &S) -> bool` where `S: PartialEq` |
| Zero-cost variant | N/A (modules-as-functors) | `trait LawfulPrism` with associated types |

## Key Insights

1. **Laws are not enforced by the type system — only by tests.**  Both OCaml and Rust let you build a `Prism` from any two functions.  The laws are semantic contracts you must verify explicitly with `check_review_preview` / `check_preview_review`; the compiler cannot check them for you.

2. **The vacuous case matters.**  Law 2 ("PreviewReview") only fires when `preview` returns `Some`.  Returning `true` on `None` is correct: the law says *if preview succeeds*, then round-trip holds.  This asymmetry mirrors OCaml's pattern match directly.

3. **Rust borrows, OCaml copies.**  OCaml's `preview : 's -> 'a option` takes ownership via value semantics.  Rust's `Fn(&S) -> Option<A>` borrows `S`, which avoids cloning the outer type on every call — important when `S` is an enum with heap-allocated variants like `Json`.

4. **`'static` bounds on boxed closures are a Rust artifact.**  The `Box<dyn Fn(...)>` approach requires `'static` because the closure may outlive the scope where it was created.  The trait-based approach (`LawfulPrism`) sidesteps this entirely: law checkers are inherent methods, so no lifetime constraint is needed on the prism itself.

5. **Unlawful prisms expose why laws matter at scale.**  An unlawful prism (`unlawful_uppercase_prism`) passes most unit tests that only check one direction.  Only the round-trip law check (`check_review_preview`) catches that `review("hello") → JString("hello")` and `preview(JString("hello")) → Some("HELLO")` disagree.  In a composed pipeline of ten prisms, this silent corruption is nearly impossible to debug without law tests.

## When to Use Each Style

**Use closure-based `Prism<S, A>` when:** you need to build prisms at runtime (e.g., from configuration, user input, or a registry of optics).  The boxing overhead is negligible compared to flexibility.

**Use trait-based `LawfulPrism` when:** all prisms are known at compile time and you want zero-cost dispatch plus the law-check methods baked into the type.  Each prism is a zero-sized marker struct; the compiler monomorphises every call.
