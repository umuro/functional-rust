(* cfg! concepts in OCaml *)

(* OCaml conditional compilation via ppx or Sys module *)

let () =
  (* Runtime platform detection (not compile-time like cfg!) *)
  let platform = match Sys.os_type with
    | "Unix" -> "Unix/Linux/macOS"
    | "Win32" -> "Windows"
    | "Cygwin" -> "Cygwin"
    | _ -> "Unknown"
  in
  Printf.printf "Platform: %s\n" platform;

  (* Simulate debug mode *)
  let debug = true in (* would be set by build system *)
  if debug then
    Printf.printf "[DEBUG] Debug mode enabled\n";

  Printf.printf "Endianness: %s\n"
    (if Sys.big_endian then "big" else "little")
