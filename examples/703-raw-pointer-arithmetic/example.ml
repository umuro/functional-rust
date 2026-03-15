(* OCaml: pointer arithmetic is hidden; arrays use safe index operations.
   We simulate stride-based access with functional list building. *)

(** Read every stride-th element starting at start. *)
let strided_read (arr : 'a array) ~(start : int) ~(stride : int) : 'a list =
  let n = Array.length arr in
  let rec go i acc =
    if i >= n then List.rev acc
    else go (i + stride) (arr.(i) :: acc)
  in
  go start []

let () =
  let data = [| 0; 1; 2; 3; 4; 5; 6; 7; 8; 9 |] in
  let every_other = strided_read data ~start:0 ~stride:2 in
  print_string "Every other: ";
  List.iter (fun x -> Printf.printf "%d " x) every_other;
  print_newline ();
  let every_third = strided_read data ~start:0 ~stride:3 in
  print_string "Every third: ";
  List.iter (fun x -> Printf.printf "%d " x) every_third;
  print_newline ()
