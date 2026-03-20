📖 **[View on hightechmind.io →](https://hightechmind.io/rust/473-string-parsing-fromstr)**

---

# String Parsing with FromStr
**Difficulty:** ⭐  
**Category:** Functional Programming  


The `FromStr` trait provides a uniform interface for parsing strings into typed values, powering the `.parse::<T>()` method used throughout the standard library and Serde.

## Problem Statement

Every program receives input as text — CLI arguments, config files, network packets. Converting that text into typed values safely requires consistent error handling and a standard interface. Rust's `FromStr` trait defines `fn from_str(s: &str) -> Result<Self, Self::Err>`, allowing any type to be constructed from a string with `"value".parse::<MyType>()`. This is the same mechanism behind `"42".parse::<i32>()`, `"127.0.0.1".parse::<IpAddr>()`, and `serde_json::from_str`.

## Learning Outcomes

- Implement `FromStr` for a custom type with a custom error type
- Use `.parse::<T>()` to trigger `FromStr` through type inference
- Define a `Display` error type that wraps the invalid input string
- Chain `?` operator errors through `map_err` for ergonomic parsing
- Understand the relationship between `FromStr`, `TryFrom<&str>`, and `serde::Deserialize`

## Rust Application

`Color` parses a `"r,g,b"` string into three `u8` fields:

```rust
impl FromStr for Color {
    type Err = ColorErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<&str> = s.split(',').collect();
        if p.len() != 3 { return Err(ColorErr(s.to_string())); }
        let u = |x: &str| x.trim().parse::<u8>().map_err(|_| ColorErr(s.to_string()));
        Ok(Color { r: u(p[0])?, g: u(p[1])?, b: u(p[2])? })
    }
}
```

The `?` operator on each `u(p[i])?` propagates a `ColorErr` if any component is out-of-range or non-numeric. Callers use `"10,20,30".parse::<Color>()`.

## OCaml Approach

OCaml has no built-in equivalent to `FromStr`. The idiomatic approach is a module-level `of_string` function returning `option` or `result`:

```ocaml
type color = { r: int; g: int; b: int }

let color_of_string s =
  match String.split_on_char ',' s with
  | [r; g; b] -> (
      try Some { r = int_of_string (String.trim r);
                 g = int_of_string (String.trim g);
                 b = int_of_string (String.trim b) }
      with Failure _ -> None)
  | _ -> None
```

The `ppx_sexp_conv` library generates `t_of_sexp` automatically for types annotated with `[@@deriving sexp]`, which serves a role similar to `serde::Deserialize`.

## Key Differences

1. **Trait vs. convention**: Rust's `FromStr` is a standardised trait enabling generic code (`T: FromStr`); OCaml uses a naming convention (`of_string`) without enforcement.
2. **Error type**: Rust's `type Err` is part of the trait contract, requiring a concrete error type; OCaml `of_string` often returns `option`, losing error detail.
3. **Operator integration**: Rust's `?` operator threads `FromStr::Err` through `Result` automatically; OCaml requires explicit `match` or `Result.bind`.
4. **Generic parsing**: Rust code can be generic over `T: FromStr` (e.g., `fn read_list<T: FromStr>(s: &str) -> Vec<T>`); OCaml achieves this only via functors.

## Exercises

1. **IP address parser**: Implement `FromStr` for a `struct Ipv4Addr { octets: [u8; 4] }` that parses `"192.168.1.1"`, returning a custom error for invalid octets.
2. **CSV row**: Implement `FromStr` for `struct CsvRow(Vec<String>)` that splits on commas and trims whitespace from each field.
3. **Round-trip**: Implement both `Display` and `FromStr` for `Color` and write a test that encodes a `Color` to a string and parses it back, asserting equality.
