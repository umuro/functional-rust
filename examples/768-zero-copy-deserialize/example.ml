(* Zero-copy style in OCaml — using substrings that share the underlying buffer
   Note: OCaml strings are immutable; Bytes.sub_string still copies.
   We simulate zero-copy with offset+length pairs. *)

type string_view = { src: string; off: int; len: int }

let view_to_string sv = String.sub sv.src sv.off sv.len

(* A "zero-copy" record — fields are views into the input buffer *)
type person_view = {
  name: string_view;
  age_str: string_view;  (* raw string, parse lazily *)
}

(* Parse "name=Alice|age=30" without copying field contents *)
let parse_view (s: string) : person_view option =
  let find_char s start c =
    let rec go i = if i >= String.length s then None
                   else if s.[i] = c then Some i
                   else go (i+1)
    in go start
  in
  (* find first '|' *)
  match find_char s 0 '|' with
  | None -> None
  | Some pipe ->
    (* name field: "name=Alice" → from 5 to pipe *)
    let name_off = 5 in  (* skip "name=" *)
    let name_len = pipe - name_off in
    (* age field: skip "age=" after pipe *)
    let age_off  = pipe + 1 + 4 in  (* skip "age=" *)
    let age_len  = String.length s - age_off in
    if name_len <= 0 || age_len <= 0 then None
    else Some {
      name    = { src = s; off = name_off; len = name_len };
      age_str = { src = s; off = age_off;  len = age_len  };
    }

let () =
  let input = "name=Alice|age=30" in
  match parse_view input with
  | None -> Printf.printf "parse failed\n"
  | Some pv ->
    Printf.printf "Name (view): %s\n" (view_to_string pv.name);
    Printf.printf "Age  (view): %s\n" (view_to_string pv.age_str);
    Printf.printf "Age  (int) : %d\n"
      (int_of_string (view_to_string pv.age_str))
