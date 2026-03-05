(* 440. Logging macros – OCaml *)

type level = Debug | Info | Warn | Error

let level_to_int = function Debug->0 | Info->1 | Warn->2 | Error->3
let level_str    = function Debug->"DEBUG" | Info->"INFO" | Warn->"WARN" | Error->"ERROR"

let min_level = ref Info

let log level fmt =
  if level_to_int level >= level_to_int !min_level then
    Printf.ksprintf (fun s ->
      Printf.eprintf "[%s] %s\n%!" (level_str level) s
    ) fmt
  else
    Printf.ksprintf ignore fmt

let () =
  log Info "Starting v%s" "1.0";
  log Debug "This is hidden";  (* below min level *)
  log Warn "Low memory: %d MB" 42;
  log Error "Fatal: %s" "disk full";
  min_level := Debug;
  log Debug "Debug now visible"
