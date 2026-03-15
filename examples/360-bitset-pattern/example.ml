(* OCaml: bitset via int (up to 62 bits on 64-bit) *)

let empty = 0
let add s i = s lor (1 lsl i)
let remove s i = s land (lnot (1 lsl i))
let mem s i = (s lsr i) land 1 = 1
let union = (lor)
let inter = (land)
let diff a b = a land (lnot b)
let to_list s = List.init 62 (fun i -> if mem s i then [i] else []) |> List.concat

let () =
  let a = List.fold_left add empty [0;1;3;5;7] in
  let b = List.fold_left add empty [1;2;3;4;5] in
  Printf.printf "A: %s\n" (String.concat "," (List.map string_of_int (to_list a)));
  Printf.printf "A∩B: %s\n" (String.concat "," (List.map string_of_int (to_list (inter a b))))
