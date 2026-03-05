(* Build-time values in OCaml via dune substitution *)

(* In dune, you'd use (subst) and write version strings *)
(* Here we simulate with constants *)

let version = "1.0.0"  (* would come from %{version} in dune *)
let pkg_name = "my-app"
let build_date = "2026-01-01"  (* would come from build system *)

let () =
  Printf.printf "%s v%s\n" pkg_name version;
  Printf.printf "Built: %s\n" build_date;

  (* Runtime env var access *)
  let home = try Sys.getenv "HOME" with Not_found -> "/unknown" in
  Printf.printf "HOME: %s\n" home;

  let debug = match Sys.getenv_opt "DEBUG" with
    | Some "1" | Some "true" -> true
    | _ -> false
  in
  Printf.printf "Debug mode: %b\n" debug
