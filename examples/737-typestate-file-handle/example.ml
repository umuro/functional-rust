(* 737: File handle typestate — Open / Closed / ReadOnly permissions in OCaml *)
(* Rust encodes file permissions as phantom type parameters so that
   write operations are unavailable on read-only handles at compile time.
   OCaml achieves the same using phantom types. *)

(* ── Permission markers ──────────────────────────────────────────────────── *)
type closed    = private Closed_
type read_write = private ReadWrite_
type read_only  = private ReadOnly_

(* ── File handle record ──────────────────────────────────────────────────── *)
(* In this example the "file" is an in-memory buffer, matching the Rust version. *)
type 'mode file_handle = {
  path    : string;
  content : bytes;   (* mutable in OCaml via Bytes ref *)
  mutable pos : int;
}

(* ── Closed operations ───────────────────────────────────────────────────── *)

let new_handle path : closed file_handle =
  { path; content = Bytes.empty; pos = 0 }

(* Open for read-write — transitions Closed → ReadWrite *)
let open_rw (h : closed file_handle) : (read_write file_handle, string) result =
  Printf.printf "Opening '%s' read-write\n" h.path;
  Ok { h with content = Bytes.create 0; pos = 0 }

(* Open as read-only with initial content — transitions Closed → ReadOnly *)
let open_ro initial (h : closed file_handle) : (read_only file_handle, string) result =
  Printf.printf "Opening '%s' read-only\n" h.path;
  Ok { h with content = Bytes.of_string initial; pos = 0 }

(* ── ReadWrite operations ───────────────────────────────────────────────── *)

let write_all data (h : read_write file_handle) : read_write file_handle =
  let new_content = Bytes.cat h.content (Bytes.of_string data) in
  Printf.printf "Wrote %d bytes to '%s'\n" (String.length data) h.path;
  { h with content = new_content }

let read_to_string_rw (h : read_write file_handle) : string * read_write file_handle =
  let s = Bytes.sub_string h.content h.pos (Bytes.length h.content - h.pos) in
  let h' = { h with pos = Bytes.length h.content } in
  (s, h')

(* Downgrade to read-only — write operations become unavailable *)
let into_readonly (h : read_write file_handle) : read_only file_handle =
  Printf.printf "Downgrading '%s' to read-only\n" h.path;
  { h with pos = h.pos }

(* Close — transitions ReadWrite → Closed *)
let close_rw (h : read_write file_handle) : closed file_handle =
  Printf.printf "Closing '%s'\n" h.path;
  { h with content = Bytes.empty; pos = 0 }

(* ── ReadOnly operations ─────────────────────────────────────────────────── *)

let read_to_string_ro (h : read_only file_handle) : string * read_only file_handle =
  let s = Bytes.sub_string h.content h.pos (Bytes.length h.content - h.pos) in
  let h' = { h with pos = Bytes.length h.content } in
  (s, h')

let close_ro (h : read_only file_handle) : closed file_handle =
  Printf.printf "Closing '%s' (read-only)\n" h.path;
  { h with content = Bytes.empty; pos = 0 }

let () =
  (* write then read *)
  let h  = new_handle "test.txt" in
  let rw = Result.get_ok (open_rw h) in
  let rw = write_all "hello world" rw in
  let (s, rw) = read_to_string_rw rw in
  assert (s = "hello world");
  ignore (close_rw rw);
  print_endline "write_then_read: ok";

  (* downgrade to read-only *)
  let h2  = new_handle "test.txt" in
  let rw2 = Result.get_ok (open_rw h2) in
  let rw2 = write_all "data" rw2 in
  let ro  = into_readonly rw2 in
  let (s2, ro) = read_to_string_ro ro in
  assert (s2 = "data");
  ignore (close_ro ro);
  print_endline "downgrade_to_readonly: ok";

  (* open_ro with initial content *)
  let h3  = new_handle "test.txt" in
  let ro3 = Result.get_ok (open_ro "preloaded" h3) in
  let (s3, ro3) = read_to_string_ro ro3 in
  assert (s3 = "preloaded");
  ignore (close_ro ro3);
  print_endline "open_ro_with_initial: ok";

  (* The following would be COMPILE ERRORS:
       write_all "x" (close_rw rw)   (* closed handle *)
       write_all "x" ro              (* read_only handle *)
  *)

  print_endline "All assertions passed."
