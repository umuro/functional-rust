(* 998: Circuit Breaker *)
(* Open/Half-Open/Closed state machine for fault tolerance *)

type state = Closed | Open of float | HalfOpen

type 'a circuit_breaker = {
  mutable state: state;
  mutable failures: int;
  failure_threshold: int;
  recovery_timeout_s: float;
  m: Mutex.t;
}

let make_breaker ?(failure_threshold=3) ?(recovery_timeout_s=1.0) () = {
  state = Closed;
  failures = 0;
  failure_threshold;
  recovery_timeout_s;
  m = Mutex.create ();
}

type 'a breaker_result = BrResult of 'a | CircuitOpen | CallError of string

let call breaker f =
  Mutex.lock breaker.m;

  (* Transition from Open to HalfOpen if timeout elapsed *)
  (match breaker.state with
  | Open since when Unix.gettimeofday () -. since >= breaker.recovery_timeout_s ->
    breaker.state <- HalfOpen
  | _ -> ());

  let state = breaker.state in
  Mutex.unlock breaker.m;

  match state with
  | Open _ -> CircuitOpen
  | Closed | HalfOpen ->
    (match (try Ok (f ()) with e -> Error (Printexc.to_string e)) with
    | Ok v ->
      Mutex.lock breaker.m;
      breaker.failures <- 0;
      breaker.state <- Closed;
      Mutex.unlock breaker.m;
      BrResult v
    | Error e ->
      Mutex.lock breaker.m;
      breaker.failures <- breaker.failures + 1;
      if breaker.failures >= breaker.failure_threshold then
        breaker.state <- Open (Unix.gettimeofday ());
      Mutex.unlock breaker.m;
      CallError e)

let state_name b = match b.state with
  | Closed -> "Closed"
  | Open _ -> "Open"
  | HalfOpen -> "HalfOpen"

(* --- Approach 1: Fail 3 times → Open, then recover --- *)

let () =
  let b = make_breaker ~failure_threshold:3 ~recovery_timeout_s:0.05 () in
  assert (state_name b = "Closed");

  (* Fail 3 times *)
  for _ = 1 to 3 do
    let _ = call b (fun () -> failwith "simulated error") in ()
  done;
  assert (state_name b = "Open");
  Printf.printf "Approach 1: after 3 failures: %s\n" (state_name b);

  (* Circuit is open — calls rejected *)
  (match call b (fun () -> 42) with
  | CircuitOpen -> Printf.printf "Approach 1: call rejected (circuit open)\n"
  | _ -> assert false);

  (* Wait for recovery timeout *)
  Unix.sleepf 0.06;
  (* Next call should go through (HalfOpen) *)
  (match call b (fun () -> 99) with
  | BrResult v ->
    assert (v = 99);
    assert (state_name b = "Closed");
    Printf.printf "Approach 1: recovered, got %d, state: %s\n" v (state_name b)
  | _ -> assert false)

let () = Printf.printf "✓ All tests passed\n"
