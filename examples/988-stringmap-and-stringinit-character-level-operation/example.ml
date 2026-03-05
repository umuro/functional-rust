(* String.map and String.init — Character-level Operations *)
(* Transform strings character by character *)

let rot13 c =
  if c >= 'a' && c <= 'z' then Char.chr ((Char.code c - Char.code 'a' + 13) mod 26 + Char.code 'a')
  else if c >= 'A' && c <= 'Z' then Char.chr ((Char.code c - Char.code 'A' + 13) mod 26 + Char.code 'A')
  else c

let encoded = String.map rot13 "Hello World"
let decoded = String.map rot13 encoded
let () = Printf.printf "%s -> %s\n" encoded decoded

let alphabet = String.init 26 (fun i -> Char.chr (i + Char.code 'a'))
