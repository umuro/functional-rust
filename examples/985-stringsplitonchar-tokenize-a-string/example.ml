(* String.split_on_char — Tokenize a String *)
(* Split a string into parts by a delimiter character *)

let csv_line = "Alice,30,Engineer,Amsterdam"
let fields = String.split_on_char ',' csv_line
let () = List.iteri (fun i f -> Printf.printf "Field %d: %s\n" i f) fields

let words = String.split_on_char ' ' "  hello   world  "
let nonempty = List.filter (fun s -> s <> "") words
