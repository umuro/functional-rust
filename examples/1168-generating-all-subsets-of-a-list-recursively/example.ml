(* Idiomatic OCaml: generate all subsets recursively *)
let rec powerset = function
  | [] -> [[]]
  | x :: rest ->
    let ps = powerset rest in
    ps @ List.map (fun s -> x :: s) ps

let () =
  assert (powerset [] = [[]]);
  assert (List.length (powerset [1;2;3]) = 8);
  assert (List.length (powerset [1;2;3;4]) = 16);
  assert (List.mem [] (powerset [1;2;3]));
  assert (List.mem [1;2;3] (powerset [1;2;3]));
  print_endline "ok"
