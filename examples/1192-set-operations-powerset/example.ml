(* Set Operations — Powerset *)
(* Compute the powerset of a set *)

module IntSet = Set.Make(Int)

let powerset s =
  IntSet.fold (fun x acc ->
    List.fold_left (fun acc2 subset ->
      IntSet.add x subset :: acc2
    ) acc acc
  ) s [IntSet.empty]

let s = IntSet.of_list [1; 2; 3]
let ps = powerset s
let () = List.iter (fun sub ->
  Printf.printf "{%s} "
    (IntSet.elements sub |> List.map string_of_int |> String.concat ",")
) ps
