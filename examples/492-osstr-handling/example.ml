(* 492. OsStr – OCaml note *)
(* OCaml uses UTF-8 strings on Unix; file system may have non-UTF-8 names *)
(* Demonstrate with Sys module *)
let () =
  Printf.printf "os: %s\n" Sys.os_type;
  Printf.printf "cwd: %s\n" (Sys.getcwd ());
  (match Sys.getenv_opt "HOME" with
   | Some h -> Printf.printf "HOME=%s\n" h
   | None -> print_string "HOME not set\n");
  (* Filename operations *)
  let f = "/tmp/test.txt" in
  Printf.printf "basename: %s\n" (Filename.basename f);
  Printf.printf "dirname:  %s\n" (Filename.dirname f);
  Printf.printf "check_suffix .txt: %b\n" (Filename.check_suffix f ".txt")
