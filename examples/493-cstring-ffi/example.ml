(* 493. CString for FFI – OCaml *)
(* OCaml uses Ctypes for FFI; strings need null termination *)
let () =
  (* In OCaml FFI, strings to C functions need null termination *)
  (* OCaml strings are NOT null-terminated by default *)
  let s = "hello" in
  (* Create null-terminated version: *)
  let cs = Bytes.create (String.length s + 1) in
  Bytes.blit_string s 0 cs 0 (String.length s);
  Bytes.set cs (String.length s) '\000';
  Printf.printf "c-string len=%d (including null)\n" (Bytes.length cs);

  (* Check for interior nulls *)
  let has_null s =
    String.exists ((=) '\000') s
  in
  Printf.printf "has_null 'hello': %b\n" (has_null s);
  Printf.printf "has_null 'hel\000lo': %b\n" (has_null "hel\000lo")
