# Comparison: Display Trait

## Simple Types

**OCaml:**
```ocaml
type color = Red | Green | Blue

let color_to_string = function
  | Red -> "Red" | Green -> "Green" | Blue -> "Blue"

let () = Printf.printf "%s\n" (color_to_string Red)
```

**Rust:**
```rust
enum Color { Red, Green, Blue }

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Blue => write!(f, "Blue"),
        }
    }
}

println!("{}", Color::Red);  // Display enables format!
```

## Recursive Types

**OCaml:**
```ocaml
let rec tree_to_string to_s = function
  | Leaf -> "."
  | Node (l, v, r) ->
    Printf.sprintf "(%s %s %s)" (tree_to_string to_s l) (to_s v) (tree_to_string to_s r)
```

**Rust:**
```rust
impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tree::Leaf => write!(f, "."),
            Tree::Node(l, v, r) => write!(f, "({} {} {})", l, v, r),
        }
    }
}
```

## to_string

**OCaml:** Manual function, no standard trait
```ocaml
let point_to_string p = Printf.sprintf "(%.2f, %.2f)" p.x p.y
```

**Rust:** Automatic via Display
```rust
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}
let s = point.to_string();  // Free!
```
