(* 341: Buffered Stream — efficient I/O with buffering.
   OCaml: channels are buffered by default. Buffer module for in-memory buffering.
   Scanf / Printf handle formatted I/O; input_line reads line-by-line efficiently. *)

(* Approach 1: Count lines in a string using Scanf *)
let count_lines s =
  let n = ref 0 in
  String.iter (fun c -> if c = '\n' then incr n) s;
  !n

(* Read all lines from a string (simulates BufReader::lines) *)
let read_lines s =
  (* Split on newline, drop trailing empty if string ended with \n *)
  let lines = String.split_on_char '\n' s in
  (* Remove the final empty string that split_on_char adds after trailing \n *)
  match List.rev lines with
  | "" :: rest -> List.rev rest
  | _          -> lines

(* Approach 2: Buffer-based writing (mirrors BufWriter) *)
let write_lines lines =
  let buf = Buffer.create 256 in
  List.iter (fun line ->
    Buffer.add_string buf line;
    Buffer.add_char buf '\n'
  ) lines;
  Buffer.contents buf

(* Approach 3: Build CSV using Buffer *)
let build_csv headers rows =
  let buf = Buffer.create 256 in
  Buffer.add_string buf (String.concat "," headers);
  Buffer.add_char buf '\n';
  List.iter (fun row ->
    Buffer.add_string buf (String.concat "," row);
    Buffer.add_char buf '\n'
  ) rows;
  Buffer.contents buf

(* Approach 4: Read lines from a file channel with buffering *)
let read_file_lines ic =
  let lines = ref [] in
  (try while true do
    lines := input_line ic :: !lines
  done with End_of_file -> ());
  List.rev !lines

(* ── Demo ─────────────────────────────────────────────────────────────────── *)

let () =
  (* Count lines *)
  Printf.printf "line count \"a\\nb\\nc\\n\" = %d\n" (count_lines "a\nb\nc\n");
  Printf.printf "line count \"\" = %d\n" (count_lines "");

  (* Read lines *)
  let lines = read_lines "hello\nworld\n" in
  Printf.printf "read_lines: [%s]\n" (String.concat ";" lines);

  (* Write lines *)
  let out = write_lines ["hello"; "world"] in
  Printf.printf "write_lines contains hello: %b\n"
    (String.length out > 0 &&
     match String.split_on_char '\n' out with
     | l :: _ -> l = "hello"
     | [] -> false);

  (* Build CSV *)
  let csv = build_csv ["a"; "b"] [["1"; "2"]; ["3"; "4"]] in
  Printf.printf "CSV starts with a,b: %b\n"
    (String.sub csv 0 3 = "a,b");
  Printf.printf "CSV:\n%s" csv;

  (* In-memory channel simulation using Buffer + Scanf.Scanning *)
  let content = "line one\nline two\nline three\n" in
  let scan = Scanf.Scanning.open_in_channel (Scanf.Scanning.stdin) in
  ignore scan;
  (* Simpler: use String.split_on_char as the OCaml "buffered reader" *)
  let parsed = read_lines content in
  Printf.printf "parsed %d lines from in-memory string\n" (List.length parsed)
