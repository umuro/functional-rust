(* Declarative macros in OCaml via ppx and module-level patterns *)
(* No direct macro_rules! equivalent; we show patterns with functions *)

(* Pattern: log with file/line info *)
let log level msg =
  Printf.printf "[%s] %s\n" level msg

(* Pattern: assert with message *)
let assert_eq_msg a b msg =
  if a <> b then
    failwith (Printf.sprintf "Assertion failed: %s (got %d, expected %d)" msg a b)

(* Pattern: repeat *)
let repeat n f =
  for _ = 1 to n do f () done

(* Pattern: min/max macros *)
let min_of a b = if a < b then a else b
let max_of a b = if a > b then a else b

let () =
  log "INFO" "Starting application";
  assert_eq_msg (2 + 2) 4 "arithmetic";
  repeat 3 (fun () -> Printf.printf "Hello!\n");
  Printf.printf "min(3,7)=%d max(3,7)=%d\n" (min_of 3 7) (max_of 3 7)
