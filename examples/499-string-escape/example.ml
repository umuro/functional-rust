(* 499. String escaping – OCaml *)
let escape_html s =
  let buf = Buffer.create (String.length s) in
  String.iter (fun c -> match c with
    | '<' -> Buffer.add_string buf "&lt;"
    | '>' -> Buffer.add_string buf "&gt;"
    | '&' -> Buffer.add_string buf "&amp;"
    | '"' -> Buffer.add_string buf "&quot;"
    | '\'' -> Buffer.add_string buf "&#39;"
    | c   -> Buffer.add_char buf c
  ) s;
  Buffer.contents buf

let escape_backslash s =
  let buf = Buffer.create (String.length s) in
  String.iter (fun c -> match c with
    | '\n' -> Buffer.add_string buf "\\n"
    | '\t' -> Buffer.add_string buf "\\t"
    | '\\' -> Buffer.add_string buf "\\\\"
    | c   -> Buffer.add_char buf c
  ) s;
  Buffer.contents buf

let () =
  let html = "<div class=\"hello\">Hello & World!</div>" in
  Printf.printf "%s\n" (escape_html html);
  let raw = "line1\nline2\ttab\\slash" in
  Printf.printf "%s\n" (escape_backslash raw)
