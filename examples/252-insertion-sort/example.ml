(* Idiomatic OCaml — insert x into a sorted list, then fold *)
let rec insert x = function
  | [] -> [x]
  | h :: t as l ->
    if x <= h then x :: l
    else h :: insert x t

let insertion_sort lst =
  List.fold_left (fun acc x -> insert x acc) [] lst

(* Recursive variant that makes the fold explicit *)
let rec insertion_sort_rec = function
  | [] -> []
  | x :: rest -> insert x (insertion_sort_rec rest)

let () =
  assert (insertion_sort [5; 3; 1; 4; 2] = [1; 2; 3; 4; 5]);
  assert (insertion_sort [] = []);
  assert (insertion_sort [1] = [1]);
  assert (insertion_sort [3; 1; 4; 1; 5] = [1; 1; 3; 4; 5]);
  assert (insertion_sort_rec [5; 3; 1; 4; 2] = [1; 2; 3; 4; 5]);
  print_endline "ok"
