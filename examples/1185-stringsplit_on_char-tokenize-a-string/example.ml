(* Idiomatic OCaml: String.split_on_char tokenizes a string *)
let csv_line = "Alice,30,Engineer,Amsterdam"
let fields = String.split_on_char ',' csv_line

(* Filter empty tokens arising from consecutive delimiters *)
let words = String.split_on_char ' ' "  hello   world  "
let nonempty = List.filter (fun s -> s <> "") words

(* Split first occurrence: use String.index + String.sub *)
let split_once delim s =
  match String.index_opt s delim with
  | None -> None
  | Some i ->
    let before = String.sub s 0 i in
    let after = String.sub s (i+1) (String.length s - i - 1) in
    Some (before, after)

let () =
  assert (fields = ["Alice"; "30"; "Engineer"; "Amsterdam"]);
  assert (List.length fields = 4);
  assert (nonempty = ["hello"; "world"]);
  assert (String.split_on_char ',' "" = [""]);
  assert (split_once '=' "key=value=extra" = Some ("key", "value=extra"));
  assert (split_once '=' "no-delimiter" = None);
  print_endline "ok"
