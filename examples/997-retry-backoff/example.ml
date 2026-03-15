(* 997: Retry with Exponential Backoff
   Retry a fallible operation with increasing delays between attempts.
   Includes: fixed delay, exponential backoff, jitter, and max-attempts cap.
   OCaml: pure functions + Thread.delay. *)

type retry_config = {
  max_attempts  : int;
  base_delay_ms : float;   (* initial delay in ms *)
  max_delay_ms  : float;   (* cap on delay *)
  multiplier    : float;   (* backoff factor (e.g. 2.0 = double each time) *)
  jitter        : bool;    (* add random jitter to avoid thundering herd *)
}

let default_config = {
  max_attempts  = 5;
  base_delay_ms = 100.0;
  max_delay_ms  = 5000.0;
  multiplier    = 2.0;
  jitter        = true;
}

(* Compute delay for attempt n (0-indexed) *)
let compute_delay cfg n =
  let raw = cfg.base_delay_ms *. (cfg.multiplier ** float_of_int n) in
  let capped = min raw cfg.max_delay_ms in
  if cfg.jitter then
    (* Full jitter: random in [0, capped] — better than half-jitter *)
    Random.float capped
  else
    capped

(* Retry loop: returns Ok on first success, Error on exhaustion *)
let retry cfg f =
  let rec loop attempt last_err =
    if attempt >= cfg.max_attempts then
      Error (Printf.sprintf "failed after %d attempts: %s" cfg.max_attempts
               (Option.value last_err ~default:"unknown"))
    else begin
      match (try Ok (f attempt) with e -> Error (Printexc.to_string e)) with
      | Ok v    -> Ok v
      | Error e ->
        let delay = compute_delay cfg attempt in
        Printf.printf "  attempt %d failed (%s); retry in %.0fms\n%!" (attempt+1) e delay;
        Thread.delay (delay /. 1000.0);
        loop (attempt + 1) (Some e)
    end
  in
  loop 0 None

(* Retry with Result-returning function (no exceptions) *)
let retry_result cfg f =
  let rec loop attempt =
    if attempt >= cfg.max_attempts then
      Error (Printf.sprintf "exhausted %d attempts" cfg.max_attempts)
    else begin
      match f attempt with
      | Ok v    -> Ok v
      | Error e ->
        let delay = compute_delay cfg attempt in
        Printf.printf "  attempt %d: %s; backoff %.0fms\n%!" (attempt+1) e delay;
        Thread.delay (delay /. 1000.0);
        loop (attempt + 1)
    end
  in
  loop 0

(* Retry with predicate: only retry on specific errors *)
let retry_if cfg ~should_retry f =
  let rec loop attempt =
    if attempt >= cfg.max_attempts then Error "max attempts"
    else begin
      match (try Ok (f ()) with e -> Error e) with
      | Ok v    -> Ok v
      | Error e when should_retry e ->
        let delay = compute_delay cfg attempt in
        Printf.printf "  retrying after %.0fms (transient: %s)\n%!" delay (Printexc.to_string e);
        Thread.delay (delay /. 1000.0);
        loop (attempt + 1)
      | Error e -> Error e  (* permanent error — don't retry *)
    end
  in
  loop 0

(* Retry with deadline: stop retrying after a wall-clock deadline *)
let retry_deadline ~deadline_ms cfg f =
  let deadline = Unix.gettimeofday () +. float_of_int deadline_ms /. 1000.0 in
  let rec loop attempt =
    if Unix.gettimeofday () >= deadline then Error "deadline exceeded"
    else if attempt >= cfg.max_attempts then Error "max attempts"
    else begin
      match (try Ok (f ()) with e -> Error (Printexc.to_string e)) with
      | Ok v    -> Ok v
      | Error e ->
        let delay = min (compute_delay cfg attempt)
          ((deadline -. Unix.gettimeofday ()) *. 1000.0) in
        if delay <= 0.0 then Error "deadline exceeded"
        else begin
          Thread.delay (delay /. 1000.0);
          loop (attempt + 1)
        end
    end
  in
  loop 0

let () =
  Random.self_init ();

  Printf.printf "=== Fixed retry (succeed on 3rd attempt) ===\n";
  let call_count = ref 0 in
  let cfg = { default_config with base_delay_ms = 5.0; max_delay_ms = 20.0;
                                  jitter = false; max_attempts = 5 } in
  let result = retry cfg (fun _attempt ->
    incr call_count;
    if !call_count < 3 then failwith "transient error"
    else "success"
  ) in
  Printf.printf "result: %s (calls=%d)\n"
    (match result with Ok v -> v | Error e -> e)
    !call_count;

  Printf.printf "\n=== Exponential backoff with jitter ===\n";
  let delays = Array.init 5 (fun i ->
    compute_delay { default_config with base_delay_ms = 100.0; max_delay_ms = 1000.0 } i
  ) in
  Printf.printf "delay sequence (jitter on): ";
  Array.iter (fun d -> Printf.printf "%.0f " d) delays;
  print_newline ();

  Printf.printf "\n=== Retry with predicate (skip permanent errors) ===\n";
  let n = ref 0 in
  let result2 = retry_if
    { default_config with base_delay_ms = 5.0; max_delay_ms = 10.0; max_attempts = 5 }
    ~should_retry:(function
      | Failure "transient" -> true
      | _ -> false)
    (fun () ->
      incr n;
      if !n < 3 then failwith "transient"
      else "ok after transient")
  in
  Printf.printf "result: %s (attempts=%d)\n"
    (match result2 with Ok v -> v | Error e -> Printexc.to_string e) !n;

  Printf.printf "\n=== Exhausted retries ===\n";
  let always_fail = retry { default_config with max_attempts = 3;
                             base_delay_ms = 1.0; max_delay_ms = 5.0;
                             jitter = false }
    (fun _ -> failwith "always fails")
  in
  Printf.printf "exhausted: %s\n"
    (match always_fail with Ok _ -> "ok" | Error e -> e)
