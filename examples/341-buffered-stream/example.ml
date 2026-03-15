(* 341: Buffered Stream *)
(* OCaml channels are already buffered by default *)

(* Approach 1: Buffered input *)
let read_lines_buffered ic =
  let rec aux acc =
    match input_line ic with
    | line -> aux (line :: acc)
    | exception End_of_file -> List.rev acc
  in
  aux []

(* Approach 2: Buffered output *)
let write_lines_buffered oc lines =
  List.iter (fun line ->
    output_string oc line;
    output_char oc '\n'
  ) lines;
  flush oc

(* Approach 3: Buffer module for string building *)
let build_string parts =
  let buf = Buffer.create 256 in
  List.iter (fun s ->
    Buffer.add_string buf s;
    Buffer.add_char buf '\n'
  ) parts;
  Buffer.contents buf

(* Tests *)
let () =
  let result = build_string ["hello"; "world"; "test"] in
  assert (result = "hello\nworld\ntest\n");
  let buf = Buffer.create 16 in
  Buffer.add_string buf "abc";
  Buffer.add_string buf "def";
  assert (Buffer.contents buf = "abcdef");
  Printf.printf "✓ All tests passed\n"
