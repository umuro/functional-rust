# Comparison: FlatMap/Bind Chains

## Long Chain with >>= vs and_then

**OCaml:**
```ocaml
let process_name json =
  parse_json json >>= fun j ->
  extract_field "name" j >>= fun name ->
  validate_length 1 50 name >>= fun valid ->
  to_uppercase valid
```

**Rust (and_then):**
```rust
fn process_name(json: &str) -> Option<String> {
    parse_json(json)
        .and_then(|j| extract_field("name", j))
        .and_then(|name| validate_length(1, 50, name))
        .map(|s| s.to_uppercase())
}
```

## Database Lookup Chain

**OCaml:**
```ocaml
find_user user_id >>= fun user ->
find_dept user.dept_id >>= fun dept ->
find_user dept.mgr_id >>= fun manager ->
Some (user.name ^ "'s manager is " ^ manager.name)
```

**Rust (? operator):**
```rust
fn find_manager(user_id: u32, users: &[User], depts: &[Dept]) -> Option<String> {
    let user = users.iter().find(|u| u.id == user_id)?;
    let dept = depts.iter().find(|d| d.id == user.dept_id)?;
    let mgr = users.iter().find(|u| u.id == dept.mgr_id)?;
    Some(format!("{}'s manager is {}", user.name, mgr.name))
}
```

## Bounded Computation

**OCaml:**
```ocaml
return_ 0 >>= step_add 10 >>= step_mul 3 >>= step_add 20
```

**Rust:**
```rust
Some(0)
    .and_then(|a| step_add(10, a))
    .and_then(|a| step_mul(3, a))
    .and_then(|a| step_add(20, a))
```
