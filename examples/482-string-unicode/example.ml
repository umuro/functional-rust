(* 482: String Unicode — normalization, ASCII-only ops, emoji in OCaml *)
(* Key point: OCaml strings are bytes. NFC vs NFD are different byte sequences.
   Unicode normalization requires an external library (e.g., uunf).
   Here we demonstrate ASCII-safe operations and show the byte/codepoint distinction. *)

(* Case-insensitive comparison for ASCII strings only *)
let eq_ignore_ascii_case s1 s2 =
  let n = String.length s1 in
  if n <> String.length s2 then false
  else
    let rec loop i =
      if i = n then true
      else
        let lower c =
          if c >= 'A' && c <= 'Z' then Char.chr (Char.code c + 32) else c
        in
        if lower s1.[i] <> lower s2.[i] then false
        else loop (i + 1)
    in
    loop 0

(* Check if a string contains only ASCII bytes (all < 128) *)
let is_ascii s =
  let len = String.length s in
  let rec loop i =
    if i = len then true
    else if Char.code s.[i] >= 128 then false
    else loop (i + 1)
  in
  loop 0

(* Count Unicode codepoints (minimal UTF-8 decoder — codepoint count only) *)
let char_count s =
  let len = String.length s in
  let count = ref 0 in
  let i = ref 0 in
  while !i < len do
    let b = Char.code s.[!i] in
    let advance =
      if b land 0x80 = 0 then 1
      else if b land 0xE0 = 0xC0 then 2
      else if b land 0xF0 = 0xE0 then 3
      else 4
    in
    incr count;
    i := !i + advance
  done;
  !count

let () =
  (* NFC vs NFD: in OCaml, these are simply different byte sequences.
     "café" composed (U+00E9 = \xC3\xA9) vs decomposed (e + combining accent \xCC\x81) *)
  let nfc = "caf\xC3\xA9" in         (* U+00E9: composed é *)
  let nfd = "cafe\xCC\x81" in        (* U+0065 + U+0301: decomposed *)
  assert (nfc <> nfd);               (* different byte sequences *)
  Printf.printf "NFC <> NFD (different bytes): ok\n";
  Printf.printf "  NFC bytes=%d  NFD bytes=%d\n"
    (String.length nfc) (String.length nfd);

  (* ASCII case-insensitive comparison *)
  assert (eq_ignore_ascii_case "hello" "HELLO");
  assert (not (eq_ignore_ascii_case "hello" "world"));
  print_endline "eq_ignore_ascii_case: ok";

  (* is_ascii *)
  assert (is_ascii "hello");
  assert (not (is_ascii "caf\xC3\xA9"));
  print_endline "is_ascii: ok";

  (* Emoji: U+1F600 = \xF0\x9F\x98\x80 — 4 bytes, 1 codepoint *)
  let emoji = "\xF0\x9F\x98\x80" in
  assert (String.length emoji = 4);
  assert (char_count emoji = 1);
  Printf.printf "emoji bytes=%d  codepoints=%d\n"
    (String.length emoji) (char_count emoji);

  print_endline "All assertions passed."
