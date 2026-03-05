(* Drop and RAII concepts in OCaml *)

(* OCaml has finalizers, but RAII is done via explicit cleanup or with_* functions *)

type file_handle = {
  name: string;
  mutable closed: bool;
}

let open_file name =
  Printf.printf "Opening file: %s\n" name;
  { name; closed = false }

let close_file fh =
  if not fh.closed then begin
    Printf.printf "Closing file: %s\n" fh.name;
    fh.closed <- true
  end

(* RAII pattern via with_file *)
let with_file name f =
  let fh = open_file name in
  Fun.protect ~finally:(fun () -> close_file fh) (fun () -> f fh)

let () =
  (* Explicit cleanup *)
  let f = open_file "data.txt" in
  Printf.printf "Using file: %s\n" f.name;
  close_file f;

  (* RAII via with_file *)
  with_file "config.json" (fun f ->
    Printf.printf "Processing: %s\n" f.name
  )
  (* file automatically closed even if exception *)
