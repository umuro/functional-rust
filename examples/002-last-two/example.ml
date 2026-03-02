(* Find the last two elements of a list *)

let rec last_two = function
  | [] | [_] -> None
  | [x; y] -> Some (x, y)
  | _ :: t -> last_two t

(* Tests *)
let () =
  assert (last_two [] = None);
  assert (last_two [1] = None);
  assert (last_two [1; 2] = Some (1, 2));
  assert (last_two [1; 2; 3; 4] = Some (3, 4));
  print_endline "✓ OCaml tests passed"
