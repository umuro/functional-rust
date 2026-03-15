(* 498: Safe Unicode truncation — truncate at byte or codepoint boundary *)
(* OCaml strings are byte arrays; we must not cut in the middle of a multi-byte
   UTF-8 sequence when truncating. *)

(* Byte-level truncation: find the nearest valid UTF-8 boundary at or before max_bytes *)
let truncate_bytes s max_bytes =
  let len = String.length s in
  if len <= max_bytes then s
  else begin
    (* Walk back from max_bytes until we're at the start of a UTF-8 sequence.
       Continuation bytes have the form 10xxxxxx (0x80..0xBF). *)
    let pos = ref max_bytes in
    while !pos > 0 && (Char.code s.[!pos] land 0xC0 = 0x80) do
      decr pos
    done;
    String.sub s 0 !pos
  end

(* Count UTF-8 codepoints efficiently using the leading-byte rule *)
let utf8_char_count s =
  let len = String.length s in
  let count = ref 0 in
  for i = 0 to len - 1 do
    (* Count only non-continuation bytes: start bytes are 0xxxxxxx or 11xxxxxx *)
    if Char.code s.[i] land 0xC0 <> 0x80 then incr count
  done;
  !count

(* Codepoint-level truncation: keep at most max_chars codepoints *)
let truncate_chars s max_chars =
  let len = String.length s in
  let count = ref 0 in
  let pos = ref 0 in
  while !pos < len && !count < max_chars do
    let b = Char.code s.[!pos] in
    let width =
      if b land 0x80 = 0 then 1
      else if b land 0xE0 = 0xC0 then 2
      else if b land 0xF0 = 0xE0 then 3
      else 4
    in
    pos := !pos + width;
    incr count
  done;
  if !pos > len then s   (* shorter than max_chars *)
  else if !count = max_chars then String.sub s 0 !pos
  else s

(* Truncate with ellipsis — inserts "…" (U+2026, 3 UTF-8 bytes) *)
let ellipsis = "\xe2\x80\xa6"  (* UTF-8 encoding of … *)

let truncate_with_ellipsis s max_chars =
  if utf8_char_count s <= max_chars then s
  else
    let truncated = truncate_chars s (max_chars - 1) in
    truncated ^ ellipsis

let () =
  (* truncate_bytes *)
  assert (truncate_bytes "hello" 3 = "hel");
  (* "café" = c a f \xC3\xA9; truncate at 3 should not cut into é *)
  let cafe = "caf\xC3\xA9" in
  assert (truncate_bytes cafe 3 = "caf");
  print_endline "truncate_bytes: ok";

  (* truncate_chars *)
  assert (truncate_chars cafe 3 = "caf");
  assert (truncate_chars "hello" 10 = "hello");
  print_endline "truncate_chars: ok";

  (* truncate_with_ellipsis *)
  let r = truncate_with_ellipsis "hello world" 8 in
  assert (String.length r > 0);
  Printf.printf "truncate_with_ellipsis: %s\n" r;

  assert (truncate_with_ellipsis "hi" 10 = "hi");

  (* emoji: 🌍🌎🌏 — 3 codepoints, 4 bytes each *)
  let emoji = "\xF0\x9F\x8C\x8D\xF0\x9F\x8C\x8E\xF0\x9F\x8C\x8F" in
  let t = truncate_chars emoji 2 in
  assert (utf8_char_count t = 2);
  print_endline "emoji truncation: ok";

  print_endline "All assertions passed."
