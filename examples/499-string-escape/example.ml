(* 499: Escaping and unescaping strings — HTML and control characters *)

(* HTML escaping: & < > " ' → entities *)
let escape_html s =
  let buf = Buffer.create (String.length s) in
  String.iter (fun c -> match c with
    | '<'  -> Buffer.add_string buf "&lt;"
    | '>'  -> Buffer.add_string buf "&gt;"
    | '&'  -> Buffer.add_string buf "&amp;"
    | '"'  -> Buffer.add_string buf "&quot;"
    | '\'' -> Buffer.add_string buf "&#39;"
    | c    -> Buffer.add_char buf c
  ) s;
  Buffer.contents buf

(* HTML unescaping: entities → characters.
   Simple replacement chain; for full HTML5 entities use a library. *)
let unescape_html s =
  let s = Str.global_replace (Str.regexp_string "&lt;")   "<"  s in
  let s = Str.global_replace (Str.regexp_string "&gt;")   ">"  s in
  let s = Str.global_replace (Str.regexp_string "&amp;")  "&"  s in
  let s = Str.global_replace (Str.regexp_string "&quot;") "\"" s in
  let s = Str.global_replace (Str.regexp_string "&#39;")  "'"  s in
  s

(* Control character escaping: \n \t \r \\ \" → escape sequences *)
let escape_control s =
  let buf = Buffer.create (String.length s * 2) in
  String.iter (fun c -> match c with
    | '\n' -> Buffer.add_string buf "\\n"
    | '\t' -> Buffer.add_string buf "\\t"
    | '\r' -> Buffer.add_string buf "\\r"
    | '\\' -> Buffer.add_string buf "\\\\"
    | '"'  -> Buffer.add_string buf "\\\""
    | c    -> Buffer.add_char buf c
  ) s;
  Buffer.contents buf

(* Unescape control sequences: recognize \\n \\t \\r \\\\ \\" *)
let unescape_control s =
  let buf = Buffer.create (String.length s) in
  let len = String.length s in
  let i = ref 0 in
  while !i < len do
    if s.[!i] = '\\' && !i + 1 < len then begin
      (match s.[!i + 1] with
      | 'n'  -> Buffer.add_char buf '\n'
      | 't'  -> Buffer.add_char buf '\t'
      | 'r'  -> Buffer.add_char buf '\r'
      | '\\' -> Buffer.add_char buf '\\'
      | '"'  -> Buffer.add_char buf '"'
      | c    -> Buffer.add_char buf '\\'; Buffer.add_char buf c);
      i := !i + 2
    end else begin
      Buffer.add_char buf s.[!i];
      i := !i + 1
    end
  done;
  Buffer.contents buf

(* Note: the Str library is used for unescape_html; if not available,
   a Buffer-based replacement loop (like in escape_html) works equally well. *)

let () =
  (* HTML escape *)
  assert (escape_html "<b>hi</b>" = "&lt;b&gt;hi&lt;/b&gt;");
  print_endline "escape_html: ok";

  (* HTML unescape *)
  assert (unescape_html "&lt;b&gt;" = "<b>");
  print_endline "unescape_html: ok";

  (* HTML roundtrip *)
  let s = "<div>&amp;</div>" in
  assert (unescape_html (escape_html s) = s);
  print_endline "html roundtrip: ok";

  (* Control escape *)
  assert (escape_control "a\nb" = "a\\nb");
  print_endline "escape_control: ok";

  (* Control unescape *)
  assert (unescape_control "a\\nb" = "a\nb");
  print_endline "unescape_control: ok";

  print_endline "All assertions passed."
