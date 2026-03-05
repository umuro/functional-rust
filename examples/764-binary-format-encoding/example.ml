(* Binary format: length-prefixed records in OCaml *)

(* ── Encoder ────────────────────────────────────────────────────────────────── *)
let encode_u32 n =
  let b = Bytes.create 4 in
  Bytes.set_uint8 b 0 ((n lsr 24) land 0xFF);
  Bytes.set_uint8 b 1 ((n lsr 16) land 0xFF);
  Bytes.set_uint8 b 2 ((n lsr  8) land 0xFF);
  Bytes.set_uint8 b 3 ( n         land 0xFF);
  b

let encode_string s =
  let len_bytes = encode_u32 (String.length s) in
  Bytes.cat len_bytes (Bytes.of_string s)

(* Record: tag(1 byte) + string name + u32 age + bool active *)
type person = { name: string; age: int; active: bool }

let encode p =
  let buf = Buffer.create 64 in
  Buffer.add_bytes buf (encode_string p.name);
  Buffer.add_bytes buf (encode_u32 p.age);
  Buffer.add_uint8 buf (if p.active then 1 else 0);
  Buffer.to_bytes buf

(* ── Decoder ────────────────────────────────────────────────────────────────── *)
let decode_u32 bytes pos =
  let b0 = Bytes.get_uint8 bytes  pos    in
  let b1 = Bytes.get_uint8 bytes (pos+1) in
  let b2 = Bytes.get_uint8 bytes (pos+2) in
  let b3 = Bytes.get_uint8 bytes (pos+3) in
  (b0 lsl 24) lor (b1 lsl 16) lor (b2 lsl 8) lor b3, pos + 4

let decode_string bytes pos =
  let len, pos = decode_u32 bytes pos in
  (Bytes.sub_string bytes pos len), pos + len

let decode bytes =
  let pos = ref 0 in
  let name, p = decode_string bytes !pos in pos := p;
  let age,  p = decode_u32    bytes !pos in pos := p;
  let active  = Bytes.get_uint8 bytes !pos = 1 in
  ignore pos;
  { name; age; active }

let hex_dump bytes =
  Bytes.to_seq bytes
  |> Seq.map (fun b -> Printf.sprintf "%02X" (Char.code b))
  |> List.of_seq
  |> String.concat " "

let () =
  let alice = { name = "Alice"; age = 30; active = true } in
  let encoded = encode alice in
  Printf.printf "Encoded (%d bytes): %s\n" (Bytes.length encoded) (hex_dump encoded);
  let decoded = decode encoded in
  Printf.printf "Decoded: name=%s age=%d active=%b\n"
    decoded.name decoded.age decoded.active
