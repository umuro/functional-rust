# Comparison: Iterator Adapters

## Map/Filter Pipeline

**OCaml:**
```ocaml
let pipeline data =
  data
  |> List.filter (fun x -> x > 0)
  |> List.map (fun x -> x * x)
  |> List.map string_of_int
```

**Rust:**
```rust
fn pipeline(data: &[i32]) -> Vec<String> {
    data.iter()
        .filter(|&&x| x > 0)
        .map(|&x| x * x)
        .map(|x| x.to_string())
        .collect()
}
```

## Flat Map

**OCaml:**
```ocaml
let flat_map_example data =
  data
  |> List.map (fun s -> String.split_on_char ' ' s)
  |> List.flatten
```

**Rust:**
```rust
fn flat_map_example(data: &[&str]) -> Vec<String> {
    data.iter()
        .flat_map(|s| s.split_whitespace())
        .map(String::from)
        .collect()
}
```

## Take/Skip

**OCaml (Seq):**
```ocaml
let result =
  data |> List.to_seq
  |> Seq.filter (fun x -> x mod 2 = 0)
  |> Seq.map (fun x -> x * 3)
  |> Seq.take 5
  |> List.of_seq
```

**Rust:**
```rust
data.iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * 3)
    .take(5)
    .collect::<Vec<_>>()
```
