(* Function-like macro concepts in OCaml *)

(* PPX-based custom syntax for SQL-like DSL *)

(* Simulate sql! macro *)
let validate_sql sql =
  (* Simple validation *)
  let required = ["SELECT"; "FROM"] in
  let upper = String.uppercase_ascii sql in
  List.for_all (fun kw -> try
    let _ = Str.search_forward (Str.regexp_string kw) upper 0 in true
    with Not_found -> false) required

(* Simulate html! macro *)
let make_element tag content =
  Printf.sprintf "<%s>%s</%s>" tag content tag

let () =
  let sql = "SELECT id, name FROM users WHERE active = true" in
  if validate_sql sql then
    Printf.printf "Valid SQL: %s\n" sql
  else
    Printf.printf "Invalid SQL!\n";

  let html = make_element "div"
    (make_element "h1" "Hello" ^
     make_element "p" "World") in
  Printf.printf "%s\n" html
