(* OCaml: ordered sets via Set module *)
module IS = Set.Make(Int)

let () =
  let a = IS.of_list [1;3;5;7;9] in
  let b = IS.of_list [3;5;6;7;8] in
  IS.iter (Printf.printf "%d ") (IS.inter a b); print_newline ();
  IS.iter (Printf.printf "%d ") (IS.union a b); print_newline ();
  IS.iter (Printf.printf "%d ") (IS.diff a b); print_newline ()
