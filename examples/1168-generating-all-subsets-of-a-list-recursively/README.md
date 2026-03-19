# Generating all subsets of a list recursively

**Source:** https://v2.ocaml.org/learn/tutorials/99problems.html

**Difficulty:** Intermediate

**Category:** Math/recursion

## OCaml

```ocaml
let rec powerset = function
  | [] -> [[]]
  | x :: rest ->
    let ps = powerset rest in
    ps @ List.map (fun s -> x :: s) ps

let () =
  let sets = powerset [1; 2; 3] in
  Printf.printf "%d subsets:\n" (List.length sets);
  List.iter (fun s ->
    Printf.printf "{%s}\n" (String.concat "," (List.map string_of_int s))
  ) sets
```

## Rust

*To be implemented.*
