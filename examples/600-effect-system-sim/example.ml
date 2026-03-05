(* Algebraic effects in OCaml 5 *)
(* Simulated here with a simple handler model *)

(* Effect as a type *)
type 'a request =
  | Log    of string
  | Random of (int -> 'a)
  | State  of { get: unit -> int; set: int -> unit }

(* Simple simulation *)
let handle_effects prog =
  let state = ref 0 in
  let log_buf = Buffer.create 64 in
  let get () = !state in
  let set v  = state := v in
  let log s  = Buffer.add_string log_buf (s^"\n") in
  prog get set log;
  (Buffer.contents log_buf, !state)

let () =
  let (logs, final_state) = handle_effects (fun get set log ->
    log "Starting";
    set (get () + 10);
    log (Printf.sprintf "State = %d" (get ()));
    set (get () * 2);
    log (Printf.sprintf "Final state = %d" (get ()))
  ) in
  print_string logs;
  Printf.printf "state=%d\n" final_state
