# Comparison: Applicative Validation

## Validated Type

**OCaml:**
```ocaml
type ('a, 'e) validated =
  | Valid of 'a
  | Invalid of 'e list
```

**Rust:**
```rust
enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),
}
```

## Error-Accumulating Apply

**OCaml:**
```ocaml
let apply vf vx = match vf, vx with
  | Valid f, Valid x -> Valid (f x)
  | Invalid e1, Invalid e2 -> Invalid (e1 @ e2)  (* accumulate! *)
  | Invalid e, _ | _, Invalid e -> Invalid e
```

**Rust:**
```rust
fn apply<A, B, E, F: FnOnce(A) -> B>(vf: Validated<F, E>, va: Validated<A, E>) -> Validated<B, E> {
    match (vf, va) {
        (Validated::Valid(f), Validated::Valid(a)) => Validated::Valid(f(a)),
        (Validated::Invalid(mut e1), Validated::Invalid(e2)) => {
            e1.extend(e2);  // accumulate!
            Validated::Invalid(e1)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
    }
}
```

## Validating a Record

**OCaml:**
```ocaml
let validate_user name age email =
  pure make_user <*> validate_name name <*> validate_age age <*> validate_email email
(* All three validations run independently *)
```

**Rust:**
```rust
fn validate_user(name: &str, age: i32, email: &str) -> Validated<User, String> {
    lift3(
        |name, age, email| User { name, age, email },
        validate_name(name),
        validate_age(age),
        validate_email(email),
    )
}
```
