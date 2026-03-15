(* OCaml: Memory layout inspection
   OCaml doesn't expose repr attributes, but we can observe sizes
   using Obj and compare with C layouts via ctypes (not used here — std only). *)

(* OCaml's Obj module lets us inspect the runtime layout *)

(* --- Field ordering and padding in OCaml --- *)
(* OCaml records are laid out in declaration order; each field is one word.
   There is no padding between fields (they're always word-sized). *)

type compact = { a: int; b: bool; c: int }
(* In OCaml: a=1 word, b=1 word (bool is boxed as int), c=1 word = 3 words *)

type c_like = { x: float; y: float; z: float }
(* float array is unboxed; float record fields are also unboxed *)

let () =
  (* Obj.size gives number of fields (words, not bytes) *)
  let v = { a = 1; b = true; c = 3 } in
  Printf.printf "compact: Obj.size=%d words\n" (Obj.size (Obj.repr v));

  let p = { x = 1.0; y = 2.0; z = 3.0 } in
  Printf.printf "c_like: Obj.size=%d words\n" (Obj.size (Obj.repr p));

  (* On 64-bit: 1 word = 8 bytes *)
  let word_size = Sys.int_size / 8 + 1 in
  Printf.printf "Word size: %d bytes\n" word_size

(* --- Simulated packed / aligned structs --- *)
(* OCaml has no packed or align attributes.
   For FFI, you'd use the `ctypes` library which has Ctypes.Struct. *)

(* ctypes conceptual example (not compiled — just illustration):
   open Ctypes
   let my_struct = structure "my_struct"
   let f1 = field my_struct "field1" uint8_t
   let f2 = field my_struct "field2" uint32_t
   let () = seal my_struct
   (* ctypes computes the right C layout with padding *)
*)

(* --- Byte-level layout via Bytes --- *)
(* The closest OCaml can get to packed repr is Bytes/Bigarray *)
let pack_u8_u32 (a : int) (b : int32) : bytes =
  let buf = Bytes.create 5 in
  Bytes.set buf 0 (Char.chr (a land 0xFF));
  (* Little-endian u32 *)
  Bytes.set buf 1 (Char.chr (Int32.to_int (Int32.logand b 0xFFl)));
  Bytes.set buf 2 (Char.chr (Int32.to_int (Int32.logand (Int32.shift_right_logical b 8) 0xFFl)));
  Bytes.set buf 3 (Char.chr (Int32.to_int (Int32.logand (Int32.shift_right_logical b 16) 0xFFl)));
  Bytes.set buf 4 (Char.chr (Int32.to_int (Int32.logand (Int32.shift_right_logical b 24) 0xFFl)));
  buf

let () =
  let packed = pack_u8_u32 0xAB 0x12345678l in
  Printf.printf "Packed bytes: %s\n"
    (String.concat " " (List.init (Bytes.length packed)
       (fun i -> Printf.sprintf "%02X" (Char.code (Bytes.get packed i)))))
