(* include_str! concept in OCaml *)
(* OCaml doesn't have compile-time file inclusion; use read_file *)

let read_file path =
  let ic = open_in path in
  let n = in_channel_length ic in
  let s = Bytes.create n in
  really_input ic s 0 n;
  close_in ic;
  Bytes.to_string s

(* Simulate include_str! with a constant *)
let sample_text =
  "Hello, World!\n   This is a multi-line string.\n   Embedded at compile time."

let version_from_file = "1.0.0"  (* would be include("VERSION") *)

let () =
  Printf.printf "Embedded text:\n%s\n" sample_text;
  Printf.printf "Version: %s\n" version_from_file;
  Printf.printf "Text length: %d bytes\n" (String.length sample_text)
