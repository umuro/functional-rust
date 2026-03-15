(* 999: Rate Limiter
   Limits the rate at which operations can be performed.
   Three classic algorithms:
     1. Token bucket: smooth bursts, allows short bursts up to capacity
     2. Leaky bucket: strict constant output rate
     3. Fixed window counter: simple, but has boundary-spike problem
   OCaml: mutable state + Mutex + optional Thread.delay. *)

(* --- Token Bucket --- *)
(* Tokens refill at `rate` per second; max `capacity` tokens.
   Each request consumes one token; blocks if none available. *)
type token_bucket = {
  mutable tokens   : float;
  capacity         : float;
  refill_rate      : float;   (* tokens per second *)
  mutable last_refill : float;
  mutex : Mutex.t;
}

let make_token_bucket ~capacity ~rate =
  { tokens = float_of_int capacity; capacity = float_of_int capacity;
    refill_rate = rate;
    last_refill = Unix.gettimeofday ();
    mutex = Mutex.create () }

(* Refill tokens based on elapsed time *)
let refill tb =
  let now = Unix.gettimeofday () in
  let elapsed = now -. tb.last_refill in
  tb.tokens <- min tb.capacity (tb.tokens +. elapsed *. tb.refill_rate);
  tb.last_refill <- now

(* Try to acquire n tokens; returns true if successful *)
let try_acquire_tokens tb ?(n=1.0) () =
  Mutex.lock tb.mutex;
  refill tb;
  let ok = tb.tokens >= n in
  if ok then tb.tokens <- tb.tokens -. n;
  Mutex.unlock tb.mutex;
  ok

(* Acquire and wait until a token is available *)
let acquire_token tb =
  let acquired = ref false in
  while not !acquired do
    Mutex.lock tb.mutex;
    refill tb;
    if tb.tokens >= 1.0 then begin
      tb.tokens <- tb.tokens -. 1.0;
      acquired := true
    end;
    let wait_s = if !acquired then 0.0
                 else (1.0 -. tb.tokens) /. tb.refill_rate in
    Mutex.unlock tb.mutex;
    if not !acquired then Thread.delay wait_s
  done

(* --- Fixed Window Counter --- *)
type window_counter = {
  mutable count : int;
  mutable window_start : float;
  window_s   : float;   (* window duration in seconds *)
  max_count  : int;
  mutex : Mutex.t;
}

let make_window_counter ~window_s ~max_count =
  { count = 0; window_start = Unix.gettimeofday ();
    window_s; max_count; mutex = Mutex.create () }

let window_allow wc =
  Mutex.lock wc.mutex;
  let now = Unix.gettimeofday () in
  if now -. wc.window_start >= wc.window_s then begin
    wc.count <- 0;
    wc.window_start <- now
  end;
  let ok = wc.count < wc.max_count in
  if ok then wc.count <- wc.count + 1;
  Mutex.unlock wc.mutex;
  ok

(* --- Sliding Window Log --- *)
(* Exact sliding window: keep timestamps of recent requests *)
type sliding_log = {
  mutable timestamps : float list;
  window_s  : float;
  max_count : int;
  mutex : Mutex.t;
}

let make_sliding_log ~window_s ~max_count =
  { timestamps = []; window_s; max_count; mutex = Mutex.create () }

let sliding_allow sl =
  Mutex.lock sl.mutex;
  let now = Unix.gettimeofday () in
  let cutoff = now -. sl.window_s in
  sl.timestamps <- List.filter (fun t -> t >= cutoff) sl.timestamps;
  let ok = List.length sl.timestamps < sl.max_count in
  if ok then sl.timestamps <- now :: sl.timestamps;
  Mutex.unlock sl.mutex;
  ok

let () =
  Printf.printf "=== Token Bucket (capacity=5, rate=10/s) ===\n";
  let tb = make_token_bucket ~capacity:5 ~rate:10.0 in

  (* Burst: consume all 5 tokens immediately *)
  let burst = ref 0 in
  while try_acquire_tokens tb () do incr burst done;
  Printf.printf "initial burst: %d tokens (capacity=5)\n" !burst;

  (* Wait for refill *)
  Thread.delay 0.2;  (* 0.2s × 10/s = 2 new tokens *)
  let refilled = ref 0 in
  while try_acquire_tokens tb () do incr refilled done;
  Printf.printf "after 200ms refill: %d new tokens (~2)\n" !refilled;

  Printf.printf "\n=== Token bucket rate-limited loop ===\n";
  let tb2 = make_token_bucket ~capacity:3 ~rate:20.0 in
  let start = Unix.gettimeofday () in
  for i = 1 to 6 do
    acquire_token tb2;
    Printf.printf "  request %d at %.0fms\n%!" i
      ((Unix.gettimeofday () -. start) *. 1000.0)
  done;

  Printf.printf "\n=== Fixed Window Counter (5 req/100ms) ===\n";
  let wc = make_window_counter ~window_s:0.1 ~max_count:5 in
  let allowed = ref 0 and denied = ref 0 in
  for _ = 1 to 8 do
    if window_allow wc then incr allowed else incr denied
  done;
  Printf.printf "allowed=%d denied=%d (first 5 pass)\n" !allowed !denied;

  (* New window *)
  Thread.delay 0.11;
  let a2 = ref 0 in
  for _ = 1 to 3 do if window_allow wc then incr a2 done;
  Printf.printf "new window: %d allowed\n" !a2;

  Printf.printf "\n=== Sliding Window Log (5 req/100ms) ===\n";
  let sl = make_sliding_log ~window_s:0.1 ~max_count:5 in
  let a3 = ref 0 and d3 = ref 0 in
  for _ = 1 to 7 do
    if sliding_allow sl then incr a3 else incr d3
  done;
  Printf.printf "allowed=%d denied=%d\n" !a3 !d3;

  Thread.delay 0.05;
  Printf.printf "after 50ms (some slots free): %b\n" (sliding_allow sl);

  Printf.printf "\n=== Concurrent rate limiting (10 threads, 5/s limit) ===\n";
  let tb3 = make_token_bucket ~capacity:5 ~rate:50.0 in
  let results = ref [] in
  let rmutex = Mutex.create () in
  let threads = List.init 10 (fun i ->
    Thread.create (fun () ->
      let t = Unix.gettimeofday () in
      acquire_token tb3;
      Mutex.lock rmutex;
      results := (i, Unix.gettimeofday () -. t) :: !results;
      Mutex.unlock rmutex
    ) ()
  ) in
  List.iter Thread.join threads;
  let sorted = List.sort (fun (a,_) (b,_) -> compare a b) !results in
  List.iter (fun (i, wait) ->
    Printf.printf "  thread %d waited %.0fms\n" i (wait *. 1000.0)
  ) sorted
