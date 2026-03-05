(* Timsort concept in OCaml *)
(* OCaml's List.sort uses merge sort - similar principles *)

let () =
  let arr = [5; 2; 8; 1; 9; 3] in
  let sorted = List.sort compare arr in
  Printf.printf "Sorted: [%s]\n"
    (String.concat "; " (List.map string_of_int sorted))
