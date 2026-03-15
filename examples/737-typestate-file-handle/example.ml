(* 737: File Handle Typestate — OCaml already has this!
   OCaml separates in_channel (read-only) and out_channel (write-only) at the type level.
   This IS the typestate pattern, built into the language. *)

(* Read-only handle — in_channel *)
let demo_read filename =
  let ic = open_in filename in
  (try
    let line = input_line ic in
    Printf.printf "Read: %s\n" line
    (* ic.output_string "cannot write" ← TYPE ERROR: no such method on in_channel *)
  with End_of_file -> ());
  close_in ic

(* Write-only handle — out_channel *)
let demo_write filename =
  let oc = open_out filename in
  output_string oc "Hello from OCaml typestate!\n";
  (* input_line oc ← TYPE ERROR: no such method on out_channel *)
  close_out oc

let () =
  let tmpfile = Filename.temp_file "typestate_" ".txt" in
  demo_write tmpfile;
  demo_read tmpfile;
  Sys.remove tmpfile;
  Printf.printf "Done. OCaml's in_channel/out_channel = typestate pattern!\n"
