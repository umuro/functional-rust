(* 483: String encoding — UTF-8 encoding patterns in OCaml *)
(* OCaml strings are UTF-8 (by convention). We show encoding/validation explicitly. *)

(* Encode a Unicode codepoint to a UTF-8 string *)
let encode_utf8 cp =
  let buf = Buffer.create 4 in
  if cp < 0x80 then
    Buffer.add_char buf (Char.chr cp)
  else if cp < 0x800 then begin
    Buffer.add_char buf (Char.chr (0xC0 lor (cp lsr 6)));
    Buffer.add_char buf (Char.chr (0x80 lor (cp land 0x3F)))
  end else if cp < 0x10000 then begin
    Buffer.add_char buf (Char.chr (0xE0 lor (cp lsr 12)));
    Buffer.add_char buf (Char.chr (0x80 lor ((cp lsr 6) land 0x3F)));
    Buffer.add_char buf (Char.chr (0x80 lor (cp land 0x3F)))
  end else begin
    Buffer.add_char buf (Char.chr (0xF0 lor (cp lsr 18)));
    Buffer.add_char buf (Char.chr (0x80 lor ((cp lsr 12) land 0x3F)));
    Buffer.add_char buf (Char.chr (0x80 lor ((cp lsr 6)  land 0x3F)));
    Buffer.add_char buf (Char.chr (0x80 lor (cp land 0x3F)))
  end;
  Buffer.contents buf

(* UTF-8 byte length of a codepoint — mirrors Rust's char::len_utf8() *)
let len_utf8 cp =
  if cp < 0x80 then 1
  else if cp < 0x800 then 2
  else if cp < 0x10000 then 3
  else 4

(* Validate a string as UTF-8 *)
let validate_utf8 s =
  let len = String.length s in
  let i = ref 0 in
  try
    while !i < len do
      let b = Char.code s.[!i] in
      let rest =
        if b land 0x80 = 0 then 0
        else if b land 0xE0 = 0xC0 then 1
        else if b land 0xF0 = 0xE0 then 2
        else if b land 0xF8 = 0xF0 then 3
        else failwith "invalid"
      in
      for j = 1 to rest do
        if !i + j >= len then failwith "truncated";
        if Char.code s.[!i + j] land 0xC0 <> 0x80 then failwith "bad continuation"
      done;
      i := !i + 1 + rest
    done;
    Ok ()
  with Failure msg -> Error msg

(* Strip a UTF-8 BOM (U+FEFF = \xEF\xBB\xBF) if present *)
let strip_bom s =
  let bom = "\xEF\xBB\xBF" in
  let bom_len = String.length bom in
  if String.length s >= bom_len && String.sub s 0 bom_len = bom then
    Some (String.sub s bom_len (String.length s - bom_len))
  else
    None

let () =
  (* Encode 'A' (0x41) → "A" *)
  let a_enc = encode_utf8 0x41 in
  assert (a_enc = "A");
  Printf.printf "encode U+0041 = %s\n" a_enc;

  (* len_utf8 of é (U+00E9) = 2 *)
  assert (len_utf8 0xE9 = 2);
  Printf.printf "len_utf8(é) = %d\n" (len_utf8 0xE9);

  (* validate valid UTF-8 *)
  assert (validate_utf8 "hi" = Ok ());
  assert (Result.is_error (validate_utf8 "\xFF"));
  print_endline "validate_utf8: ok";

  (* BOM stripping *)
  let with_bom = "\xEF\xBB\xBFhi" in
  assert (strip_bom with_bom = Some "hi");
  assert (strip_bom "hi" = None);
  print_endline "strip_bom: ok";

  print_endline "All assertions passed."
