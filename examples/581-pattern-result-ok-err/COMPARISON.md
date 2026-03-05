# OCaml vs Rust: Result Pattern Matching

## Basic Result Functions

### OCaml
```ocaml
let parse s = match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "not int: %s" s)

let validate n =
  if n >= 1 && n <= 100 then Ok n
  else Error (Printf.sprintf "%d out of range" n)
```

### Rust
```rust
fn parse(s: &str) -> Result<i32, MyError> {
    s.parse().map_err(|e| MyError::Parse(e.to_string()))
}

fn validate(n: i32) -> Result<i32, MyError> {
    if (1..=100).contains(&n) { Ok(n) }
    else { Err(MyError::Range(n)) }
}
```

## Chaining with ? Operator

### OCaml
```ocaml
let (let*) = Result.bind

let process s =
  let* n = parse s in
  let* v = validate n in
  Ok (v * v)
```

### Rust
```rust
fn process(s: &str) -> Result<i32, MyError> {
    let n = parse(s)?;
    let v = validate(n)?;
    Ok(v * v)
}
```

## Combinators Comparison

| Operation | OCaml | Rust |
|-----------|-------|------|
| **Map success** | `Result.map f r` | `r.map(f)` |
| **Map error** | `Result.map_error f r` | `r.map_err(f)` |
| **Chain** | `Result.bind r f` | `r.and_then(f)` |
| **Early return** | `let*` binding | `?` operator |
| **To Option** | `Result.to_option r` | `r.ok()` |
| **From Option** | `Option.to_result ~none:e` | `opt.ok_or(e)` |
| **Unwrap or** | `Result.value r ~default:x` | `r.unwrap_or(x)` |

## Collecting Results

### Rust
```rust
// Vec<Result<T, E>> -> Result<Vec<T>, E>
let results: Result<Vec<i32>, _> = 
    strings.iter().map(|s| parse(s)).collect();
```

### OCaml
```ocaml
(* No built-in; use recursion or fold *)
let sequence_results = 
  List.fold_right (fun r acc ->
    match r, acc with
    | Ok x, Ok xs -> Ok (x :: xs)
    | Error e, _ | _, Error e -> Error e
  ) results (Ok [])
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| **Error propagation** | `let*` binding operator | `?` operator |
| **Type** | `('a, 'e) result` | `Result<T, E>` |
| **Unwrap** | `Result.get_ok` (may raise) | `.unwrap()` (panics) |
| **Collect** | Manual | Built-in `collect()` |
| **Error trait** | None | `std::error::Error` trait |
