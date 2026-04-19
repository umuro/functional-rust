(* Example 1218 — List.sort with a custom comparator. *)

let words = ["banana"; "apple"; "cherry"; "date"]

(* Idiomatic: String.compare is already a three-way comparator. *)
let by_alphabet = List.sort String.compare words

(* Custom comparator built from String.length. *)
let by_length =
  List.sort (fun a b -> compare (String.length a) (String.length b)) words

(* Descending alphabetical — swap the arguments to reverse the ordering. *)
let by_alpha_desc = List.sort (fun a b -> String.compare b a) words

let () =
  assert (by_alphabet = ["apple"; "banana"; "cherry"; "date"]);
  (* stable sort: "banana" and "cherry" tie on length 6 and keep input order *)
  assert (by_length = ["date"; "apple"; "banana"; "cherry"]);
  assert (by_alpha_desc = ["date"; "cherry"; "banana"; "apple"]);
  Printf.printf "Alphabetical: %s\n" (String.concat " " by_alphabet);
  Printf.printf "By length:    %s\n" (String.concat " " by_length);
  Printf.printf "Descending:   %s\n" (String.concat " " by_alpha_desc);
  print_endline "ok"
