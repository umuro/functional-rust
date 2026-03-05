# Comparison: Generic Bounds

## Single Bound

**OCaml — Implicit polymorphism:**
```ocaml
let find_max lst =
  match lst with
  | [] -> None
  | x :: rest -> Some (List.fold_left max x rest)
```

**Rust — Explicit trait bound:**
```rust
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().reduce(|a, b| if a >= b { a } else { b })
}
```

## Multiple Bounds

**OCaml — Structural (no syntax needed):**
```ocaml
let print_max lst =
  match find_max lst with
  | None -> "empty"
  | Some x -> Printf.sprintf "Max: %s" (string_of_int x)
```

**Rust — Combined with `+`:**
```rust
fn print_max<T: PartialOrd + Display>(slice: &[T]) -> Option<String> {
    find_max(slice).map(|v| format!("Max: {}", v))
}
```

## Trait Hierarchy

**OCaml — Module type inclusion:**
```ocaml
module type Printable = sig
  type t
  val to_string : t -> string
end
```

**Rust — Supertrait:**
```rust
trait Summarize: Display {
    fn summary(&self) -> String;
}
```
