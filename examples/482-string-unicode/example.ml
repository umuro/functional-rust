(* 482. Unicode – OCaml with Uutf *)
(* Requires: ocamlfind + uutf / uunf for full Unicode support *)
(* Using standard string for demonstration *)
let () =
  let s = "caf\xc3\xa9" in  (* café in UTF-8 bytes *)
  Printf.printf "byte_len=%d\n" (String.length s);

  (* Count Unicode code points manually for UTF-8 *)
  let count_codepoints s =
    let n = String.length s in
    let count = ref 0 and i = ref 0 in
    while !i < n do
      let b = Char.code s.[!i] in
      let len = if b land 0x80 = 0 then 1
                else if b land 0xE0 = 0xC0 then 2
                else if b land 0xF0 = 0xE0 then 3
                else 4 in
      incr count; i := !i + len
    done;
    !count
  in
  Printf.printf "codepoints=%d\n" (count_codepoints s);

  (* Check if valid UTF-8 by trying to iterate *)
  let is_valid s =
    let n = String.length s in
    let i = ref 0 and ok = ref true in
    while !i < n && !ok do
      let b = Char.code s.[!i] in
      let len = if b land 0x80=0 then 1 else if b land 0xE0=0xC0 then 2
                else if b land 0xF0=0xE0 then 3 else if b land 0xF8=0xF0 then 4
                else (ok:=false; 0) in
      i := !i + len
    done; !ok
  in
  Printf.printf "valid=%b\n" (is_valid s)
