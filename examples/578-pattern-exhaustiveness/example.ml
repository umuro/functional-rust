(* OCaml: exhaustiveness warnings *)
type dir = N | S | E | W

let describe = function
  | N -> "north" | S -> "south" | E -> "east" | W -> "west"

let horizontal = function E | W -> true | _ -> false

(* Adding a new variant later would trigger exhaustiveness warnings *)
type code = OK | NotFound | Unauthorized | ServerError

let status_text = function
  | OK          -> "OK"
  | NotFound    -> "Not Found"
  | Unauthorized-> "Unauthorized"
  | ServerError -> "Internal Server Error"

let () =
  List.iter (fun d->Printf.printf "%s " (describe d)) [N;S;E;W]; print_newline ();
  Printf.printf "%b %b\n" (horizontal E) (horizontal N)
