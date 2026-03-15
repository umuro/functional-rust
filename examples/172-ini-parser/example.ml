(* Example 172: INI File Parser *)
(* INI file parser: sections [name], key = value pairs *)

type 'a parse_result = ('a * string, string) result
type 'a parser = string -> 'a parse_result

type ini_value = string
type ini_section = { name: string; entries: (string * ini_value) list }
type ini_file = ini_section list

let ws0 input =
  let rec skip i = if i < String.length input &&
    (input.[i] = ' ' || input.[i] = '\t') then skip (i+1) else i in
  let i = skip 0 in String.sub input i (String.length input - i)

let skip_line input =
  match String.index_opt input '\n' with
  | Some i -> String.sub input (i+1) (String.length input - i - 1)
  | None -> ""

(* Approach 1: Parse section header [name] *)
let parse_section_header input =
  let s = ws0 input in
  if String.length s > 0 && s.[0] = '[' then
    match String.index_opt s ']' with
    | Some i ->
      let name = String.trim (String.sub s 1 (i - 1)) in
      let rest = String.sub s (i+1) (String.length s - i - 1) in
      Ok (name, skip_line rest)
    | None -> Error "Expected ']'"
  else Error "Expected '['"

(* Approach 2: Parse key = value *)
let parse_entry input =
  let s = ws0 input in
  if String.length s = 0 || s.[0] = '[' || s.[0] = '#' || s.[0] = ';' || s.[0] = '\n' then
    Error "Not a key=value entry"
  else
    match String.index_opt s '=' with
    | Some i ->
      let key = String.trim (String.sub s 0 i) in
      let rest_line = String.sub s (i+1) (String.length s - i - 1) in
      let value_end = match String.index_opt rest_line '\n' with
        | Some j -> j | None -> String.length rest_line in
      let value = String.trim (String.sub rest_line 0 value_end) in
      (* strip inline comments *)
      let value = match String.index_opt value '#' with
        | Some j -> String.trim (String.sub value 0 j) | None -> value in
      let remaining = if value_end < String.length rest_line then
        String.sub rest_line (value_end + 1) (String.length rest_line - value_end - 1)
      else "" in
      Ok ((key, value), remaining)
    | None -> Error "Expected '='"

(* Approach 3: Full INI parser *)
let skip_blank_and_comments input =
  let rec go s =
    let s = ws0 s in
    if String.length s = 0 then s
    else if s.[0] = '\n' then go (String.sub s 1 (String.length s - 1))
    else if s.[0] = '#' || s.[0] = ';' then go (skip_line s)
    else s
  in go input

let parse_ini input =
  let rec parse_sections acc remaining =
    let remaining = skip_blank_and_comments remaining in
    if String.length remaining = 0 then Ok (List.rev acc, "")
    else
      match parse_section_header remaining with
      | Ok (name, rest) ->
        let rec parse_entries eacc r =
          let r = skip_blank_and_comments r in
          match parse_entry r with
          | Ok (entry, rest) -> parse_entries (entry :: eacc) rest
          | Error _ -> (List.rev eacc, r)
        in
        let (entries, rest) = parse_entries [] rest in
        parse_sections ({ name; entries } :: acc) rest
      | Error e -> Error e
  in
  parse_sections [] input

(* Tests *)
let () =
  let ini_text = "[database]\nhost = localhost\nport = 5432\n\n[server]\nname = myapp # main\nlog = true\n" in
  (match parse_ini ini_text with
   | Ok (sections, "") ->
     assert (List.length sections = 2);
     let db = List.nth sections 0 in
     assert (db.name = "database");
     assert (List.assoc "host" db.entries = "localhost");
     assert (List.assoc "port" db.entries = "5432");
     let srv = List.nth sections 1 in
     assert (srv.name = "server");
     assert (List.assoc "name" srv.entries = "myapp");
   | _ -> failwith "INI parse failed");

  assert (parse_section_header "[test]" = Ok ("test", ""));

  print_endline "✓ All tests passed"
