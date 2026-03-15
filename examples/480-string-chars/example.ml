(* 480: String chars — character-level operations in OCaml *)
(* OCaml 5 note: OCaml strings are bytes; for Unicode codepoints use Uchar / utf decode *)

(* Decode a UTF-8 string into a list of Unicode code points (Uchar.t).
   We use the Buffer module's UTF-8 decoder via Bytes for correctness. *)
let utf8_codepoints s =
  (* Walk the string decoding UTF-8 sequences manually *)
  let len = String.length s in
  let result = ref [] in
  let i = ref 0 in
  while !i < len do
    let b = Char.code s.[!i] in
    let (cp, advance) =
      if b land 0x80 = 0 then (b, 1)
      else if b land 0xE0 = 0xC0 then
        let b2 = Char.code s.[!i + 1] in
        ((b land 0x1F) lsl 6 lor (b2 land 0x3F), 2)
      else if b land 0xF0 = 0xE0 then
        let b2 = Char.code s.[!i + 1] in
        let b3 = Char.code s.[!i + 2] in
        ((b land 0x0F) lsl 12 lor (b2 land 0x3F) lsl 6 lor (b3 land 0x3F), 3)
      else
        let b2 = Char.code s.[!i + 1] in
        let b3 = Char.code s.[!i + 2] in
        let b4 = Char.code s.[!i + 3] in
        ((b land 0x07) lsl 18 lor (b2 land 0x3F) lsl 12
          lor (b3 land 0x3F) lsl 6 lor (b4 land 0x3F), 4)
    in
    result := Uchar.of_int cp :: !result;
    i := !i + advance
  done;
  List.rev !result

(* Count Unicode codepoints — analogous to Rust's str::chars().count() *)
let char_count s = List.length (utf8_codepoints s)

(* Filter by codepoint predicate, return new UTF-8 string *)
let filter_chars pred s =
  let cps = utf8_codepoints s in
  let buf = Buffer.create (String.length s) in
  List.iter (fun u ->
    if pred u then begin
      (* encode Uchar back to UTF-8 *)
      let cp = Uchar.to_int u in
      if cp < 0x80 then Buffer.add_char buf (Char.chr cp)
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
      end
    end
  ) cps;
  Buffer.contents buf

(* Reverse a string at codepoint granularity *)
let rev_chars s =
  let cps = List.rev (utf8_codepoints s) in
  let buf = Buffer.create (String.length s) in
  let encode_uchar buf u =
    let cp = Uchar.to_int u in
    if cp < 0x80 then Buffer.add_char buf (Char.chr cp)
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
    end
  in
  List.iter (encode_uchar buf) cps;
  Buffer.contents buf

(* nth codepoint *)
let nth_char s n =
  match List.nth_opt (utf8_codepoints s) n with
  | Some u -> Some (Uchar.to_int u)
  | None   -> None

let () =
  (* "café" has 4 codepoints but 5 bytes (é = 2 bytes) *)
  let caf = "caf\xC3\xA9" in (* café in UTF-8 *)
  assert (char_count caf = 4);
  assert (String.length caf = 5);
  Printf.printf "char_count(café)=%d  byte_len=%d\n" (char_count caf) (String.length caf);

  (* filter digits *)
  let digits = filter_chars
    (fun u -> let c = Uchar.to_int u in c >= 0x30 && c <= 0x39)
    "Hello123"
  in
  assert (digits = "123");
  Printf.printf "filter digits: %s\n" digits;

  (* reverse *)
  let rev = rev_chars "abcde" in
  assert (rev = "edcba");
  Printf.printf "rev: %s\n" rev;

  (* nth *)
  assert (nth_char "hello" 1 = Some (Char.code 'e'));
  print_endline "All assertions passed."
