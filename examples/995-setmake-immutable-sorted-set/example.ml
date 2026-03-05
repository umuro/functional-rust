(* Set.Make — Immutable Sorted Set *)
(* Create and use functional sets *)

module IntSet = Set.Make(Int)

let s1 = IntSet.of_list [1; 3; 5; 7; 9]
let s2 = IntSet.of_list [2; 3; 5; 7; 11]

let union = IntSet.union s1 s2
let inter = IntSet.inter s1 s2
let diff = IntSet.diff s1 s2

let print_set s =
  IntSet.elements s |> List.map string_of_int |> String.concat ", "
  |> Printf.printf "{%s}\n"

let () = print_set union; print_set inter; print_set diff
