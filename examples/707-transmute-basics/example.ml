(* OCaml: Obj.magic is the transmute equivalent — equally dangerous.
   Always prefer typed conversions. *)

(** Float to bits — idiomatic safe way. *)
let float_to_bits (f : float) : int64 = Int64.bits_of_float f
let bits_to_float (b : int64) : float = Int64.float_of_bits b

(** int32 byte view via Bytes — safe byte manipulation. *)
let int32_to_bytes_le (n : int32) : bytes =
  let b = Bytes.create 4 in
  Bytes.set_int32_le b 0 n;
  b

let () =
  let pi = Float.pi in
  Printf.printf "pi bits: 0x%Lx\n" (float_to_bits pi);
  Printf.printf "round-trip: %f (equal: %b)\n"
    (bits_to_float (float_to_bits pi))
    (pi = bits_to_float (float_to_bits pi));
  let bytes = int32_to_bytes_le 0x12345678l in
  print_string "int32 LE bytes: ";
  Bytes.iter (fun c -> Printf.printf "%02x " (Char.code c)) bytes;
  print_newline ()
