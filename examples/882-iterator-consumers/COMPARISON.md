# Comparison: Iterator Consumers

## Fold

**OCaml:**
```ocaml
let sum lst = List.fold_left (+) 0 lst
let product lst = List.fold_left ( * ) 1 lst
```

**Rust:**
```rust
let sum: i32 = data.iter().sum();
let product: i32 = data.iter().product();
// Or explicit fold:
let sum = data.iter().fold(0, |acc, &x| acc + x);
```

## Find / Position

**OCaml:**
```ocaml
let find_first pred lst =
  try Some (List.find pred lst) with Not_found -> None

let rec find_position pred i = function
  | [] -> None
  | x :: _ when pred x -> Some i
  | _ :: rest -> find_position pred (i+1) rest
```

**Rust:**
```rust
data.iter().find(|&&x| x > 3)      // Option<&&i32>
data.iter().position(|&x| x > 3)   // Option<usize>
```

## Frequencies

**OCaml:**
```ocaml
let frequencies lst =
  let tbl = Hashtbl.create 16 in
  List.iter (fun x ->
    let c = try Hashtbl.find tbl x with Not_found -> 0 in
    Hashtbl.replace tbl x (c + 1)
  ) lst;
  Hashtbl.fold (fun k v acc -> (k, v) :: acc) tbl []
```

**Rust:**
```rust
fn frequencies(data: &[i32]) -> HashMap<i32, usize> {
    data.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    })
}
```
