(* Range-like matching in OCaml via guards *)
let grade n =
  match n with
  | n when n >= 90 -> 'A'
  | n when n >= 80 -> 'B'
  | n when n >= 70 -> 'C'
  | n when n >= 60 -> 'D'
  | _              -> 'F'

let classify_char c =
  match Char.code c with
  | n when n >= 65 && n <= 90  -> "upper"
  | n when n >= 97 && n <= 122 -> "lower"
  | n when n >= 48 && n <= 57  -> "digit"
  | _                          -> "other"

let () =
  List.iter (fun n -> Printf.printf "%d->%c " n (grade n)) [95;82;74;61;45];
  print_newline ();
  List.iter (fun c -> Printf.printf "%c:%s " c (classify_char c)) ['A';'z';'5';'!']
