(* OCaml: persistent list — natural! *)

let lst1 = [1;2;3;4;5]
let lst2 = 0 :: lst1  (* shares tail with lst1 *)
let lst3 = List.tl lst1  (* shares almost all of lst1 *)

let () =
  Printf.printf "lst1: [%s]\n" (String.concat ";" (List.map string_of_int lst1));
  Printf.printf "lst2: [%s]\n" (String.concat ";" (List.map string_of_int lst2));
  Printf.printf "lst3: [%s]\n" (String.concat ";" (List.map string_of_int lst3))
