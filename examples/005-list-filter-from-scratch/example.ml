(* Filter removes elements that don't satisfy the predicate *)
let rec filter p = function
  | []     -> []
  | h :: t ->
    let t' = filter p t in
    if p h then h :: t' else t'

(* Specialised predicates *)
let is_even x = x mod 2 = 0
let is_odd x = x mod 2 <> 0
let is_positive x = x > 0

let () =
  let nums = [-2; -1; 0; 1; 2; 3; 4] in
  List.iter (Printf.printf "%d ") (filter is_even nums); print_newline ();
  List.iter (Printf.printf "%d ") (filter is_positive nums); print_newline ()