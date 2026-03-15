(* 258. Index-value pairs with enumerate() - OCaml *)

let () =
  let fruits = ["apple"; "banana"; "cherry"] in
  List.iteri (fun i fruit ->
    Printf.printf "%d: %s\n" i fruit
  ) fruits;

  let evens_only = List.filteri (fun i _ -> i mod 2 = 0) fruits in
  Printf.printf "Even indices: %s\n" (String.concat ", " evens_only);

  let indexed_names = List.mapi (fun i name ->
    Printf.sprintf "#%d %s" (i + 1) name
  ) fruits in
  List.iter print_endline indexed_names
