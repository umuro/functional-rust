(* 483. UTF-8 encoding – OCaml *)
let encode_utf8 codepoint =
  if codepoint < 0x80 then
    Bytes.create 1 |> (fun b -> Bytes.set b 0 (Char.chr codepoint); b)
  else if codepoint < 0x800 then
    let b = Bytes.create 2 in
    Bytes.set b 0 (Char.chr (0xC0 lor (codepoint lsr 6)));
    Bytes.set b 1 (Char.chr (0x80 lor (codepoint land 0x3F)));
    b
  else
    let b = Bytes.create 3 in
    Bytes.set b 0 (Char.chr (0xE0 lor (codepoint lsr 12)));
    Bytes.set b 1 (Char.chr (0x80 lor ((codepoint lsr 6) land 0x3F)));
    Bytes.set b 2 (Char.chr (0x80 lor (codepoint land 0x3F)));
    b

let () =
  let e = encode_utf8 0xE9 in  (* é *)
  Printf.printf "é bytes: %02x %02x\n" (Char.code (Bytes.get e 0)) (Char.code (Bytes.get e 1));
  let s = "caf\xc3\xa9" in
  Printf.printf "valid UTF-8 test: byte_len=%d\n" (String.length s)
