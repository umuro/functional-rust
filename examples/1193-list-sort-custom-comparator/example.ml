(* Idiomatic OCaml: List.sort with various comparison functions *)
let words = ["banana"; "apple"; "cherry"; "date"]
let sorted = List.sort String.compare words
let by_length = List.sort (fun a b -> compare (String.length a) (String.length b)) words
let descending = List.sort (fun a b -> String.compare b a) words

let () =
  assert (sorted = ["apple"; "banana"; "cherry"; "date"]);
  assert (by_length = ["date"; "apple"; "banana"; "cherry"]);
  assert (descending = ["date"; "cherry"; "banana"; "apple"]);
  assert (List.sort String.compare [] = []);
  print_endline "ok"
