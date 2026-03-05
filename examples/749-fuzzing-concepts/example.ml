(* 749: Fuzzing Concepts — OCaml
   OCaml can be fuzzed with AFL (American Fuzzy Lop).
   Key principle: parsers should never raise exceptions on bad input. *)

(* A safe parser that returns Result instead of raising exceptions *)
type parse_error =
  | UnexpectedEnd
  | InvalidByte of char
  | TooLong of int

type packet = {
  version: int;
  payload_len: int;
  payload: bytes;
}

let parse_packet (data: bytes) : (packet, parse_error) result =
  let n = Bytes.length data in
  if n < 3 then Error UnexpectedEnd
  else
    let version = Char.code (Bytes.get data 0) in
    if version > 5 then Error (InvalidByte (Bytes.get data 0))
    else
      let payload_len = Char.code (Bytes.get data 1) in
      if payload_len > 255 then Error (TooLong payload_len)
      else if n < 2 + payload_len then Error UnexpectedEnd
      else
        let payload = Bytes.sub data 2 payload_len in
        Ok { version; payload_len; payload }

(* Fuzz-target-style function: accepts any bytes, never raises *)
let fuzz_target data =
  (match parse_packet data with
  | Ok p ->
    (* Invariant: payload length matches header *)
    assert (Bytes.length p.payload = p.payload_len)
  | Error _ ->
    (* Reject is fine — we just must not crash *)
    ())

(* Simulate fuzzing with some inputs *)
let () =
  let inputs = [
    Bytes.of_string "\x01\x05hello";  (* valid *)
    Bytes.of_string "";                (* too short *)
    Bytes.of_string "\x09\x01x";     (* invalid version *)
    Bytes.of_string "\x01\xFF";       (* truncated payload *)
  ] in
  List.iter fuzz_target inputs;
  Printf.printf "Fuzz targets: no panics!\n"
