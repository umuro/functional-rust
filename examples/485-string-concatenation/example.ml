(* 485: String concatenation — idiomatic ways to join strings in OCaml *)

(* ^ operator: simple two-string concatenation (creates new string) *)
let with_caret () =
  let a = "hi" in
  let b = "!" in
  let s = a ^ b in
  assert (s = "hi!");
  Printf.printf "^: %s\n" s

(* String.concat: join a list with a separator — like slice.join() in Rust *)
let with_concat () =
  let s = String.concat "-" ["a"; "b"; "c"] in
  assert (s = "a-b-c");
  Printf.printf "concat: %s\n" s

(* Printf.sprintf: format-based concatenation — like format!() in Rust *)
let with_sprintf () =
  let s = Printf.sprintf "%d-%d" 1 2 in
  assert (s = "1-2");
  Printf.printf "sprintf: %s\n" s

(* Buffer: efficient incremental building — avoids O(n²) from repeated ^ *)
let with_buffer () =
  let parts = ["a"; "b"; "c"] in
  let buf = Buffer.create 16 in
  List.iter (Buffer.add_string buf) parts;
  let s = Buffer.contents buf in
  assert (s = "abc");
  Printf.printf "Buffer: %s\n" s

(* String.concat with empty separator — join without delimiter *)
let join_no_sep () =
  let s = String.concat "" ["a"; "b"; "c"] in
  assert (s = "abc");
  Printf.printf "join empty sep: %s\n" s

let () =
  with_caret ();
  with_concat ();
  with_sprintf ();
  with_buffer ();
  join_no_sep ();
  print_endline "All assertions passed."
