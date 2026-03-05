(* Recursive List Functions *)
(* Write recursive functions over lists *)

let rec length = function
  | [] -> 0
  | _ :: tl -> 1 + length tl

let rec append l1 l2 = match l1 with
  | [] -> l2
  | hd :: tl -> hd :: append tl l2

let rec rev_acc acc = function
  | [] -> acc
  | hd :: tl -> rev_acc (hd :: acc) tl
let rev = rev_acc []

let () = Printf.printf "length: %d\n" (length [1;2;3;4;5])
let () = List.iter (fun x -> Printf.printf "%d " x) (rev [1;2;3])
