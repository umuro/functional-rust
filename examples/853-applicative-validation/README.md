📖 **[View on hightechmind.io →](https://hightechmind.io/rust/853-applicative-validation)**

---

# Applicative Validation

## Problem Statement

`Result` short-circuits on the first error — useful for computations where later steps depend on earlier ones, but poor for user input validation where you want to report all errors at once. If a signup form has invalid email AND weak password, showing only the first error forces users to resubmit multiple times. The `Validated` type accumulates all errors instead of short-circuiting. This is the applicative approach: both validations run independently and their errors are combined. In contrast to monadic `and_then` which chains dependent operations, applicative validation runs all checks in parallel and collects all failures. This pattern is used in form validation libraries, configuration parsers, and data pipeline error reporting.

## Learning Outcomes

- Implement `Validated<T, E>` with `Valid(T)` and `Invalid(Vec<E>)` variants
- Implement `apply` that combines two `Validated` values: both must be `Valid` to succeed; errors accumulate
- Contrast with `Result::and_then` which short-circuits at the first `Err`
- Apply to form validation: validate name, email, and age independently, report all errors
- Recognize the semigroup constraint: errors must be combinable (`Vec<E>` is a natural semigroup)

## Rust Application

```rust
#[derive(Debug, PartialEq, Clone)]
enum Validated<T, E> { Valid(T), Invalid(Vec<E>) }

impl<T, E> Validated<T, E> {
    pub fn apply<U, F: FnOnce(T) -> U>(self, vf: Validated<F, E>) -> Validated<U, E> {
        match (vf, self) {
            (Validated::Valid(f), Validated::Valid(x)) => Validated::Valid(f(x)),
            (Validated::Invalid(e1), Validated::Invalid(e2)) => {
                let mut errors = e1; errors.extend(e2); Validated::Invalid(errors)
            },
            (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
        }
    }
}
```

The match on `(vf, self)` handles all four combinations: both valid (apply function), both invalid (merge errors), one invalid (propagate errors). The `errors.extend(e2)` merges error vectors without cloning individual errors. The pattern `| (_, Validated::Invalid(e))` handles the asymmetric cases. This accumulation behavior — combining errors with `extend` — requires `E` to be `Clone` or requires consuming both values (handled here by consuming both via `match`).

## OCaml Approach

OCaml defines `type ('a, 'e) validated = Valid of 'a | Invalid of 'e list`. The `apply` function: `let apply vf vx = match vf, vx with Valid f, Valid x -> Valid (f x) | Invalid e1, Invalid e2 -> Invalid (e1 @ e2) | Invalid e, _ | _, Invalid e -> Invalid e`. The `@` operator appends lists. OCaml's `List.concat` merges multiple error lists. Form validation: `validate_name name |> apply (validate_email email) |> apply (validate_age age)` runs all validations and combines errors. The `Alcotest` library uses similar validation for test result accumulation.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Error accumulation | `Vec::extend` | `List.(@)` append |
| Both invalid | `extend` merges vectors | `@` concatenates lists |
| Apply signature | `.apply(vf: Validated<F,E>)` | `let apply vf vx` |
| Form validation | `validate().apply(validate()).apply(...)` | `va \|> apply vb \|> apply vc` |
| Error type | `Vec<E>` | `'e list` |
| vs. Result | Short-circuits | This accumulates |

## Exercises

1. Implement form validation for a user signup struct with name, email, password, and age fields — report all errors.
2. Implement `validate_all(validations: Vec<Validated<T, E>>) -> Validated<Vec<T>, E>` using `apply`.
3. Show that `Validated::apply` satisfies the applicative identity law: `Valid(|x| x).apply(vx) == vx`.
4. Implement a parser combinator using `Validated` that runs multiple field parsers and collects all parse errors.
5. Compare the user experience: validate a form with `Result` (first-error-only) and `Validated` (all-errors) and show the difference in error output.
