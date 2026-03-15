(* 998: Circuit Breaker
   Protects a service by "opening" after too many failures and refusing
   further calls until a cooldown period elapses.
   States: Closed (normal) → Open (failing fast) → Half-Open (testing) → Closed.
   OCaml: mutable state machine guarded by a Mutex. *)

type state =
  | Closed                   (* normal operation *)
  | Open of float            (* opened_at timestamp *)
  | Half_open                (* testing: one probe request allowed *)

type circuit_breaker = {
  mutable state           : state;
  mutable failure_count   : int;
  mutable success_count   : int;
  failure_threshold : int;   (* failures to open *)
  success_threshold : int;   (* successes in half-open to close *)
  timeout_s         : float; (* seconds to stay open before half-open *)
  mutex : Mutex.t;
}

exception Circuit_open

let create
    ?(failure_threshold=5)
    ?(success_threshold=2)
    ?(timeout_s=1.0)
    () =
  { state = Closed;
    failure_count = 0;
    success_count = 0;
    failure_threshold;
    success_threshold;
    timeout_s;
    mutex = Mutex.create () }

(* Execute f through the circuit breaker.
   Returns Ok result, Error Circuit_open (fast fail), or Error (f's exception). *)
let call cb f =
  Mutex.lock cb.mutex;
  let state_snapshot = match cb.state with
    | Open t when Unix.gettimeofday () -. t >= cb.timeout_s ->
      cb.state <- Half_open;
      cb.success_count <- 0;
      Half_open
    | s -> s
  in
  Mutex.unlock cb.mutex;

  match state_snapshot with
  | Open _ -> Error Circuit_open

  | Closed | Half_open ->
    (match (try Ok (f ()) with e -> Error e) with
     | Ok v ->
       Mutex.lock cb.mutex;
       (match cb.state with
        | Half_open ->
          cb.success_count <- cb.success_count + 1;
          if cb.success_count >= cb.success_threshold then begin
            cb.state <- Closed;
            cb.failure_count <- 0;
            Printf.printf "  [CB] → CLOSED (recovered)\n%!"
          end
        | Closed ->
          cb.failure_count <- 0   (* reset on success in closed state *)
        | _ -> ());
       Mutex.unlock cb.mutex;
       Ok v

     | Error Circuit_open as e ->
       Mutex.unlock (Mutex.create ());  (* no-op; already unlocked *)
       e

     | Error e ->
       Mutex.lock cb.mutex;
       cb.failure_count <- cb.failure_count + 1;
       (match cb.state with
        | Closed when cb.failure_count >= cb.failure_threshold ->
          cb.state <- Open (Unix.gettimeofday ());
          Printf.printf "  [CB] → OPEN after %d failures\n%!" cb.failure_count
        | Half_open ->
          cb.state <- Open (Unix.gettimeofday ());
          Printf.printf "  [CB] → OPEN again (probe failed)\n%!"
        | _ -> ());
       Mutex.unlock cb.mutex;
       Error e)

let state_name cb =
  Mutex.lock cb.mutex;
  let s = match cb.state with
    | Closed   -> "CLOSED"
    | Open _   -> "OPEN"
    | Half_open -> "HALF-OPEN"
  in
  Mutex.unlock cb.mutex;
  s

let () =
  Printf.printf "=== Circuit breaker demonstration ===\n";
  let cb = create ~failure_threshold:3 ~success_threshold:2 ~timeout_s:0.05 () in

  let call_service will_fail label =
    let r = call cb (fun () ->
      if will_fail then failwith "service unavailable"
      else Printf.sprintf "%s: ok" label
    ) in
    match r with
    | Ok msg       -> Printf.printf "  %s → %s [%s]\n" label msg (state_name cb)
    | Error Circuit_open ->
      Printf.printf "  %s → FAST FAIL (circuit open) [%s]\n" label (state_name cb)
    | Error e      ->
      Printf.printf "  %s → ERROR: %s [%s]\n" label (Printexc.to_string e) (state_name cb)
  in

  (* Phase 1: trigger failures to open the circuit *)
  Printf.printf "\n-- Phase 1: triggering failures --\n";
  for i = 1 to 5 do
    call_service true (Printf.sprintf "call-%d" i)
  done;

  (* Phase 2: circuit is open — fast fail *)
  Printf.printf "\n-- Phase 2: circuit open --\n";
  for i = 6 to 8 do
    call_service false (Printf.sprintf "call-%d" i)
  done;

  (* Phase 3: wait for timeout, half-open state, probe *)
  Printf.printf "\n-- Phase 3: wait for timeout → half-open --\n";
  Thread.delay 0.06;
  call_service false "probe-1";  (* success: half-open *)
  call_service false "probe-2";  (* success: closes circuit *)

  (* Phase 4: circuit closed again *)
  Printf.printf "\n-- Phase 4: circuit recovered --\n";
  for i = 1 to 3 do
    call_service false (Printf.sprintf "healthy-call-%d" i)
  done;

  Printf.printf "\n=== Statistics ===\n";
  Printf.printf "failure_count = %d  success_count = %d  state = %s\n"
    cb.failure_count cb.success_count (state_name cb)
