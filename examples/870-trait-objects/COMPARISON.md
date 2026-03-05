# Comparison: Trait Objects

## Dynamic Dispatch

**OCaml — Object types:**
```ocaml
class type shape = object
  method area : float
  method name : string
end

class circle r = object
  method area = Float.pi *. r *. r
  method name = "Circle"
end

let total_area (shapes : shape list) =
  List.fold_left (fun acc s -> acc +. s#area) 0.0 shapes
```

**Rust — dyn Trait:**
```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}
```

## Enum-Based (ADT) Dispatch

**OCaml:**
```ocaml
type shape = Circle of float | Rectangle of float * float

let area = function
  | Circle r -> Float.pi *. r *. r
  | Rectangle (w, h) -> w *. h
```

**Rust:**
```rust
enum Shape { Circle(f64), Rectangle(f64, f64) }

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}
```

## Static Dispatch (Generics)

**OCaml — Functor/constraint:**
```ocaml
module type SHAPE = sig
  type t
  val area : t -> float
end
```

**Rust — Generic bounds:**
```rust
fn describe<S: Shape>(shape: &S) -> String {
    format!("{}: area={:.2}", shape.name(), shape.area())
}
```
