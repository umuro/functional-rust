(* 999: Rate Limiter — Token Bucket *)
(* Refill tokens at a fixed rate; consume one per request *)

type token_bucket = {
  mutable tokens: float;
  capacity: float;
  refill_rate: float;  (* tokens per second *)
  mutable last_refill: float;
  m: Mutex.t;
}

let make_bucket ?(capacity=10.0) ?(refill_rate=5.0) () = {
  tokens = capacity;
  capacity;
  refill_rate;
  last_refill = Unix.gettimeofday ();
  m = Mutex.create ();
}

let refill bucket =
  let now = Unix.gettimeofday () in
  let elapsed = now -. bucket.last_refill in
  let new_tokens = elapsed *. bucket.refill_rate in
  bucket.tokens <- Float.min bucket.capacity (bucket.tokens +. new_tokens);
  bucket.last_refill <- now

let try_acquire ?(cost=1.0) bucket =
  Mutex.lock bucket.m;
  refill bucket;
  let allowed = bucket.tokens >= cost in
  if allowed then bucket.tokens <- bucket.tokens -. cost;
  Mutex.unlock bucket.m;
  allowed

let acquire ?(cost=1.0) bucket =
  let rec wait () =
    if try_acquire ~cost bucket then ()
    else (Unix.sleepf 0.001; wait ())
  in
  wait ()

(* --- Approach 1: Burst then throttle --- *)

let () =
  let bucket = make_bucket ~capacity:5.0 ~refill_rate:100.0 () in
  let allowed = ref 0 in
  let denied = ref 0 in

  (* Try 10 requests immediately — only 5 should pass (capacity=5) *)
  for _ = 1 to 10 do
    if try_acquire bucket then incr allowed
    else incr denied
  done;

  assert (!allowed = 5);
  assert (!denied = 5);
  Printf.printf "Approach 1 (burst): %d allowed, %d denied\n" !allowed !denied

(* --- Approach 2: Refill over time --- *)

let () =
  let bucket = make_bucket ~capacity:3.0 ~refill_rate:1000.0 () in
  (* Drain it *)
  for _ = 1 to 3 do assert (try_acquire bucket) done;
  assert (not (try_acquire bucket));  (* empty *)

  (* Wait for refill *)
  Unix.sleepf 0.01;  (* 10ms * 1000 tokens/s = 10 tokens *)

  let allowed = ref 0 in
  for _ = 1 to 5 do
    if try_acquire bucket then incr allowed
  done;
  assert (!allowed >= 3);  (* at least 3 tokens refilled *)
  Printf.printf "Approach 2 (refill): %d tokens refilled\n" !allowed

let () = Printf.printf "✓ All tests passed\n"
