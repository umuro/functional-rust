(* OCaml: String is UTF-8 bytes + length; C strings are null-terminated. *)

(** Simulate a C-style null-terminated string as a Bytes. *)
let to_c_string (s : string) : bytes =
  let n = String.length s in
  let b = Bytes.create (n + 1) in
  Bytes.blit_string s 0 b 0 n;
  Bytes.set b n '\000';
  b

(** Read until null terminator — simulate C strlen. *)
let c_strlen (b : bytes) : int =
  let rec go i =
    if i >= Bytes.length b || Bytes.get b i = '\000' then i else go (i + 1)
  in go 0

(** Convert C string (bytes) back to OCaml string. *)
let from_c_string (b : bytes) : string =
  Bytes.sub_string b 0 (c_strlen b)

let () =
  let s = "Hello, FFI!" in
  let cs = to_c_string s in
  Printf.printf "Original:        '%s' (len=%d)\n" s (String.length s);
  Printf.printf "C string strlen: %d\n" (c_strlen cs);
  Printf.printf "C->OCaml:        '%s'\n" (from_c_string cs)
