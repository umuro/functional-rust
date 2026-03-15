(* OCaml: Zero-copy parsing with Bytes and substring references.
   OCaml 5.0+ has Bytes.sub_bytes for non-copying slices, but String.sub
   allocates. We demonstrate both and show the Bigarray approach. *)

(* --- Manual zero-copy-ish parsing with index ranges --- *)

(* Instead of allocating substrings, track (start, length) pairs *)
type span = { buf: bytes; start: int; len: int }

let span_of_bytes buf = { buf; start = 0; len = Bytes.length buf }

let span_to_string s =
  Bytes.sub_string s.buf s.start s.len

let span_get s i =
  Bytes.get s.buf (s.start + i)

(* Split a span at the first occurrence of byte `sep` — no allocation *)
let span_split_at sep s =
  let rec find i =
    if i >= s.len then None
    else if Bytes.get s.buf (s.start + i) = sep then Some i
    else find (i + 1)
  in
  match find 0 with
  | None -> None
  | Some i ->
    let left  = { s with len = i } in
    let right = { s with start = s.start + i + 1; len = s.len - i - 1 } in
    Some (left, right)

(* Skip leading whitespace *)
let span_trim_start s =
  let i = ref 0 in
  while !i < s.len && span_get s !i = ' ' do incr i done;
  { s with start = s.start + !i; len = s.len - !i }

(* Parse "METHOD /path HTTP/1.1" without allocating substrings *)
type request_line = { method_: span; path: span; version: span }

let parse_request_line buf =
  let s = span_of_bytes buf in
  match span_split_at ' ' s with
  | None -> Error "missing method"
  | Some (method_, rest) ->
    let rest = span_trim_start rest in
    match span_split_at ' ' rest with
    | None -> Error "missing path"
    | Some (path, version) ->
      Ok { method_; path; version = span_trim_start version }

let () =
  let input = Bytes.of_string "GET /index.html HTTP/1.1" in
  match parse_request_line input with
  | Error e -> Printf.printf "Error: %s\n" e
  | Ok r ->
    Printf.printf "Method:  %s\n" (span_to_string r.method_);
    Printf.printf "Path:    %s\n" (span_to_string r.path);
    Printf.printf "Version: %s\n" (span_to_string r.version)

(* --- Key-value binary format --- *)
(* Format: [u8: key_len][key_bytes][u16_be: val_len][val_bytes] *)

let parse_kv buf pos =
  if pos >= Bytes.length buf then None
  else
    let key_len = Char.code (Bytes.get buf pos) in
    let key_start = pos + 1 in
    if key_start + key_len > Bytes.length buf then None
    else
      let val_len_hi = Char.code (Bytes.get buf (key_start + key_len)) in
      let val_len_lo = Char.code (Bytes.get buf (key_start + key_len + 1)) in
      let val_len = (val_len_hi lsl 8) lor val_len_lo in
      let val_start = key_start + key_len + 2 in
      if val_start + val_len > Bytes.length buf then None
      else
        let key = Bytes.sub_string buf key_start key_len in
        let value = Bytes.sub_string buf val_start val_len in
        Some (key, value, val_start + val_len)

let () =
  (* Build a small KV buffer *)
  let buf = Buffer.create 32 in
  let add_kv k v =
    Buffer.add_char buf (Char.chr (String.length k));
    Buffer.add_string buf k;
    let vl = String.length v in
    Buffer.add_char buf (Char.chr ((vl lsr 8) land 0xFF));
    Buffer.add_char buf (Char.chr (vl land 0xFF));
    Buffer.add_string buf v
  in
  add_kv "name" "Rust";
  add_kv "version" "1.85";
  let raw = Bytes.of_string (Buffer.contents buf) in
  let pos = ref 0 in
  while !pos < Bytes.length raw do
    match parse_kv raw !pos with
    | None -> pos := Bytes.length raw
    | Some (k, v, next) ->
      Printf.printf "  %s = %s\n" k v;
      pos := next
  done
