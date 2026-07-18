(* Range *)
(* OCaml 99 Problems #22 *)

let range a b =
  let rec aux acc n = if n < a then acc else aux (n :: acc) (n - 1) in
  aux [] b

(* Tests *)
let () =
  assert (range 4 9 = [4; 5; 6; 7; 8; 9]);
  assert (range 5 5 = [5]);
  assert (range 5 3 = []);
  assert (range (-2) 2 = [-2; -1; 0; 1; 2]);
  print_endline "✓ OCaml tests passed"
