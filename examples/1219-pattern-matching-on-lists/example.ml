(* Sum all elements in a list *)
let rec sum lst =
  match lst with
  | [] -> 0
  | hd :: tl -> hd + sum tl

(* Check if a list contains a value *)
let rec contains x lst =
  match lst with
  | [] -> false
  | hd :: tl -> hd = x || contains x tl

let () =
  Printf.printf "sum [1;2;3] = %d\n" (sum [1;2;3]);      (* 6 *)
  Printf.printf "contains 2 [1;2;3] = %b\n" (contains 2 [1;2;3]);  (* true *)