(* 997: Retry with Exponential Backoff *)
(* Pure functional combinator — no concurrency needed *)

(* --- Core retry combinator --- *)

let retry ?(max_attempts=3) ?(base_delay_ms=100) f =
  let rec loop attempt =
    match f () with
    | Ok v -> Ok v
    | Error e when attempt >= max_attempts -> Error e
    | Error _ ->
      let delay = base_delay_ms * (1 lsl (attempt - 1)) in
      Unix.sleepf (float_of_int delay /. 1000.0);
      loop (attempt + 1)
  in
  loop 1

(* --- Approach 1: Retry a flaky operation --- *)

let () =
  let attempt_count = ref 0 in
  let result = retry ~max_attempts:5 ~base_delay_ms:1 (fun () ->
    incr attempt_count;
    if !attempt_count < 3 then Error "not ready yet"
    else Ok !attempt_count
  ) in
  assert (result = Ok 3);
  assert (!attempt_count = 3);
  Printf.printf "Approach 1 (retry): succeeded on attempt %d\n" !attempt_count

(* --- Approach 2: Retry combinator with jitter --- *)

let retry_with_jitter ?(max_attempts=5) ?(base_ms=50) f =
  let rec loop attempt =
    match f () with
    | Ok v -> Ok v
    | Error e when attempt >= max_attempts -> Error e
    | Error _ ->
      let delay = base_ms * (1 lsl (attempt - 1)) in
      let jitter = Random.int (delay / 2 + 1) in
      Unix.sleepf (float_of_int (delay + jitter) /. 1000.0);
      loop (attempt + 1)
  in
  loop 1

let () =
  let calls = ref 0 in
  let result = retry_with_jitter ~max_attempts:4 ~base_ms:1 (fun () ->
    incr calls;
    if !calls < 2 then Error "busy"
    else Ok 42
  ) in
  assert (result = Ok 42);
  Printf.printf "Approach 2 (with jitter): ok after %d calls\n" !calls

(* --- Approach 3: Retry with predicate (not all errors are retryable) --- *)

type error = Transient of string | Permanent of string

let retry_if ~is_retryable ?(max_attempts=3) ?(base_ms=10) f =
  let rec loop attempt =
    match f () with
    | Ok v -> Ok v
    | Error (Permanent _ as e) -> Error e
    | Error e when attempt >= max_attempts -> Error e
    | Error e when not (is_retryable e) -> Error e
    | Error _ ->
      Unix.sleepf (float_of_int (base_ms * (1 lsl (attempt - 1))) /. 1000.0);
      loop (attempt + 1)
  in
  loop 1

let () =
  let calls = ref 0 in
  let result = retry_if ~is_retryable:(function Transient _ -> true | _ -> false)
    ~max_attempts:5 ~base_ms:1 (fun () ->
      incr calls;
      if !calls < 3 then Error (Transient "network error")
      else Ok "connected"
    )
  in
  assert (result = Ok "connected");
  Printf.printf "Approach 3 (selective retry): ok after %d calls\n" !calls

let () = Printf.printf "✓ All tests passed\n"
