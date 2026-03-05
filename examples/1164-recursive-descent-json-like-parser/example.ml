(* Recursive Descent — JSON-like Parser *)
(* Parse a simplified JSON structure *)

type json =
  | JNull | JBool of bool | JNum of float
  | JStr of string | JList of json list

let rec json_to_string = function
  | JNull -> "null"
  | JBool b -> string_of_bool b
  | JNum n -> Printf.sprintf "%.0f" n
  | JStr s -> "\"" ^ s ^ "\""
  | JList lst ->
    "[" ^ String.concat ", " (List.map json_to_string lst) ^ "]"

let example = JList [JNum 1.0; JStr "hello"; JBool true; JNull;
                     JList [JNum 2.0; JNum 3.0]]
let () = Printf.printf "%s\n" (json_to_string example)
