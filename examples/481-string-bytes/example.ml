(* 481: String bytes — byte-level operations in OCaml *)
(* OCaml strings ARE byte arrays, so byte operations are direct *)

(* Collect bytes as a list of ints — mirrors Rust bytes().collect::<Vec<_>>() *)
let string_to_bytes s =
  let n = String.length s in
  Array.init n (fun i -> Char.code s.[i])

(* Build a string from a byte array — mirrors String::from_utf8 *)
let string_from_bytes arr =
  let n = Array.length arr in
  let b = Bytes.create n in
  Array.iteri (fun i v -> Bytes.set b i (Char.chr v)) arr;
  Bytes.to_string b

(* Validate UTF-8 — OCaml strings hold arbitrary bytes; check validity manually *)
let is_valid_utf8 s =
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
        else raise Exit
      in
      for j = 1 to rest do
        if !i + j >= len then raise Exit;
        if Char.code s.[!i + j] land 0xC0 <> 0x80 then raise Exit
      done;
      i := !i + 1 + rest
    done;
    true
  with Exit -> false

(* Lossy decode: replace invalid bytes with the Unicode replacement character U+FFFD *)
let from_utf8_lossy s =
  if is_valid_utf8 s then s
  else begin
    let buf = Buffer.create (String.length s) in
    let len = String.length s in
    let i = ref 0 in
    while !i < len do
      let b = Char.code s.[!i] in
      if b land 0x80 = 0 then begin
        Buffer.add_char buf (Char.chr b);
        incr i
      end else begin
        (* emit replacement char U+FFFD = \xEF\xBF\xBD *)
        Buffer.add_string buf "\xEF\xBF\xBD";
        incr i
      end
    done;
    Buffer.contents buf
  end

let () =
  (* byte values of "hi" *)
  let bs = string_to_bytes "hi" in
  assert (bs = [| 104; 105 |]);
  Printf.printf "bytes: [%d; %d]\n" bs.(0) bs.(1);

  (* round-trip from bytes *)
  let s = string_from_bytes [| 104; 105 |] in
  assert (s = "hi");
  Printf.printf "from_bytes: %s\n" s;

  (* invalid UTF-8 *)
  let invalid = "\xFF" in
  assert (not (is_valid_utf8 invalid));
  Printf.printf "is_valid_utf8(\\xFF) = false: ok\n";

  (* lossy: contains 'h' *)
  let lossy = from_utf8_lossy (String.concat "" ["h"; "\xFF"; "i"]) in
  assert (String.contains lossy 'h');
  Printf.printf "lossy: %s\n" lossy;

  print_endline "All assertions passed."
