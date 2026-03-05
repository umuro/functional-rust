(* OCaml: literal constants in patterns *)
let max_age = 65  (* Cannot use this in patterns directly *)

let classify n =
  match n with
  | 0    -> "zero"
  | n when n < 10  -> "small"
  | n when n = 100 -> "century"
  | n when n < 100 -> "medium"
  | _              -> "large"

let classify_char c =
  match c with
  | '\n' -> "newline" | '\t' -> "tab" | ' ' -> "space"
  | c when c>='a'&&c<='z' -> "lower" | _ -> "other"

let () =
  List.iter (fun n -> Printf.printf "%d:%s " n (classify n)) [0;5;50;100;200];
  print_newline ()
