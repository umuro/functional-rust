(* Idiomatic OCaml: List.filter keeps elements satisfying a predicate *)
let numbers = [1; 2; 3; 4; 5; 6; 7; 8]
let evens = List.filter (fun x -> x mod 2 = 0) numbers
let odds = List.filter (fun x -> x mod 2 <> 0) numbers

(* Recursive OCaml: explicit pattern matching on list spine *)
let rec filter_rec pred = function
  | [] -> []
  | x :: rest ->
    if pred x then x :: filter_rec pred rest
    else filter_rec pred rest

let () =
  assert (evens = [2; 4; 6; 8]);
  assert (odds = [1; 3; 5; 7]);
  assert (List.filter (fun _ -> false) [1;2;3] = []);
  assert (List.filter (fun _ -> true) [1;2;3] = [1;2;3]);
  assert (filter_rec (fun x -> x > 3) [1;2;3;4;5] = [4;5]);
  print_endline "ok"
