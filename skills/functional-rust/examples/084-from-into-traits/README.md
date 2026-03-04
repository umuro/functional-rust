# 084: From/Into Traits

**Difficulty:** 2  **Level:** Intermediate

Implement `From<T>` to define infallible conversions between types — and get `Into<T>` for free. Use `TryFrom`/`TryInto` when conversion can fail.

## The Problem This Solves

Every codebase has conversions: Celsius to Fahrenheit, tuples to structs, raw strings to validated types. Without a standard vocabulary, everyone writes their own: `to_fahrenheit()`, `from_string()`, `as_point()` — different names, inconsistent call sites, nothing interoperable.

Rust's standard library defines the vocabulary: `From` and `Into` for infallible conversions, `TryFrom` and `TryInto` for fallible ones. Implement `From<Celsius> for Fahrenheit` and you get `Fahrenheit::from(celsius)` and `let f: Fahrenheit = celsius.into()` for free — and any library function that accepts `impl Into<Fahrenheit>` will accept your `Celsius` automatically.

This is the type conversion protocol the whole ecosystem understands.

## The Intuition

In Python, you'd write a `__init__` that accepts multiple types, or a `@classmethod` factory. In Java, you'd write a static `of()` factory method or a constructor. In OCaml, explicit conversion functions with consistent naming conventions. In Rust, `From`/`Into` are the standard interface — implement once, integrate everywhere.

The free `Into` impl is the key insight: Rust provides a blanket `impl<T, U: From<T>> Into<U> for T`. So implementing `From<Celsius> for Fahrenheit` automatically makes `Celsius: Into<Fahrenheit>`. You only ever need to implement `From`.

## How It Works in Rust

```rust
#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

// Implement From — Into comes free
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

let c = Celsius(100.0);
let f: Fahrenheit = c.into();          // Into<Fahrenheit> is free from From impl
let f2 = Fahrenheit::from(Celsius(0.0)); // explicit From call also works
```

```rust
// Into in generic functions — accepts any type that can become Celsius
fn print_temperature<T: Into<Celsius>>(temp: T) {
    let c: Celsius = temp.into();   // converts whatever T is into Celsius
    println!("Temperature: {:.1}°C", c.0);
}

print_temperature(Celsius(37.0));       // T = Celsius
print_temperature(Fahrenheit(98.6));    // T = Fahrenheit (via our From impl)
```

```rust
// TryFrom for fallible conversions
impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 { return Err("Expected x,y".into()); }
        let x = parts[0].trim().parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
        let y = parts[1].trim().parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok(Point { x, y })
    }
}

// TryInto comes free from TryFrom, just like Into from From
let p: Result<Point, _> = Point::try_from("3, 4");
```

```rust
// Bidirectional conversions between tuples and structs
impl From<(i32, i32)> for Point { fn from((x, y): (i32, i32)) -> Self { Point { x, y } } }
impl From<Point> for (i32, i32) { fn from(p: Point) -> Self { (p.x, p.y) } }

let p: Point = (3, 4).into();
let t: (i32, i32) = p.into();
```

## What This Unlocks

- **Flexible function signatures**: `fn process(data: impl Into<MyType>)` accepts any type with a defined conversion — callers choose the most convenient input form.
- **Error type conversions**: `?` operator uses `From` to convert between error types — implement `From<io::Error> for MyError` and `?` handles the conversion automatically.
- **Collection transformations**: `data.iter().copied().map(Point::from).collect::<Vec<Point>>()` converts a slice of tuples to a Vec of Points cleanly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Infallible conversion | Explicit function `celsius_to_fahrenheit` | `impl From<Celsius> for Fahrenheit` |
| Fallible conversion | Returns `option` or `result` type | `impl TryFrom<T>` returns `Result` |
| Free inverse | Must implement separately | `Into` automatically derived from `From` |
| Generic acceptance | Parametric polymorphism + modules | `impl Into<T>` bound in function signature |
| `?` operator support | No equivalent | Uses `From` for error type coercion |
