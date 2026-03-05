(* Phantom Type State Machine — File Handle *)

type opened
type closed

type 'state handle = { name: string; content: string list }

let open_file name : opened handle =
  { name; content = ["line1"; "line2"; "line3"] }

let read_line (h : opened handle) n : string =
  List.nth h.content n

let close_file (_ : opened handle) : closed handle =
  { name = "closed"; content = [] }

(* read_line on a closed handle would be a type error! *)
(* let _ = read_line (close_file (open_file "test")) 0 *)

let () =
  let f = open_file "data.txt" in
  assert (read_line f 0 = "line1");
  assert (read_line f 1 = "line2");
  let _closed = close_file f in
  Printf.printf "%s\n" (read_line f 0);
  Printf.printf "%s\n" (read_line f 1);
  Printf.printf "File safely closed\n";
  print_endline "ok"
