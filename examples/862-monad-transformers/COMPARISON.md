# Comparison: Monad Transformers

## OptionT Bind

**OCaml:**
```ocaml
let bind m f = match m with
  | Error e -> Error e
  | Ok None -> Ok None
  | Ok (Some a) -> f a
```

**Rust:**
```rust
fn bind<A, B, E>(m: Result<Option<A>, E>, f: impl FnOnce(A) -> Result<Option<B>, E>) -> Result<Option<B>, E> {
    match m {
        Err(e) => Err(e),
        Ok(None) => Ok(None),
        Ok(Some(a)) => f(a),
    }
}
```

## Chained OptionT

**OCaml:**
```ocaml
find_user id >>= fun name -> find_email name
```

**Rust (transformer):**
```rust
option_t::bind(find_user(id), |name| find_email(&name))
```

**Rust (idiomatic with ? and early return):**
```rust
fn get_user_email(id: i32) -> Result<Option<String>, String> {
    let user = match find_user(id)? {
        Some(u) => u,
        None => return Ok(None),   // early exit for "not found"
    };
    find_email(&user)
}
```

## Lifting

**OCaml:**
```ocaml
let lift_result r = Result.map (fun x -> Some x) r
let lift_option o = Ok o
```

**Rust:**
```rust
fn lift_result<A, E>(r: Result<A, E>) -> Result<Option<A>, E> { r.map(Some) }
fn lift_option<A, E>(o: Option<A>) -> Result<Option<A>, E> { Ok(o) }
```
