(* Flatten Nested List *)
(* OCaml 99 Problems #7 *)

(* Define nested list type *)
type 'a node =
  | One of 'a
  | Many of 'a node list

(* Flatten recursively *)
let flatten lst =
  let rec aux acc = function
    | [] -> acc
    | One x :: t -> aux (x :: acc) t
    | Many xs :: t -> aux (aux acc xs) t
  in
  List.rev (aux [] lst)

(* Tests *)
let () =
  assert (flatten [One 1; Many [One 2; Many [One 3; One 4]]; One 5] = [1; 2; 3; 4; 5]);
  assert (flatten [] = []);
  assert (flatten [One 1] = [1]);
  print_endline "✓ OCaml tests passed"
