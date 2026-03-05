(* 752: Test Doubles Taxonomy — OCaml *)

(* The interface/dependency *)
module type LOGGER = sig
  val log   : string -> unit
  val error : string -> unit
end

(* ── Stub: returns canned values, no verification ─────────────────── *)
module NullLogger : LOGGER = struct
  let log   _ = ()
  let error _ = ()
end

(* ── Fake: working but simplified (in-memory) ─────────────────────── *)
module InMemoryLogger : sig
  include LOGGER
  val get_logs   : unit -> string list
  val get_errors : unit -> string list
end = struct
  let logs   = ref []
  let errors = ref []
  let log   s = logs   := s :: !logs
  let error s = errors := s :: !errors
  let get_logs   () = List.rev !logs
  let get_errors () = List.rev !errors
end

(* ── Mock: records interactions for assertion ──────────────────────── *)
module MockLogger : sig
  include LOGGER
  val call_count : unit -> int
  val reset : unit -> unit
end = struct
  let calls = ref 0
  let log   _ = incr calls
  let error _ = incr calls
  let call_count () = !calls
  let reset () = calls := 0
end

(* Business logic *)
module App(L : LOGGER) = struct
  let process items =
    List.iter (fun item ->
      if item < 0
      then L.error (Printf.sprintf "negative item: %d" item)
      else L.log (Printf.sprintf "processing: %d" item)
    ) items
end

let () =
  (* Using Stub: silent, no verification *)
  let module A = App(NullLogger) in
  A.process [1; -2; 3];
  Printf.printf "Stub: processed silently\n";

  (* Using Fake: readable log *)
  let module B = App(InMemoryLogger) in
  B.process [1; -2; 3];
  Printf.printf "Fake logs: [%s]\n"
    (String.concat "; " (InMemoryLogger.get_logs ()));
  Printf.printf "Fake errors: [%s]\n"
    (String.concat "; " (InMemoryLogger.get_errors ()));

  (* Using Mock: count calls *)
  MockLogger.reset ();
  let module C = App(MockLogger) in
  C.process [1; 2; 3; -1; -2];
  Printf.printf "Mock: %d call(s) made\n" (MockLogger.call_count ())
