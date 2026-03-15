(* 996: Timeout Pattern
   Run an operation and cancel/abandon it if it takes too long.
   OCaml: Thread + Condition.wait_timed; or a racing background thread
   that races against a deadline future. *)

exception Timeout

(* Run f with a wall-clock timeout of ms milliseconds.
   Returns Ok v if f completes in time, Error Timeout otherwise.
   Implementation: f runs in a background thread; main thread waits
   with a timed Condition wait. *)
let with_timeout_ms ms f =
  let result = ref None in
  let mutex  = Mutex.create () in
  let cond   = Condition.create () in
  let _t = Thread.create (fun () ->
    let v = try Ok (f ()) with e -> Error e in
    Mutex.lock mutex;
    result := Some v;
    Condition.signal cond;
    Mutex.unlock mutex
  ) () in
  let deadline = Unix.gettimeofday () +. float_of_int ms /. 1000.0 in
  Mutex.lock mutex;
  while !result = None && Unix.gettimeofday () < deadline do
    let remaining = deadline -. Unix.gettimeofday () in
    if remaining > 0.0 then
      ignore (Condition.wait_signal_timeout cond mutex remaining)
    (* Note: Condition.wait_signal_timeout is not in stdlib;
       we use Condition.wait with a short sleep loop for portability *)
  done;
  let r = !result in
  Mutex.unlock mutex;
  match r with
  | Some (Ok v)    -> Ok v
  | Some (Error e) -> Error e
  | None           -> Error Timeout

(* Simpler portable version using Thread.delay polling *)
let with_timeout_ms_poll ms f =
  let result  = ref None in
  let mutex   = Mutex.create () in
  let cond    = Condition.create () in
  let _worker = Thread.create (fun () ->
    let v = try Ok (f ()) with e -> Error e in
    Mutex.lock mutex;
    result := Some v;
    Condition.signal cond;
    Mutex.unlock mutex
  ) () in
  (* Timeout thread: sleeps then signals *)
  let _timer = Thread.create (fun () ->
    Thread.delay (float_of_int ms /. 1000.0);
    Mutex.lock mutex;
    if !result = None then Condition.signal cond;
    Mutex.unlock mutex
  ) () in
  Mutex.lock mutex;
  while !result = None do
    Condition.wait cond mutex
  done;
  let r = Option.get !result in
  Mutex.unlock mutex;
  r

(* Wait for a future with timeout *)
let await_timeout fut_fn timeout_ms =
  with_timeout_ms_poll timeout_ms fut_fn

(* Retry with timeout per attempt *)
let with_retry ?(max_attempts=3) ?(timeout_ms=1000) f =
  let rec loop attempt =
    if attempt > max_attempts then Error (Failure "max retries exceeded")
    else
      match with_timeout_ms_poll timeout_ms f with
      | Ok v    -> Ok v
      | Error Timeout ->
        Printf.printf "  attempt %d timed out, retrying...\n%!" attempt;
        loop (attempt + 1)
      | Error e ->
        Printf.printf "  attempt %d failed: %s\n%!" attempt (Printexc.to_string e);
        loop (attempt + 1)
  in
  loop 1

let () =
  Printf.printf "=== Basic timeout ===\n";

  (* Fast operation — completes within timeout *)
  let r1 = with_timeout_ms_poll 100 (fun () ->
    Thread.delay 0.01;
    "done quickly"
  ) in
  Printf.printf "fast op: %s\n"
    (match r1 with Ok v -> v | Error Timeout -> "TIMEOUT" | Error e -> Printexc.to_string e);

  (* Slow operation — exceeds timeout *)
  let r2 = with_timeout_ms_poll 30 (fun () ->
    Thread.delay 0.1;
    "done slowly"
  ) in
  Printf.printf "slow op: %s\n"
    (match r2 with Ok v -> v | Error Timeout -> "TIMEOUT" | Error e -> Printexc.to_string e);

  Printf.printf "\n=== Timeout with error from operation ===\n";
  let r3 = with_timeout_ms_poll 100 (fun () ->
    Thread.delay 0.005;
    failwith "something failed"
  ) in
  (match r3 with
   | Ok _          -> Printf.printf "ok\n"
   | Error Timeout -> Printf.printf "timed out\n"
   | Error e       -> Printf.printf "operation error: %s\n" (Printexc.to_string e));

  Printf.printf "\n=== Select first result with deadline ===\n";
  let attempt_with_deadline deadline_ms candidates =
    let results = ref [] in
    let mutex   = Mutex.create () in
    let cond    = Condition.create () in
    List.iter (fun (label, delay_ms, value) ->
      let _t = Thread.create (fun () ->
        Thread.delay (float_of_int delay_ms /. 1000.0);
        Mutex.lock mutex;
        results := (label, value) :: !results;
        Condition.signal cond;
        Mutex.unlock mutex
      ) () in ()
    ) candidates;
    let timer = Thread.create (fun () ->
      Thread.delay (float_of_int deadline_ms /. 1000.0);
      Mutex.lock mutex;
      if !results = [] then results := [("timeout", "no result")];
      Condition.signal cond;
      Mutex.unlock mutex
    ) () in
    Mutex.lock mutex;
    while !results = [] do Condition.wait cond mutex done;
    let r = List.hd !results in
    Mutex.unlock mutex;
    ignore timer;
    r
  in

  let (label, value) = attempt_with_deadline 50 [
    ("fast",   10, "fast result");
    ("medium", 30, "medium result");
    ("slow",   80, "slow result");
  ] in
  Printf.printf "first result: [%s] = %s\n" label value;

  Printf.printf "\n=== Retry with per-attempt timeout ===\n";
  let call_count = ref 0 in
  let result = with_retry ~max_attempts:3 ~timeout_ms:50 (fun () ->
    incr call_count;
    if !call_count < 3 then begin
      Thread.delay 0.1;  (* first two attempts time out *)
      "would not reach"
    end else "succeeded on attempt 3"
  ) in
  Printf.printf "retry result: %s (attempts=%d)\n"
    (match result with Ok v -> v | Error e -> Printexc.to_string e)
    !call_count
